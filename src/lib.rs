#![feature(
async_closure, bool_to_option, cow_is_borrowed, once_cell, box_syntax,
str_split_as_str, str_split_once, try_trait, option_result_contains
)]

use derive_more::Display;
use reqwest::Client;
use url::Url;

use player_response::playability_status::{Reason, Status};

use crate::error::Error;
pub use crate::id::{Id, IdBuf};
pub use crate::player_response::PlayerResponse;
pub use crate::player_response::streaming_data::StreamingData;
use crate::player_response::video_details::VideoDetails;
pub use crate::streams::Stream;
pub use crate::video_info::VideoInfo;

// pub type OnProgress = Box<dyn Fn(&dyn Any, &[u8], u32)>;
// pub type OnComplete = Box<dyn Fn(&dyn Any, Option<PathBuf>)>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

mod cipher;
pub mod error;
mod extract;
pub mod id;
mod itags;
mod parser;
pub mod player_response;
mod serde;
pub mod streams;
pub mod video_info;

/// A YouTubeFetcher, used to download all necessary data from YouTube, which then could be used
/// to extract video-urls, or other video information.
/// 
/// You will probably rarely use this type directly, and use [`YouTube`] instead. 
/// 
/// # Example
/// ```no_run
///# use rustube::{YouTubeFetcher, Id, YouTubeDescrambler};
///# use url::Url;
/// const URL: &str = "https://youtube.com/watch?iv=5jlI4uzZGjU";
/// let url = Url::parse(URL).unwrap();
/// 
/// let fetcher: YouTubeFetcher =  YouTubeFetcher::from_url(&url).unwrap();
/// ```
#[derive(Clone, Debug, Display)]
#[display(fmt = "YouTubeFetcher({})", video_id)]
pub struct YouTubeFetcher {
    video_id: IdBuf,
    watch_url: Url,
    client: Client,
}


/// A YouTubeDescrambler, used to decrypt the data fetched by [`YouTubeFetcher`].
///
/// You will probably rarely use this type directly, and use [`YouTube`] instead. 
/// There's no public way of directly constructing a [`YouTubeDescrambler`]. The only way of getting
/// one is by calling [`YouTubeFetcher::fetch`].
///
/// # Example
/// ```no_run
///# use rustube::{YouTubeFetcher, Id, YouTubeDescrambler};
///# use url::Url;
/// const URL: &str = "https://youtube.com/watch?iv=5jlI4uzZGjU";
/// let url = Url::parse(URL).unwrap();
/// 
///# tokio_test::block_on(async {
/// let fetcher: YouTubeFetcher =  YouTubeFetcher::from_url(&url).unwrap();
/// let descrambler: YouTubeDescrambler = fetcher.fetch().await.unwrap();
///# }); 
/// ``` 
#[derive(Clone, Debug, Display)]
#[display(fmt = "YouTubeDescrambler({})", "video_info.player_response.video_details.video_id")]
pub struct YouTubeDescrambler {
    video_info: VideoInfo,
    client: Client,
    js: String,
}


