use chrono::{DateTime, Utc};
use mime::Mime;
use reqwest::Client;
use std::ops::Range;
#[cfg(any(feature = "download", doc))]
#[doc(cfg(feature = "download"))]
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
#[cfg(any(feature = "download", doc))]
#[doc(cfg(feature = "download"))]
use tokio::{
    fs::File,
    io::AsyncWriteExt,
};
#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
use tokio::{sync::mpsc::{self, Sender, Receiver, error::TrySendError}, task};
#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
use std::future::Future;
#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
use std::pin::Pin;
#[cfg(any(feature = "download", doc))]
#[doc(cfg(feature = "download"))]
use tokio_stream::StreamExt;

#[cfg(any(feature = "download", doc))]
#[doc(cfg(feature = "download"))]
use crate::{Error, Result};
use crate::video_info::player_response::streaming_data::{AudioQuality, ColorInfo, FormatType, ProjectionType, Quality, QualityLabel, RawFormat, SignatureCipher};
use crate::VideoDetails;

// todo: 
//  there are different types of streams: video, audio, and video + audio
//  make Stream and RawFormat an enum, so there are less options in it

// maybe:
//  pub type OnProgress = Box<dyn Fn(&dyn Any, &[u8], u32)>;
//  pub type OnComplete = Box<dyn Fn(&dyn Any, Option<PathBuf>)>;
#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
pub type OnProgressClosure = dyn Fn(CallbackArguments);
#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
pub type OnCompleteClosure = dyn Fn(Option<PathBuf>);

/// Arguments given either to a on_progress callback or on_progress receiver
#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
#[derive(Clone, derivative::Derivative)]
#[derivative(Debug)]
pub struct CallbackArguments {
    current_chunk: usize,
}

// TODO: Add Debug
/// Type to process on_progress
#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
pub enum OnProgressType {
    /// Arc containing a closure to execute on progress
    Closure(Arc<OnProgressClosure>),
    // fixme: Find a way to store async closures
    /// Arc containing a async closure to execute on progress
    AsyncClosure(Arc<dyn Fn(CallbackArguments) -> Pin<Box<dyn Future<Output = ()>>>>),
    /// Channel to send a message to on progress,
    /// bool indicates whether or not to cancel on a closed channel
    Channel(Sender<CallbackArguments>, bool),
    None,
}

#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
impl Default for OnProgressType {
    fn default() -> Self {
        OnProgressType::None
    }
}

// TODO: Add Debug
/// Type to process on_progress
#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
pub enum OnCompleteType {
    /// Arc containing a closure to execute on complete
    Closure(Arc<OnCompleteClosure>),
    // fixme: Find a way to store async closures
    /// Arc containing a async closure to execute on complete
    AsyncClosure(Arc<dyn Fn(Option<PathBuf>) -> Pin<Box<dyn Future<Output = ()>>>>),
    None,
}

#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
impl Default for OnCompleteType {
    fn default() -> Self {
        OnCompleteType::None
    }
}

// TODO: Add Debug
/// Methods and streams to process either on_progress or on_complete
#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
pub struct Callback {
    pub on_progress: OnProgressType,
    pub on_complete: OnCompleteType,
    internal_sender: Sender<usize>,
    internal_receiver: Option<Receiver<usize>>,
}

