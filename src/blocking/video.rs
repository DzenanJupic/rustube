use std::ops::{Deref, DerefMut};

use url::Url;

use crate::{IdBuf, Result, Stream};
use crate::video::Video as AsyncVideo;

/// A synchronous wrapper around [`Video`](crate::Video).
#[derive(Clone, Debug, derive_more::Display, PartialEq)]
pub struct Video(pub(super) AsyncVideo);

impl Video {
    /// A synchronous wrapper around [`Video::form_url`](crate::Video::from_url).
    /// 
    /// Creates a [`Video`] from an [`Url`].
    /// ### Errors
    /// - When [`VideoFetcher::from_url`](crate::VideoFetcher::from_url) fails.
    /// - When [`VideoFetcher::fetch`](crate::VideoFetcher::fetch) fails.
    /// - When [`VideoDescrambler::descramble`](crate::VideoDescrambler::descramble) fails.
    #[inline]
    pub fn from_url(url: &Url) -> Result<Self> {
        Ok(Self(block!(AsyncVideo::from_url(url))?))
    }


    /// A synchronous wrapper around [`Video::form_id`](crate::Video::from_id).
    ///
    /// Creates a [`Video`] from an [`Id`](crate::Id).
    /// ### Errors
    /// - When [`VideoFetcher::fetch`](crate::VideoFetcher::fetch) fails.
    /// - When [`VideoDescrambler::descramble`](crate::VideoDescrambler::descramble) fails.
    #[inline]
    pub fn from_id(id: IdBuf) -> Result<Self> {
        Ok(Self(block!(AsyncVideo::from_id(id))?))
    }

    /// Takes all [`Stream`]s of the video.
    #[inline]
    pub fn into_streams(self) -> Vec<Stream> {
        self.0.streams
    }
}

impl Deref for Video {
    type Target = AsyncVideo;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Video {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
