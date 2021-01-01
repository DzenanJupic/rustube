use std::sync::Arc;

use reqwest::Client;
use url::Url;

use cipher::Cipher;

use crate::{Stream, Video, VideoDetails, VideoInfo};
use crate::error::Error;
use crate::video_info::player_response::streaming_data::RawFormat;
use crate::video_info::player_response::streaming_data::StreamingData;

mod cipher;

/// A YouTubeDescrambler, used to decrypt the data fetched by [`YouTubeFetcher`].
///
/// You will probably rarely use this type directly, and use [`YouTube`] instead. 
/// There's no public way of directly constructing a [`YouTubeDescrambler`]. The only way of getting
/// one is by calling [`YouTubeFetcher::fetch`].
///
/// # Example
/// ```no_run
///# use rustube::{VideoFetcher, Id, VideoDescrambler};
///# use url::Url;
/// const URL: &str = "https://youtube.com/watch?iv=5jlI4uzZGjU";
/// let url = Url::parse(URL).unwrap();
/// 
///# tokio_test::block_on(async {
/// let fetcher: VideoFetcher =  VideoFetcher::from_url(&url).unwrap();
/// let descrambler: VideoDescrambler = fetcher.fetch().await.unwrap();
///# }); 
/// ``` 
#[derive(Clone, derive_more::Display, derivative::Derivative)]
#[display(fmt = "VideoDescrambler({})", "video_info.player_response.video_details.video_id")]
#[derivative(Debug, PartialEq, Eq)]
pub struct VideoDescrambler {
    pub(crate) video_info: VideoInfo,
    #[derivative(Debug = "ignore", PartialEq = "ignore")]
    pub(crate) client: Client,
    pub(crate) js: String,
}

impl VideoDescrambler {
    /// Descrambles the data fetched by YouTubeFetcher.
    /// For more information have a look at the [`YouTube`] documentation.
    ///
    /// # Errors
    /// This method will fail, when it's not able do extract all necessary information out of the
    /// 
    /// # How it works
    /// Descrambling, in this case, mainly refers to descrambling the 
    /// [`player_response::streaming_data::SignatureCipher`]. YouTube does, unfortunately, not 
    /// directly provide any urls for their videos. In fact, as long as nobody requests a video, 
    /// the video has no url, and therefore cannot be accessed or downloaded via the internet.
    /// So then, how can we download it?
    /// 
    /// The first step, requesting the video, already happened in [`YouTubeFetcher::fetch`] (To fully
    /// understand `descramble`, you should first read how fetching works).
    /// The next step is to descramble (or decrypt) the signature, stored in `CipherSignature.s`,
    /// of each individual `RawFormat`. To to so, there's the so called `transform_plan`, and the 
    /// so called `transform_map`. 
    /// 
    /// The `transform_plan` is just a list of javascript function, which take a string (or an array) 
    /// plus an optional integer as input, transform the string in a certain way, and return the new
    /// string. This new string then represents the new signature.
    /// 
    /// But wait! How can we run JavaScript in Rust? And doesn't that come with a huge overhead?
    /// It actually would come with a huge overhead! That's why we need the `transform_map`. The 
    /// `transform_map` is a `HashMap<String, TransformFn>`, which maps JavaScript function names to
    /// Rust functions. For now, you can think of `TransformFn` just being a function pointer. 
    /// There's actually more to it, but that's not important for downloading the video.
    ///
    /// To finally get the videos signature, the raw, initial signature has to be transformed once 
    /// by every function in the `transform_plan`. To do so, we just iterate over it, look up the
    /// corresponding `TransformFn` in `transform_map`, and pass the signature as well as the 
    /// optional integer. to it.
    /// 
    /// The last step `descramble` performs, is to take all `RawFormat`s, which now contain the 
    /// correct signature, and convert them to `Stream`s. At the end of the day, `Stream`s are just
    /// `RawFormat`s with some extra information.
    pub fn descramble(mut self) -> crate::Result<Video> {
        let streaming_data = self.video_info.player_response.streaming_data
            .as_mut()
            .ok_or(Error::Custom(
                "VideoInfo contained no StreamingData, which is essential for downloading.".into()
            ))?;

        if let Some(ref adaptive_fmts_raw) = self.video_info.adaptive_fmts_raw {
            apply_descrambler_adaptive_fmts(streaming_data, adaptive_fmts_raw)?;
        }

        let mut streams = Vec::new();
        apply_signature(streaming_data, &self.js)?;
        Self::initialize_streams(
            streaming_data,
            &mut streams,
            &self.client,
            &self.video_info.player_response.video_details,
        )?;

        Ok(Video {
            video_info: self.video_info,
            streams,
        })
    }

    #[inline]
    pub fn video_info(&self) -> &VideoInfo {
        &self.video_info
    }

    #[inline]
    fn initialize_streams(
        streaming_data: &mut StreamingData,
        streams: &mut Vec<Stream>,
        client: &Client,
        video_details: &Arc<VideoDetails>,
    ) -> crate::Result<()> {
        for raw_format in streaming_data.formats.drain(..).chain(streaming_data.adaptive_formats.drain(..)) {
            let stream = Stream::from_raw_format(
                raw_format,
                client.clone(),
                Arc::clone(video_details),
            )?;
            streams.push(stream);
        }

        Ok(())
    }
}

#[inline]
fn apply_descrambler_adaptive_fmts(streaming_data: &mut StreamingData, adaptive_fmts_raw: &str) -> crate::Result<()> {
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

#[inline]
fn apply_signature(streaming_data: &mut StreamingData, js: &str) -> crate::Result<()> {
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
