use std::lazy::SyncLazy;

use regex::Regex;
use serde::Deserialize;
use url::Url;

use crate::{Id, Result, StreamMap};
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


    Url::parse(&format!("https://youtube.com{}", base_js))
        .map(|url| (url, player_response))
        .map_err(|_| Error::Other)
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
pub(crate) fn get_ytplayer_config(html: &str) -> Result<PlayerResponse> {
    static CONFIG_PATTERNS: SyncLazy<[Regex; 2]> = SyncLazy::new(|| [
        Regex::new(r"ytplayer\.config\s*=\s*").unwrap(),
        Regex::new(r"ytInitialPlayerResponse\s*=\s*").unwrap(),
    ]);

    let player_response = CONFIG_PATTERNS
        .iter()
        .find_map(|pattern| {
            let json = parser::parse_for_object(html, pattern).ok()?;

            if json.contains("args") {
                #[derive(Deserialize)]
                struct ArgsWrapper { args: Args }
                #[derive(Deserialize)]
                struct Args { player_response: PlayerResponse }

                Some(
                    serde_json::from_str::<ArgsWrapper>(json).ok()?
                        .args
                        .player_response
                )
            } else {
                serde_json::from_str::<PlayerResponse>(json).ok()
            }
        });

    if let Some(player_response) = player_response {
        return Ok(player_response);
    }

    // todo
    // // setConfig() needs to be handled a little differently.
    // // We want to parse the entire argument to setConfig()
    // // and use then load that as json to find PLAYER_CONFIG
    // // inside of it.
    // static SET_CONFIG_PATTERNS: SyncLazy<Regex> = SyncLazy::new(
    //     || Regex::new(r#"yt\.setConfig\(.*['"]PLAYER_CONFIG['"]:\s*"#).unwrap()
    // );
    // 
    // match parser::parse_for_object(html, &SET_CONFIG_PATTERNS) {
    //     Ok(player_config) => Ok(player_config),
    //     Err(e) => Err(e)
    // }

    Err(Error::Other)
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
        None => Err(Error::UnexpectedResponse)
    }
}

#[inline]
pub(crate) fn apply_descrambler(
    streaming_data: &mut StreamingData,
    adaptive_fmts_raw: Option<&String>,
    key: StreamMap,
) -> Result<()> {
    match key {
        StreamMap::UrlEncodedFmtStream => apply_descrambler_url_encoded_fmt_stream(streaming_data),
        StreamMap::AdaptiveFmts => apply_descrambler_adaptive_fmts(
            streaming_data,
            adaptive_fmts_raw?,
        )
    }
}

fn apply_descrambler_url_encoded_fmt_stream(streaming_data: &mut StreamingData) -> Result<()> {
    streaming_data
        .formats
        .iter_mut()
        .chain(
            streaming_data.adaptive_formats.iter_mut()
        )
        .for_each(|format| {
            if let Some(url) = format.url.take() {
                format.signature_cipher.url = url;
            }
        });

    Ok(())
}

fn apply_descrambler_adaptive_fmts(streaming_data: &mut StreamingData, adaptive_fmts_raw: &str) -> Result<()> {
    for raw_fmt in adaptive_fmts_raw.split(',') {
        // fixme: this implementation is likely wrong.
        // To make is correct, I would need sample data for adaptive_fmts_raw
        log::warn!(
            "`apply_descrambler_adaptive_fmts` is probaply broken!\
             Please open an issue on GitHub and paste in the warning message.\
             adaptive_fmts_raw: `{}`", adaptive_fmts_raw
        );
        let raw_format = serde_qs::from_str::<RawFormat>(raw_fmt)?;
        streaming_data.adaptive_formats.push(raw_format);
    }

    Ok(())
}

pub(crate) fn apply_signature(streaming_data: &mut StreamingData, fmt: StreamMap, js: &str) -> Result<()> {
    let raw_formats = match fmt {
        StreamMap::UrlEncodedFmtStream => &mut streaming_data.formats,
        StreamMap::AdaptiveFmts => &mut streaming_data.adaptive_formats,
    };
    let cipher = Cipher::from_js(js)?;

    for raw_format in raw_formats {
        let url = &mut raw_format.signature_cipher.url;
        let s = match raw_format.signature_cipher.s {
            Some(ref mut s) => s,
            None if url_already_contains_signature(url) => continue,
            None => return Err(Error::Other)
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
    url.contains("signature") || (url.contains("&sig=") || url.contains("&lsig"))
}
