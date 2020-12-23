use std::lazy::SyncLazy;

use regex::Regex;
use serde::Deserialize;
use url::Url;

use crate::{Id, Result};
use crate::cipher::Cipher;
use crate::error::Error;
use crate::parser;
use crate::player_response::playability_status::PlayabilityStatus;
use crate::player_response::PlayerResponse;
use crate::player_response::streaming_data::{RawFormat, StreamingData};

#[inline]
pub(crate) fn is_age_restricted(watch_html: &str) -> bool {
    static PATTERN: SyncLazy<Regex> = SyncLazy::new(|| Regex::new("og:restrictions:age").unwrap());
    PATTERN.is_match(watch_html)
}

// todo: this is also a field in PlayerResponse
pub(crate) fn playability_status(watch_html: &str) -> Result<Option<PlayabilityStatus>> {
    match initial_player_response(watch_html) {
        Some(initial_player_response) => {
            // todo: why do I have to deserialize it again?
            let player_response = serde_json::from_str::<PlayerResponse>(initial_player_response)?;
            Ok(Some(player_response.playability_status))
        }
        None => Ok(None)
    }
}

fn initial_player_response(watch_html: &str) -> Option<&str> {
    static PATTERN: SyncLazy<Regex> = SyncLazy::new(||
        Regex::new(r#"window\[['"]ytInitialPlayerResponse['"]]\s*=\s*(\{[^\n]+});"#).unwrap()
    );

    match PATTERN.captures(watch_html) {
        Some(c) => Some(c.get(1).unwrap().as_str()),
        None => None
    }
}

pub(crate) fn video_info_url(video_id: Id, watch_url: &Url) -> Url {
    let params: &[(&str, &str)] = &[
        ("video_id", video_id.as_str()),
        ("ps", "default"),
        ("eurl", watch_url.as_str()),
        ("hl", "en_US")
    ];
    _video_info_url(params)
}

pub(crate) fn video_info_url_age_restricted(video_id: Id, watch_url: &Url) -> Url {
    static PATTERN: SyncLazy<Regex> = SyncLazy::new(|| Regex::new(r#""sts"\s*:\s*(\d+)"#).unwrap());

    let sts = match PATTERN.captures(watch_url.as_str()) {
        Some(c) => c.get(1).unwrap().as_str(),
        None => ""
    };

    let eurl = format!("https://youtube.googleapis.com/v/{}", video_id.as_str());
    let params: &[(&str, &str)] = &[
        ("video_id", video_id.as_str()),
        ("eurl", &eurl),
        ("sts", sts)
    ];
    _video_info_url(params)
}


#[inline]
fn _video_info_url(params: &[(&str, &str)]) -> Url {
    Url::parse_with_params(
        "https://youtube.com/get_video_info?",
        params,
    ).unwrap()
}

pub(crate) fn js_url_player_response(html: &str) -> Result<(Url, PlayerResponse)> {
    let player_response = get_ytplayer_config(html)?;
    let base_js = match player_response.assets {
        Some(ref assets) => assets.js.as_str(),
        None => get_ytplayer_js(html)?
    };


    Ok(
        Url::parse(&format!("https://youtube.com{}", base_js))
            .map(|url| (url, player_response))?
    )
}

/// Get the YouTube player base JavaScript path.
/// 
/// :param str html
///     The html contents of the watch page.
/// :rtype: str
/// :returns:
///     Path to YouTube's base.js file.
fn get_ytplayer_js(html: &str) -> Result<&str> {
    static JS_URL_PATTERNS: SyncLazy<Regex> = SyncLazy::new(||
        Regex::new(r"(/s/player/[\w\d]+/[\w\d_/.]+/base\.js)").unwrap()
    );

    match JS_URL_PATTERNS.captures(html) {
        Some(function_match) => Ok(function_match.get(1).unwrap().as_str()),
        None => Err(Error::UnexpectedResponse(format!(
            "could not extract the ytplayer-javascript from: {}",
            html
        ).into()))
    }
}

/// Get the YouTube player configuration data from the watch html.
/// 
/// Extract the ``ytplayer_config``, which is json data embedded within the
/// watch html and serves as the primary source of obtaining the stream
/// manifest data.
/// 
/// :param str html:
///     The html contents of the watch page.
/// :rtype: str
/// :returns:
///     Substring of the html containing the encoded manifest data.
#[inline]
pub(crate) fn get_ytplayer_config(html: &str) -> Result<PlayerResponse> {
    static CONFIG_PATTERNS: SyncLazy<[Regex; 3]> = SyncLazy::new(|| [
        Regex::new(r"ytplayer\.config\s*=\s*").unwrap(),
        Regex::new(r"ytInitialPlayerResponse\s*=\s*").unwrap(),
        // fixme
        // pytube handles `setConfig` little bit differently. It parses the entire argument 
        // to `setConfig()` and then uses load json to find `PlayerResponse` inside of it.
        // We currently handle both the same way, and just deserialize into the `PlayerConfig` enum.
        // This *should* have the same effect.
        //
        // In the future, it may be a good idea, to also handle both cases differently, so we don't
        // loose performance on deserializing into an enum, but deserialize `CONFIG_PATTERNS` directly 
        // into `PlayerResponse`, and `SET_CONFIG_PATTERNS` into `Args`. The problem currently is, that
        // I don't know, if CONFIG_PATTERNS can also contain `Args`.
        Regex::new(r#"yt\.setConfig\(.*['"]PLAYER_CONFIG['"]:\s*"#).unwrap()
    ]);

    CONFIG_PATTERNS
        .iter()
        .find_map(|pattern| {
            let json = parser::parse_for_object(html, pattern).ok()?;
            deserialize_ytplayer_config(json).ok()
        })
        .ok_or_else(|| Error::UnexpectedResponse(
            "Could not find ytplayer_config in the watch html.".into()
        ))
}

#[inline]
fn deserialize_ytplayer_config(json: &str) -> Result<PlayerResponse> {
    #[derive(Deserialize)]
    struct Args { player_response: PlayerResponse }
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum PlayerConfig { Args { args: Args }, Response(PlayerResponse) }

    Ok(
        match serde_json::from_str::<PlayerConfig>(json)? {
            PlayerConfig::Args { args } => args.player_response,
            PlayerConfig::Response(pr) => pr
        }
    )
}

#[inline]
pub fn apply_descrambler_adaptive_fmts(streaming_data: &mut StreamingData, adaptive_fmts_raw: &str) -> Result<()> {
    for raw_fmt in adaptive_fmts_raw.split(',') {
        // fixme: this implementation is likely wrong. 
        // main question: is adaptive_fmts_raw a list of normal RawFormats?
        // To make is correct, I would need sample data for adaptive_fmts_raw
        log::warn!(
            "`apply_descrambler_adaptive_fmts` is probaply broken!\
             Please open an issue on GitHub and paste in the whole warning message (it may be quite long).\
             adaptive_fmts_raw: `{}`", raw_fmt
        );
        let raw_format = serde_qs::from_str::<RawFormat>(raw_fmt)?;
        streaming_data.formats.push(raw_format);
    }

    Ok(())
}

pub(crate) fn apply_signature(streaming_data: &mut StreamingData, js: &str) -> Result<()> {
    let cipher = Cipher::from_js(js)?;

    for raw_format in streaming_data.formats.iter_mut().chain(streaming_data.adaptive_formats.iter_mut()) {
        let url = &mut raw_format.signature_cipher.url;
        let s = match raw_format.signature_cipher.s {
            Some(ref mut s) => s,
            None if url_already_contains_signature(url) => continue,
            None => return Err(Error::UnexpectedResponse(
                "RawFormat did not contain a signature (s), nor did the url".into()
            ))
        };

        cipher.decrypt_signature(s)?;
        url
            .query_pairs_mut()
            .append_pair("sig", s);
    }

    Ok(())
}

#[inline]
fn url_already_contains_signature(url: &Url) -> bool {
    let url = url.as_str();
    url.contains("signature") || (url.contains("&sig=") || url.contains("&lsig="))
}