#[cfg(any(feature = "callback", doc))]
#[doc(cfg(feature = "callback"))]
impl Callback {
    /// Create a new callback struct without actual callbacks
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        Callback {
            on_progress: OnProgressType::None,
            on_complete: OnCompleteType::None,
            internal_sender: tx,
            internal_receiver: Some(rx)
        }
    }

    /// Attach a closure to be executed on progress
    ///
    /// ### Warning:
    /// This closure gets executed quite often, once every ~2kB progress.
    /// If it's too slow, some on_progress events will be dropped.
    #[inline]
    pub fn connect_on_progress_closure(mut self, closure: Arc<OnProgressClosure>) -> Self {
        self.on_progress = OnProgressType::Closure(closure);
        self
    }

    /// Attach a closure to be executed on progress
    ///
    /// ### Warning:
    /// This closure gets executed quite often, once every ~2kB progress.
    /// If it's too slow, some on_progress events will be dropped.
    #[inline]
    pub fn connect_on_progress_closure_async(mut self, closure: impl Fn(CallbackArguments) -> dyn Future<Output = ()>) -> Self {
        self.on_progress = OnProgressType::AsyncClosure(Arc::new(|arg| Box::pin(closure(arg))));
        self
    }

    /// Attach a bounded sender that receives messages on progress
    /// cancel_or_close indicates whether or not to cancel the download, if the receiver is closed
    ///
    /// ### Warning:
    /// This sender gets messages quite often, once every ~2kB progress.
    /// If it's too slow, some on_progress events will be dropped.
    #[inline]
    pub fn connect_on_progress_sender(
        mut self,
        sender: Sender<CallbackArguments>,
        cancel_on_close: bool
    ) -> Self {
        self.on_progress = OnProgressType::Channel(sender, cancel_on_close);
        self
    }

    /// Attach a closure to be executed on complete
    #[inline]
    pub fn connect_on_complete_closure(mut self, closure: Arc<OnCompleteClosure>) -> Self {
        self.on_complete = OnCompleteType::Closure(closure);
        self
    }

    /// Attach a closure to be executed on complete
    #[inline]
    pub fn connect_on_complete_closure_async(mut self, closure: impl Fn(Option<PathBuf>) -> dyn Future<Output = ()>) -> Self {
        self.on_complete = OnCompleteType::AsyncClosure(Arc::new(|arg| Box::pin(closure(arg))));
        self
    }
}

#[cfg(not(any(feature = "callback", doc)))]
#[derive(Debug)]
pub struct Callback {}

/// A downloadable video Stream, that contains all the important information. 
#[derive(Clone, derivative::Derivative)]
#[derivative(Debug, PartialEq)]
pub struct Stream {
    pub mime: Mime,
    pub codecs: Vec<String>,
    pub is_progressive: bool,
    pub includes_video_track: bool,
    pub includes_audio_track: bool,
    pub format_type: Option<FormatType>,
    pub approx_duration_ms: Option<u64>,
    pub audio_channels: Option<u8>,
    pub audio_quality: Option<AudioQuality>,
    pub audio_sample_rate: Option<u64>,
    pub average_bitrate: Option<u64>,
    pub bitrate: Option<u64>,
    pub color_info: Option<ColorInfo>,
    #[derivative(PartialEq(compare_with = "atomic_u64_is_eq"))]
    content_length: Arc<AtomicU64>,
    pub fps: u8,
    pub height: Option<u64>,
    pub high_replication: Option<bool>,
    pub index_range: Option<Range<u64>>,
    pub init_range: Option<Range<u64>>,
    pub is_otf: bool,
    pub itag: u64,
    pub last_modified: DateTime<Utc>,
    pub loudness_db: Option<f64>,
    pub projection_type: ProjectionType,
    pub quality: Quality,
    pub quality_label: Option<QualityLabel>,
    pub signature_cipher: SignatureCipher,
    pub width: Option<u64>,
    pub video_details: Arc<VideoDetails>,
    #[derivative(Debug = "ignore", PartialEq = "ignore")]
    client: Client,
}


