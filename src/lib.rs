#![feature(
async_closure, bool_to_option, cow_is_borrowed, once_cell, box_syntax,
str_split_as_str, str_split_once, try_trait, option_result_contains
)]
#![warn(
missing_debug_implementations,
// missing_docs,
rust_2018_idioms,
unreachable_pub
)]
#![deny(broken_intra_doc_links)]

#[doc(inline)]
#[cfg(feature = "descramble")]
pub use crate::descrambler::VideoDescrambler;
#[doc(inline)]
pub use crate::error::Error;
#[doc(inline)]
#[cfg(feature = "fetch")]
pub use crate::fetcher::VideoFetcher;
#[doc(inline)]
pub use crate::id::{Id, IdBuf};
#[doc(inline)]
#[cfg(feature = "stream")]
pub use crate::stream::Stream;
#[doc(inline)]
#[cfg(feature = "descramble")]
pub use crate::video::Video;
#[doc(inline)]
#[cfg(feature = "fetch")]
pub use crate::video_info::{
    player_response::{
        PlayerResponse,
        video_details::VideoDetails,
    },
    VideoInfo,
};

// pub type OnProgress = Box<dyn Fn(&dyn Any, &[u8], u32)>;
// pub type OnComplete = Box<dyn Fn(&dyn Any, Option<PathBuf>)>;
pub type Result<T, E = Error> = core::result::Result<T, E>;

pub mod error;
pub mod id;
#[cfg(feature = "stream")]
pub mod stream;
#[cfg(feature = "fetch")]
pub mod video_info;
#[cfg(feature = "fetch")]
pub mod fetcher;
#[cfg(feature = "descramble")]
pub mod descrambler;
#[cfg(feature = "descramble")]
pub mod video;

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
