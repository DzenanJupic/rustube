#![feature(
async_closure, bool_to_option, cow_is_borrowed, once_cell,
str_split_as_str, str_split_once, try_trait
)]
#![feature(option_result_contains)]


use std::sync::Arc;

use reqwest::{Client, Url};

use player_response::playability_status::{Reason, Status};

use crate::error::Error;
pub use crate::id::{Id, IdBuf};
use crate::player_response::PlayerResponse;
use crate::player_response::streaming_data::StreamingData;
use crate::streams::Stream;
use crate::video_info::VideoInfo;

// pub type OnProgress = Box<dyn Fn(&dyn Any, &[u8], u32)>;
// pub type OnComplete = Box<dyn Fn(&dyn Any, Option<PathBuf>)>;
// todo: use anyhow
pub type Result<T> = std::result::Result<T, Error>;

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

pub struct YouTubeFetcher {
    video_id: IdBuf,
    watch_url: Url,
    client: Client,
}

pub struct YouTubeDescrambler {
    video_info: VideoInfo,
    client: Arc<Client>,
    js: String,
}

pub struct YouTube {
    video_info: VideoInfo,
    fmt_streams: Vec<Stream>,
}

impl YouTubeFetcher {
    #[inline]
    pub fn from_url(url: &Url) -> Result<Self> {
        let id = Id::from_raw(url.as_str())?
            .into_owned();
        Self::from_id(id)
    }

    #[inline]
    pub fn from_id(id: IdBuf) -> Result<Self> {
        let client = Client::builder()
            .cookie_store(true)
            .build()?;

        Ok(Self {
            watch_url: id.watch_url(),
            video_id: id,
            client,
        })
    }

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

        video_info.is_age_restricted = extract::is_age_restricted(&watch_html);
        if let None = video_info.player_response.streaming_data {
            video_info.player_response = player_response;
        }

        Ok(YouTubeDescrambler {
            video_info,
            client: Arc::new(self.client),
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
        Ok(serde_qs::from_str::<VideoInfo>(video_info_raw.as_str())?)
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

#[derive(Clone, Copy)]
enum StreamMap {
    UrlEncodedFmtStream,
    AdaptiveFmts,
}

impl YouTubeDescrambler {
    pub fn descramble(mut self) -> Result<YouTube> {
        let streaming_data = self.video_info.player_response.streaming_data.as_mut()?;
        let stream_maps: &[StreamMap] = match self.video_info.adaptive_fmts {
            Some(_) => &[StreamMap::UrlEncodedFmtStream, StreamMap::AdaptiveFmts],
            None => &[StreamMap::UrlEncodedFmtStream]
        };

        let mut fmt_streams = Vec::new();
        for &fmt in stream_maps {
            extract::apply_descrambler(streaming_data, self.video_info.adaptive_fmts_raw.as_ref(), fmt)?;
            extract::apply_signature(streaming_data, fmt, &self.js)?;
            Self::initialize_streams(streaming_data, &mut fmt_streams, fmt, &self.client)?;
        }

        Ok(YouTube {
            video_info: self.video_info,
            fmt_streams,
        })
    }

    #[inline]
    fn initialize_streams(
        streaming_data: &mut StreamingData,
        streams: &mut Vec<Stream>,
        fmt: StreamMap,
        client: &Arc<Client>,
    ) -> Result<()> {
        let raw_formats = match fmt {
            StreamMap::UrlEncodedFmtStream => {
                std::mem::replace(
                    &mut streaming_data.formats,
                    Vec::new(),
                )
            }
            StreamMap::AdaptiveFmts => {
                std::mem::replace(
                    &mut streaming_data.adaptive_formats,
                    Vec::new(),
                )
            }
        };

        for raw_format in raw_formats {
            let stream = Stream::from_raw_format(raw_format, Arc::clone(client))?;
            streams.push(stream);
        }

        Ok(())
    }
}

impl YouTube {
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
        &self.fmt_streams
    }

    #[inline]
    pub fn video_info(&self) -> &VideoInfo {
        &self.video_info
    }

    #[inline]
    pub fn video_id(&self) -> Id {
        self.video_info.player_response.video_details.video_id.as_borrowed()
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