impl Stream {
    // maybe deserialize RawFormat seeded with client and VideoDetails
    pub(crate) fn from_raw_format(raw_format: RawFormat, client: Client, video_details: Arc<VideoDetails>) -> Self {
        Self {
            is_progressive: is_progressive(&raw_format.mime_type.codecs),
            includes_video_track: includes_video_track(&raw_format.mime_type.codecs, &raw_format.mime_type.mime),
            includes_audio_track: includes_audio_track(&raw_format.mime_type.codecs, &raw_format.mime_type.mime),
            mime: raw_format.mime_type.mime,
            codecs: raw_format.mime_type.codecs,
            format_type: raw_format.format_type,
            approx_duration_ms: raw_format.approx_duration_ms,
            audio_channels: raw_format.audio_channels,
            audio_quality: raw_format.audio_quality,
            audio_sample_rate: raw_format.audio_sample_rate,
            average_bitrate: raw_format.average_bitrate,
            bitrate: raw_format.bitrate,
            color_info: raw_format.color_info,
            content_length: Arc::new(AtomicU64::new(raw_format.content_length.unwrap_or(0))),
            fps: raw_format.fps,
            height: raw_format.height,
            high_replication: raw_format.high_replication,
            index_range: raw_format.index_range,
            init_range: raw_format.init_range,
            is_otf: raw_format.format_type.contains(&FormatType::Otf),
            itag: raw_format.itag,
            last_modified: raw_format.last_modified,
            loudness_db: raw_format.loudness_db,
            projection_type: raw_format.projection_type,
            quality: raw_format.quality,
            quality_label: raw_format.quality_label,
            signature_cipher: raw_format.signature_cipher,
            width: raw_format.width,
            client,
            video_details,
        }
    }
}

// todo: download in ranges
// todo: blocking download