/// A YouTube downloader, which allows you to download all available formats and qualities of a 
/// YouTube video. Each instance of [`YouTube`] represents an existing, available, and downloadable 
/// video.
///
/// This type is the easiest way to download a video. There are tow major ways of constructing an
/// instance of `YouTube`:
/// 1. By using the asynchronous `YouTube::from_*` methods. These methods will take some kind of 
/// video-identifier, like an `Url` or an `Id`, will then internally download the necessary video 
/// information and finally descramble it.
/// 2. By calling [`YouTubeDescrambler::descramble`]. Since a [`YouTubeDescrambler`] already 
/// contains the necessary video information, and just need to descramble it, no requests are
/// performed.
/// 
/// # Examples
/// - Constructing using [`YouTube::from_url`] (easiest way)
/// ```no_run
///# use rustube::YouTube;
///# use url::Url;
/// const URL: &str = "https://youtube.com/watch?iv=5jlI4uzZGjU";
/// let url = Url::parse(URL).unwrap();
/// 
///# tokio_test::block_on(async {
/// let yt: YouTube = YouTube::from_url(&url).await.unwrap();
///# });
/// ``` 
/// - Constructing using [`YouTubeDescrambler::descramble`]
/// ```no_run
///# use rustube::{YouTube, YouTubeFetcher, YouTubeDescrambler};
///# use url::Url;
/// const URL: &str = "https://youtube.com/watch?iv=5jlI4uzZGjU";
/// let url = Url::parse(URL).unwrap();
/// 
///# tokio_test::block_on(async {
/// let fetcher: YouTubeFetcher = YouTubeFetcher::from_url(&url).unwrap();
/// let descrambler: YouTubeDescrambler = fetcher.fetch().await.unwrap();  
/// let yt: YouTube = descrambler.descramble().unwrap();
///# });
/// ``` 
/// - Construction using chained calls
/// ```no_run
///# use rustube::{YouTube, YouTubeFetcher, YouTubeDescrambler};
///# use url::Url;
/// const URL: &str = "https://youtube.com/watch?iv=5jlI4uzZGjU";
/// let url = Url::parse(URL).unwrap();
/// 
///# tokio_test::block_on(async {
/// let yt: YouTube = YouTubeFetcher::from_url(&url)
///    .unwrap()
///    .fetch()
///    .await
///    .unwrap()  
///    .descramble()
///    .unwrap();
///# });
/// ``` 
/// - Downloading a video using an existing [`YouTube`] instance
/// ```no_run
///# use rustube::{YouTube, YouTubeFetcher, YouTubeDescrambler};
///# use url::Url;
///# const URL: &str = "https://youtube.com/watch?iv=5jlI4uzZGjU";
///# let url = Url::parse(URL).unwrap();
///# tokio_test::block_on(async { 
///#     let yt: YouTube = YouTubeFetcher::from_url(&url).unwrap()
///#        .fetch().await.unwrap()  
///#        .descramble().unwrap();
/// let video_path = yt
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
///    yt.video_info().player_response.video_details.video_id,
///    video_path 
/// );
///# });
/// ``` 
#[derive(Clone, Debug, Display)]
#[display(fmt =
"YouTube({}, streams: {})",
"video_info.player_response.video_details.video_id", "streams.len()"
)]
pub struct YouTube {
    video_info: VideoInfo,
    streams: Vec<Stream>,
}

impl YouTubeFetcher {
    /// Creates a YouTubeFetcher from an `Url`.
    /// For more information have a look at the `YouTube` documentation.
    /// # Errors
    /// This method fails if no valid video id can be extracted from the url or when `reqwest` fails
    /// to initialize an new `Client`.
    #[inline]
    pub fn from_url(url: &Url) -> Result<Self> {
        let id = Id::from_raw(url.as_str())?
            .into_owned();
        Self::from_id(id)
    }

    /// Creates a YouTubeFetcher from an `Id`.
    /// For more information have a look at the `YouTube` documentation. 
    /// # Errors
    /// This method fails if `reqwest` fails to initialize an new `Client`.
    #[inline]
    pub fn from_id(video_id: IdBuf) -> Result<Self> {
        let client = Client::builder()
            .cookie_store(true)
            .build()?;
        Ok(Self::from_id_with_client(video_id, client))
    }

    /// Creates a YouTubeFetcher from an `Id` and an existing `Client`.
    /// There are no special constrains, what the `Client` has to look like.
    /// For more information have a look at the `YouTube` documentation.
    #[inline]
    pub fn from_id_with_client(video_id: IdBuf, client: Client) -> Self {
        Self {
            watch_url: video_id.watch_url(),
            video_id,
            client,
        }
    }

