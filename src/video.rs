use std::sync::Arc;

use derive_more::Display;

use crate::{Id, Stream, VideoInfo};
use crate::video_info::player_response::video_details::VideoDetails;

/// A YouTube downloader, which allows you to download all available formats and qualities of a 
/// YouTube video. Each instance of [`Video`] represents an existing, available, and downloadable 
/// video.
///
/// This type is the easiest way to download a video. There are tow major ways of constructing an
/// instance of `Video`:
/// 1. By using the asynchronous `Video::from_*` methods. These methods will take some kind of 
/// video-identifier, like an `Url` or an `Id`, will then internally download the necessary video 
/// information and finally descramble it.
/// 2. By calling [`VideoDescrambler::descramble`]. Since a [`VideoDescrambler`] already 
/// contains the necessary video information, and just need to descramble it, no requests are
/// performed.
/// 
/// # Examples
/// - Constructing using [`Video::from_url`] (or [`Video::from_id`]) (easiest way)
/// ```no_run
///# use rustube::Video;
///# use url::Url;
/// const URL: &str = "https://youtube.com/watch?iv=5jlI4uzZGjU";
/// let url = Url::parse(URL).unwrap();
/// 
///# tokio_test::block_on(async {
/// let video: Video = Video::from_url(&url).await.unwrap();
///# });
/// ``` 
/// - Constructing using [`VideoDescrambler::descramble`]
/// ```no_run
///# use rustube::{Video, VideoFetcher, VideoDescrambler};
///# use url::Url;
/// const URL: &str = "https://youtube.com/watch?iv=5jlI4uzZGjU";
/// let url = Url::parse(URL).unwrap();
/// 
///# tokio_test::block_on(async {
/// let fetcher: VideoFetcher = VideoFetcher::from_url(&url).unwrap();
/// let descrambler: VideoDescrambler = fetcher.fetch().await.unwrap();  
/// let video: Video = descrambler.descramble().unwrap();
///# });
/// ``` 
/// - Construction using chained calls
/// ```no_run
///# use rustube::{Video, VideoFetcher, VideoDescrambler};
///# use url::Url;
/// const URL: &str = "https://youtube.com/watch?iv=5jlI4uzZGjU";
/// let url = Url::parse(URL).unwrap();
/// 
///# tokio_test::block_on(async {
/// let video: Video = VideoFetcher::from_url(&url)
///    .unwrap()
///    .fetch()
///    .await
///    .unwrap()  
///    .descramble()
///    .unwrap();
///# });
/// ``` 
/// - Downloading a video using an existing [`Video`] instance
/// ```no_run
///# use rustube::{Video, VideoFetcher, VideoDescrambler};
///# use url::Url;
///# const URL: &str = "https://youtube.com/watch?iv=5jlI4uzZGjU";
///# let url = Url::parse(URL).unwrap();
///# tokio_test::block_on(async { 
///#     let video: Video = VideoFetcher::from_url(&url).unwrap()
///#        .fetch().await.unwrap()  
///#        .descramble().unwrap();
/// let video_path = video
///    .streams()
///    .iter()
///    .filter(|stream| stream.mime.subtype() == "mp4" && stream.width.is_some())
///    .max_by(|stream0, stream1| stream0.width.unwrap().cmp(&stream1.width.unwrap()))
///    .unwrap()
///    .download()
///    .await
///    .unwrap();
/// 
/// println!(
///    "The video with the id `{}` was downloaded to: `{:?}`",
///    video.id(),
///    video_path 
/// );
///# });
/// ``` 
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt =
"Video({}, streams: {})",
"video_info.player_response.video_details.video_id", "streams.len()"
)]
pub struct Video {
    pub(crate) video_info: VideoInfo,
    pub(crate) streams: Vec<Stream>,
}

impl Video {
    /// Constructs an instance of Video from a Url
    #[inline]
    #[cfg(feature = "download")]
    pub async fn from_url(url: &url::Url) -> crate::Result<Self> {
        crate::VideoFetcher::from_url(url)?
            .fetch()
            .await?
            .descramble()
    }

    #[inline]
    #[cfg(feature = "download")]
    pub async fn from_id(id: crate::IdBuf) -> crate::Result<Self> {
        crate::VideoFetcher::from_id(id)?
            .fetch()
            .await?
            .descramble()
    }

    #[inline]
    pub fn video_info(&self) -> &VideoInfo {
        &self.video_info
    }

    #[inline]
    pub fn streams(&self) -> &Vec<Stream> {
        &self.streams
    }

    #[inline]
    pub fn into_streams(self) -> Vec<Stream> {
        self.streams
    }

    #[inline]
    pub fn video_details(&self) -> Arc<VideoDetails> {
        Arc::clone(&self.video_info.player_response.video_details)
    }

    #[inline]
    pub fn id(&self) -> Id<'_> {
        self.video_info.player_response.video_details.video_id.as_borrowed()
    }

    #[inline]
    pub fn title(&self) -> &str {
        self.video_info.player_response.video_details.title.as_str()
    }

    #[inline]
    pub fn is_age_restricted(&self) -> bool {
        self.video_info.is_age_restricted
    }

    #[inline]
    pub fn best_quality(&self) -> Option<&Stream> {
        self
            .streams
            .iter()
            .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
            .max_by_key(|stream| stream.quality_label)
    }

    #[inline]
    pub fn worst_quality(&self) -> Option<&Stream> {
        self
            .streams
            .iter()
            .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
            .min_by_key(|stream| stream.quality_label)
    }

    #[inline]
    pub fn best_video(&self) -> Option<&Stream> {
        self
            .streams
            .iter()
            .filter(|stream| stream.includes_video_track)
            .max_by_key(|stream| stream.width)
    }

    #[inline]
    pub fn worst_video(&self) -> Option<&Stream> {
        self
            .streams
            .iter()
            .filter(|stream| stream.includes_video_track)
            .min_by_key(|stream| stream.width)
    }

    #[inline]
    pub fn best_audio(&self) -> Option<&Stream> {
        self
            .streams
            .iter()
            .filter(|stream| stream.includes_audio_track)
            .max_by_key(|stream| stream.bitrate)
    }

    #[inline]
    pub fn worst_audio(&self) -> Option<&Stream> {
        self
            .streams
            .iter()
            .filter(|stream| stream.includes_audio_track)
            .min_by_key(|stream| stream.bitrate)
    }
}