#[cfg(any(feature = "download", doc))]
#[doc(cfg(feature = "download"))]
impl Stream {
    /// The content length of the video.
    /// If the content length was not included in the [`RawFormat`], this method will make a `HEAD`
    /// request, to try to figure it out.
    ///
    /// ### Errors:
    /// - When the content length was not included in the [`RawFormat`], and the request fails.
    #[inline]
    pub async fn content_length(&self) -> Result<u64> {
        let cl = self.content_length.load(Ordering::SeqCst);
        if cl != 0 { return Ok(cl); }

        self.client
            .head(self.signature_cipher.url.as_str())
            .send()
            .await?
            .error_for_status()?
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|cl| cl.to_str().ok())
            .and_then(|cl| cl.parse::<u64>().ok())
            .map(|cl| {
                log::trace!("content length of {:?} is {}", self, cl);
                self.content_length.store(cl, Ordering::SeqCst);
                cl
            })
            .ok_or_else(|| Error::UnexpectedResponse(
                "the response did not contain a valid content-length field".into()
            ))
    }

    /// Attempts to downloads the [`Stream`]s resource.
    /// This will download the video to <video_id>.mp4 in the current working directory.
    #[inline]
    pub async fn download(&self) -> Result<PathBuf> {
        self.internal_download(None).await
    }

    /// Attempts to downloads the [`Stream`]s resource.
    /// This will download the video to <video_id>.mp4 in the current working directory.
    /// Takes an [`Callback`](crate::stream::Callback)
    #[cfg(any(feature = "callback", doc))]
    #[doc(cfg(feature = "callback"))]
    #[inline]
    pub async fn download_callback(&self, callback: Callback) -> Result<PathBuf> {
        self.internal_download(Some(callback)).await
    }

    #[inline]
    async fn internal_download(&self, callback: Option<Callback>) -> Result<PathBuf> {
        let path = Path::new(self.video_details.video_id.as_str())
            .with_extension("mp4");
        self.internal_download_to(&path, callback)
            .await
            .map(|_| path)
    }

    /// Attempts to downloads the [`Stream`]s resource.
    /// This will download the video to <video_id>.mp4 in the provided directory.
    #[inline]
    pub async fn download_to_dir<P: AsRef<Path>>(&self, dir: P) -> Result<PathBuf> {
        self.internal_download_to_dir(dir, None).await
    }

    /// Attempts to downloads the [`Stream`]s resource.
    /// This will download the video to <video_id>.mp4 in the provided directory. 
    /// Takes an [`Callback`](crate::stream::Callback)
    #[cfg(any(feature = "callback", doc))]
    #[doc(cfg(feature = "callback"))]
    #[inline]
    pub async fn download_to_dir_callback<P: AsRef<Path>>(
        &self,
        dir: P,
        callback: Callback
    ) -> Result<PathBuf> {
        self.internal_download_to_dir(dir, Some(callback)).await
    }

    #[inline]
    async fn internal_download_to_dir<P: AsRef<Path>>(
        &self,
        dir: P,
        callback: Option<Callback>
    ) -> Result<PathBuf> {
        let mut path = dir
            .as_ref()
            .join(self.video_details.video_id.as_str());
        path.set_extension("mp4");
        self.internal_download_to(&path, callback)
            .await
            .map(|_| path)
    }

    #[cfg(feature = "callback")]
    #[inline]
    async fn on_progress(mut receiver: Receiver<usize>, on_progress: OnProgressType) {
        match on_progress {
            OnProgressType::None => {},
            OnProgressType::Closure(closure) => {
                while let Some(data) = receiver.recv().await {
                    let arguments = CallbackArguments { current_chunk: data };
                    closure(arguments);
                }
            }
            OnProgressType::AsyncClosure(closure) => {
                while let Some(data) = receiver.recv().await {
                    let arguments = CallbackArguments { current_chunk: data };
                    closure(arguments).await;
                }
            }
            OnProgressType::Channel(sender, cancel_on_close) => {
                while let Some(data) = receiver.recv().await {
                    let arguments = CallbackArguments { current_chunk: data };
                    // await if channel is full
                    match sender.send(arguments).await {
                        // close channel to internal loop on closed outer channel
                        Err(_) => if cancel_on_close {receiver.close()}
                        _ => {}
                    }
                }
            }
        }
    }

    #[cfg(feature = "callback")]
    #[inline]
    async fn on_complete(on_complete: OnCompleteType, path: Option<PathBuf>) {
        match on_complete {
            OnCompleteType::None => {},
            OnCompleteType::Closure(closure) => {
                closure(path)
            }
            OnCompleteType::AsyncClosure(closure) => {
                closure(path).await
            }
        }
    }

    /// Attempts to downloads the [`Stream`]s resource.
    /// This will download the video to the provided file path.
    #[inline]
    pub async fn download_to<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.internal_download_to(path, None).await
    }

    /// Attempts to downloads the [`Stream`]s resource.
    /// This will download the video to the provided file path.
    /// Takes an [`Callback`](crate::stream::Callback)
    #[cfg(any(feature = "callback", doc))]
    #[doc(cfg(feature = "callback"))]
    #[inline]
    pub async fn download_to_callback<P: AsRef<Path>>(&self, path: P, callback: Callback) -> Result<()> {
        self.internal_download_to(path, Some(callback)).await
    }

    #[allow(unused_mut)]
    async fn internal_download_to<P: AsRef<Path>>(&self, path: P, mut callback: Option<Callback>) -> Result<()> {
        log::trace!("download_to: {:?}", path.as_ref());
        let mut file = File::create(&path).await?;

        // fixme: Requires 'static
        #[cfg(feature = "callback")]
        let handle = if let Some(ref mut callback) = callback {
            Some(task::spawn(Self::on_progress(
                callback.internal_receiver.take().expect("Callback cannot be used twice"),
                std::mem::take(&mut callback.on_progress)
            )))
        } else {
            None
        };

        let result = match self.download_full(&self.signature_cipher.url, &mut file, &callback).await {
            Ok(_) => {
                log::info!(
                    "downloaded {} successfully to {:?}",
                    self.video_details.video_id, path.as_ref()
                );
                log::debug!("downloaded stream {:?}", &self);
                Ok(())
            }
            Err(Error::Request(e)) if e.status().contains(&reqwest::StatusCode::NOT_FOUND) => {
                log::error!("failed to download {}: {:?}", self.video_details.video_id, e);
                log::info!("try to download {} using sequenced download", self.video_details.video_id);
                // Some adaptive streams need to be requested with sequence numbers
                self.download_full_seq(&mut file, &callback)
                    .await
                    .map_err(|e| {
                        log::error!(
                            "failed to download {} using sequenced download: {:?}",
                            self.video_details.video_id, e
                        );
                        e
                    })
            }
            Err(e) => {
                log::error!("failed to download {}: {:?}", self.video_details.video_id, e);
                drop(file);
                tokio::fs::remove_file(path.as_ref()).await?;
                Err(e)
            }
        };

        #[cfg(feature = "callback")]
        {
            if let Some(handle) = handle {
                handle.abort();
            }
            let path = if let Ok(_) = &result {
                let mut pathbuf = PathBuf::new();
                pathbuf.push(path);
                Some(pathbuf)
            } else {
                None
            };
            if let Some(ref mut callback) = callback {
                Self::on_complete(std::mem::take(&mut callback.on_complete), path);
            }
        }


        result
    }

    async fn download_full_seq(&self, file: &mut File, callback: &Option<Callback>) -> Result<()> {
        // fixme: this implementation is **not** tested yet!
        // To test it, I would need an url of a video, which does require sequenced downloading.
        log::warn!(
            "`download_full_seq` is not tested yet and probably broken!\n\
            Please open a GitHub issue and paste the whole warning message in:\n\
            id: {}\n\
            url: {}",
            self.video_details.video_id,
            self.signature_cipher.url.as_str()
        );

        let mut url = self.signature_cipher.url.clone();
        let base_query = url
            .query()
            .map(str::to_owned)
            .unwrap_or_else(|| String::new());

        // The 0th sequential request provides the file headers, which tell us
        // information about how the file is segmented.
        Self::set_url_seq_query(&mut url, &base_query, 0);
        let res = self.get(&url).await?;
        let segment_count = Stream::extract_segment_count(&res)?;
        // No callback action since this is not really part of the progress
        self.write_stream_to_file(res.bytes_stream(), file, &None).await?;

        for i in 1..segment_count {
            Self::set_url_seq_query(&mut url, &base_query, i);
            self.download_full(&url, file, &callback).await?;
        }

        Ok(())
    }

    #[inline]
    async fn download_full(
        &self,
        url: &url::Url,
        file: &mut File,
        callback: &Option<Callback>
    ) -> Result<usize> {
        let res = self.get(url).await?;
        self.write_stream_to_file(res.bytes_stream(), file, &callback).await
    }

    #[inline]
    async fn get(&self, url: &url::Url) -> Result<reqwest::Response> {
        log::trace!("get: {}", url.as_str());
        Ok(
            self.client
                .get(url.as_str())
                .send()
                .await?
                .error_for_status()?
        )
    }

    #[inline]
    #[allow(unused_variables)]
    async fn write_stream_to_file(
        &self,
        mut stream: impl tokio_stream::Stream<Item=reqwest::Result<bytes::Bytes>> + Unpin,
        file: &mut File,
        callback: &Option<Callback>,
    ) -> Result<usize> {
        // Counter will be 0 if callback is not enabled
        #[allow(unused_mut)]
        let mut counter = 0;
        #[cfg(feature = "callback")]
        let channel = callback
            .as_ref()
            .map(|c| c.internal_sender.clone());
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file
                .write_all(&chunk)
                .await?;
            #[cfg(feature = "callback")]
            if let Some(channel) = &channel {
                counter += chunk.len();
                // Will continue even if the receiver is closed
                // Will ignore if the channel is full and thus not slow down the download
                match channel.try_send(counter) {
                    Err(TrySendError::Closed(_)) => return Err(Error::ChannelClosed),
                    _ => {}
                }
            }
        }
        Ok(counter)
    }

    #[inline]
    fn set_url_seq_query(url: &mut url::Url, base_query: &str, sq: u64) {
        url.set_query(Some(&base_query));
        url
            .query_pairs_mut()
            .append_pair("sq", &sq.to_string());
    }

    #[inline]
    fn extract_segment_count(res: &reqwest::Response) -> Result<u64> {
        Ok(
            res
                .headers()
                .get("Segment-Count")
                .ok_or_else(|| Error::UnexpectedResponse(
                    "sequence download request did not contain a Segment-Count".into()
                ))?
                .to_str()
                .map_err(|_| Error::UnexpectedResponse(
                    "Segment-Count is not valid utf-8".into()
                ))?
                .parse::<u64>()
                .map_err(|_| Error::UnexpectedResponse(
                    "Segment-Count could not be parsed into an integer".into()
                ))?
        )
    }
}