    /// Fetches all data necessary to extract important video information.
    /// For more information have a look at the `YouTube` documentation. 
    /// 
    /// # Errors
    /// This method fails, when the video is private, only for members, or otherwise not accessible,
    /// when it cannot request all necessary video resources, or when deserializing the raw response
    /// string into the corresponding Rust types fails.
    /// 
    /// When having a good internet connection, only errors due to inaccessible videos should occur.
    /// Other errors usually mean, that YouTube changed their API, and `rustube` did not adapt to 
    /// this change yet. Please feel free to open a GitHub issue if this is the case.
    /// 
    /// # How it works
    /// So you want to download a YouTube video? You probably already noticed, that YouTube makes 
    /// this quite hard, and does not just provide static urls for their videos. In fact, there's
    /// not the one url for each video. When currently nobody is watching a video, there's actually
    /// no url for this video at all!
    ///
    /// So we need to somehow show YouTube that we want to watch the video, so the YouTube server
    /// generates a url for us. To do this, we do what every 'normal' human being would do: we
    /// request the webpage of the video. To do so, we need nothing more, then the videos id (If you
    /// want to learn more about the id, you can have a look at the [`id`] module. But you don't
    /// need to know anything about it for now.). Let's say for example '5jlI4uzZGjU'. With this id,
    /// we can then visit <https://youtube.com/watch?v=5jlI4uzZGjU>, the site, you as a human, would
    /// go to when just watching the video.
    ///
    /// The next step is to extract as much information from <https://youtube.com/watch?v=5jlI4uzZGjU>
    /// as possible. This is, i.e., information like "is the video age restricted?", or "can we watch
    /// the video without being a member of that channel?".
    ///
    /// But there's information, which is a lot more important then knowing if we are old enough to
    /// to watch the video: The [`VideoInfo`], the [`PlayerResponse`], and the JavaScript of the 
    /// page. [`VideoInfo`] and [`PlayerResponse`] are JSON objects, which contain the most 
    /// important Information about the video. If you are feeling brave, feel free to have a look
    /// at the definitions of those two types, their subtypes, and all the information they contain
    /// (It's huge!). The JavaScript is not processed by fetch, but is used later by `descramble` to
    /// generate the `transform_plan` and the `transform_map` (you will learn about  both when it
    /// comes to descrambling).
    /// 
    /// To get the videos [`VideoInfo`], we actually need to request one more page, which you 
    /// usually probably don't visit as a 'normal' human being. Because we, programmers, are really
    /// creative when it comes to naming stuff, a videos [`VideoInfo`] can be requested at 
    /// <https://youtube.com/get_video_info>. Btw.: If you want to see how the computer feels, when 
    /// we ask him to deserialize the response into the [`VideoInfo`] struct, you can have a look
    /// at <https://www.youtube.com/get_video_info?video_id=5jlI4uzZGjU&eurl=https%3A%2F%2Fyoutube.com%2Fwatch%3Fiv%3D5jlI4uzZGjU&sts=>
    /// (most browsers, will download a text file!). This is the actual [`VideoInfo`] for the
    /// video with the id '5jlI4uzZGjU'. 
    /// 
    /// That's it! Of curse we are not even close to be able to download the video, yet. But that's
    /// not the task of `fetch`. `fetch` is just responsible for requesting all the important 
    /// information. To learn, how the journey continues, have a look at 
    /// [`YouTubeDescrambler::descramble`].  
    pub async fn fetch(self) -> Result<YouTubeDescrambler> {
        let watch_html = self.get_html(&self.watch_url).await?;
        self.check_availability(&watch_html)?;
        let is_age_restricted = extract::is_age_restricted(&watch_html);

        let (
            (js, player_response),
            mut video_info
        ) = tokio::try_join!(
            self.get_js_player_response(is_age_restricted, &watch_html),
            self.get_video_info(is_age_restricted)
        )?;

        if let None = video_info.player_response.streaming_data {
            video_info.player_response = player_response;
        }

        Ok(YouTubeDescrambler {
            video_info,
            client: self.client,
            js,
        })
    }

    async fn get_js_player_response(&self, is_age_restricted: bool, watch_html: &str) -> Result<(String, PlayerResponse)> {
        let (js_url, player_response) = match is_age_restricted {
            true => {
                let embed_url = self.video_id.embed_url();
                let embed_html = self.get_html(&embed_url).await?;
                extract::js_url_player_response(&embed_html)?
            }
            false => extract::js_url_player_response(watch_html)?
        };

        self
            .get_html(&js_url)
            .await
            .map(|html| (html, player_response))
    }

    async fn get_video_info(&self, is_age_restricted: bool) -> Result<VideoInfo> {
        let video_info_url = self.get_video_info_url(is_age_restricted);
        let video_info_raw = self.get_html(&video_info_url).await?;

        let mut video_info = serde_qs::from_str::<VideoInfo>(video_info_raw.as_str())?;
        video_info.is_age_restricted = is_age_restricted;

        Ok(video_info)
    }