#[cfg(any(all(feature = "stream", feature = "blocking"), doc))]
#[doc(cfg(all(feature = "stream", feature = "blocking")))]
impl Stream {
    /// A synchronous wrapper around [`Stream::download`](crate::Stream::download).
    #[inline]
    pub fn blocking_download(&self) -> Result<PathBuf> {
        Ok(crate::block!(self.download())?)
    }

    /// A synchronous wrapper around [`Stream::download_callback`](crate::Stream::download_callback).
    #[cfg(any(feature = "callback", doc))]
    #[doc(cfg(feature = "callback"))]
    #[inline]
    pub fn blocking_download_callback(&self, callback: Option<Callback>) -> Result<PathBuf> {
        Ok(crate::block!(self.download_callback(callback))?)
    }

    /// A synchronous wrapper around [`Stream::download_to_dir`](crate::Stream::download_to_dir). 
    #[inline]
    pub fn blocking_download_to_dir<P: AsRef<Path>>(&self, dir: P) -> Result<PathBuf> {
        Ok(crate::block!(self.download_to_dir(dir))?)
    }

    /// A synchronous wrapper around [`Stream::download_to_dir_callback`](crate::Stream::download_to_dir_callback).
    #[cfg(any(feature = "callback", doc))]
    #[doc(cfg(feature = "callback"))]
    #[inline]
    pub fn blocking_download_to_dir_callback<P: AsRef<Path>>(
        &self,
        dir: P,
        callback: Option<Callback>
    ) -> Result<PathBuf> {
        Ok(crate::block!(self.download_to_dir_callback(dir, callback))?)
    }