    fn get_video_info_url(&self, is_age_restricted: bool) -> Url {
        if is_age_restricted {
            extract::video_info_url_age_restricted(
                self.video_id.as_borrowed(),
                &self.watch_url,
            )
        } else {
            extract::video_info_url(
                self.video_id.as_borrowed(),
                &self.watch_url,
            )
        }
    }

    #[inline]
    fn check_availability(&self, watch_html: &str) -> Result<()> {
        if let Some(playability_status) = extract::playability_status(&watch_html)? {
            for reason in playability_status.messages {
                match (playability_status.status, reason) {
                    (Status::Unplayable, Reason::MembersOnly) => return Err(Error::MembersOnly),
                    (Status::Unplayable, Reason::RecordingNotAvailable) => return Err(Error::RecordingUnavailable),
                    (Status::Unplayable, _) => return Err(Error::VideoUnavailable),
                    (Status::LoginRequired, Reason::PrivateVideo) => return Err(Error::VideoPrivate),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    #[inline]
    async fn get_html(&self, url: &Url) -> Result<String> {
        Ok(
            self.client
                .get(url.as_str())
                .send()
                .await?
                .text()
                .await?
        )
    }
}

impl YouTubeDescrambler {
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
    pub fn descramble(mut self) -> Result<YouTube> {
        let streaming_data = self.video_info.player_response.streaming_data.as_mut().unwrap(); // todo

        if let Some(ref adaptive_fmts_raw) = self.video_info.adaptive_fmts_raw {
            extract::apply_descrambler_adaptive_fmts(streaming_data, adaptive_fmts_raw)?;
        }

        let mut streams = Vec::new();
        extract::apply_signature(streaming_data, &self.js)?;
        Self::initialize_streams(streaming_data, &mut streams, &self.client)?;

        Ok(YouTube {
            video_info: self.video_info,
            streams,
        })
    }

    #[inline]
    fn initialize_streams(
        streaming_data: &mut StreamingData,
        streams: &mut Vec<Stream>,
        client: &Client,
    ) -> Result<()> {
        for raw_format in streaming_data.formats.drain(..).chain(streaming_data.adaptive_formats.drain(..)) {
            let stream = Stream::from_raw_format(raw_format, client.clone())?;
            streams.push(stream);
        }

        Ok(())
    }
}

impl YouTube {
    /// Constructs an instance of YouTube from a Url
    #[inline]
    pub async fn from_url(url: &Url) -> Result<Self> {
        YouTubeFetcher::from_url(url)?
            .fetch()
            .await?
            .descramble()
    }

    #[inline]
    pub async fn from_id(id: IdBuf) -> Result<Self> {
        YouTubeFetcher::from_id(id)?
            .fetch()
            .await?
            .descramble()
    }

    #[inline]
    pub fn streams(&self) -> &Vec<Stream> {
        &self.streams
    }

    #[inline]
    pub fn video_info(&self) -> &VideoInfo {
        &self.video_info
    }

    #[inline]
    pub fn video_details(&self) -> &VideoDetails {
        &self.video_info.player_response.video_details
    }

    #[inline]
    pub fn video_id(&self) -> Id {
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
}

trait TryCollect<T>: Iterator {
    fn try_collect(self) -> Option<T>;
    fn try_collect_lossy(self) -> Option<T> where Self: Sized { None }
}

impl<T> TryCollect<(T::Item, )> for T
    where T: Iterator {
    #[inline]
    fn try_collect(mut self) -> Option<(T::Item, )> {
        match (self.next(), self.next()) {
            (Some(item), None) => Some((item, )),
            _ => None
        }
    }

    #[inline]
    fn try_collect_lossy(mut self) -> Option<(T::Item, )> {
        self.next().map(|v| (v, ))
    }
}

impl<T> TryCollect<(T::Item, T::Item)> for T
    where T: Iterator {
    #[inline]
    fn try_collect(mut self) -> Option<(T::Item, T::Item)> {
        match (self.next(), self.next(), self.next()) {
            (Some(item1), Some(item2), None) => Some((item1, item2)),
            _ => None
        }
    }

    #[inline]
    fn try_collect_lossy(mut self) -> Option<(T::Item, T::Item)> {
        match (self.next(), self.next()) {
            (Some(item1), Some(item2)) => Some((item1, item2)),
            _ => None
        }
    }
}