    /// A synchronous wrapper around [`Stream::download_to`](crate::Stream::download_to).
    pub fn blocking_download_to<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        Ok(crate::block!(self.download_to(path))?)
    }

    /// A synchronous wrapper around [`Stream::download_to_callback`](crate::Stream::download_to_callback).
    #[cfg(any(feature = "callback", doc))]
    #[doc(cfg(feature = "callback"))]
    pub fn blocking_download_to_callback<P: AsRef<Path>>(&self, path: P, callback: Option<Callback>) -> Result<()> {
        Ok(crate::block!(self.download_to_callback(path, callback))?)
    }

    /// A synchronous wrapper around [`Stream::content_length`](crate::Stream::content_length).
    #[inline]
    pub fn blocking_content_length(&self) -> Result<u64> {
        crate::block!(self.content_length())
    }
}

#[inline]
fn is_adaptive(codecs: &Vec<String>) -> bool {
    codecs.len() % 2 != 0
}

#[inline]
fn includes_video_track(codecs: &Vec<String>, mime: &Mime) -> bool {
    is_progressive(codecs) || mime.type_() == "video"
}

#[inline]
fn includes_audio_track(codecs: &Vec<String>, mime: &Mime) -> bool {
    is_progressive(codecs) || mime.type_() == "audio"
}

#[inline]
fn is_progressive(codecs: &Vec<String>) -> bool {
    !is_adaptive(codecs)
}

#[inline]
fn atomic_u64_is_eq(lhs: &Arc<AtomicU64>, rhs: &Arc<AtomicU64>) -> bool {
    lhs.load(Ordering::Acquire) == rhs.load(Ordering::Acquire)
}
