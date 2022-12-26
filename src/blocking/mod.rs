//! Blocking wrappers for using `rustube` in a synchronous context.
//!
//! Downloading videos with the blocking API works exactly like described for the asynchronous API 
//! in the [`crate`] documentation, and in the [`Video`](crate::Video) documentation, except for the
//! last step:
//! ```no_run
//!# use rustube::blocking::Video;
//!# use url::Url;
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let url = Url::parse("https://youtube.com/watch?iv=5jlI4uzZGjU")?;
//! let path_to_video = Video::from_url(&url)?
//!    .best_quality()
//!    .unwrap()
//!    .blocking_download()?;
//!#  Ok(())
//!# }
//!```
//!   
//! As you can see, there's no corresponding synchronous version of [`Stream`](crate::Stream), but
//! only a few methods prefixed with `blocking_`. This is not the most beautiful solution and may 
//! change in the future. 
//!  
//! Another option is using the [`block`] macro:
//! ```no_run
//!# use rustube::blocking::Video;
//!# use rustube::block;
//!# use url::Url;
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let url = Url::parse("https://youtube.com/watch?iv=5jlI4uzZGjU")?;
//! let video = Video::from_url(&url)?;
//! let best_quality = video.best_quality().unwrap();
//!  
//! let path_to_video = block!(best_quality.download());
//!#  Ok(())
//!# }
//!```
//!    
//!This macro will utilize the [`Runtime`](tokio::runtime::Runtime) created for you by `rustube`, 
//! and block on the provided future (You can also use it for other asynchronous stuff, not related 
//! to `rustube`).

use once_cell::sync::Lazy;

use tokio::runtime::Runtime;

#[doc(inline)]
#[cfg(feature = "descramble")]
pub use descrambler::VideoDescrambler;
#[doc(inline)]
#[cfg(feature = "fetch")]
pub use fetcher::VideoFetcher;
#[doc(inline)]
#[cfg(feature = "descramble")]
pub use video::Video;
#[doc(inline)]
#[cfg(feature = "playlist")]
pub use playlist::Playlist;

/// A [`Runtime`](tokio::runtime::Runtime) for executing asynchronous code. 
pub static RT: Lazy<Runtime> = Lazy::new(||
    Runtime::new().expect("Unable to start the tokio Runtime")
);

/// A convenient macro for executing asynchronous code in a synchronous context.
#[macro_export]
#[cfg(feature = "blocking")]
macro_rules! block {
    (async $future:block) => { $crate::blocking::RT.block_on(async $future) };
    (async move $future:block) => { $crate::blocking::RT.block_on(async move $future) };
    ($future:expr) => {
        $crate::blocking::RT.block_on(async {
            $future.await
        })
    };
}

#[doc(hidden)]
#[cfg(feature = "fetch")]
pub mod fetcher;
#[doc(hidden)]
#[cfg(feature = "descramble")]
pub mod descrambler;
#[doc(hidden)]
#[cfg(feature = "descramble")]
pub mod video;
#[doc(hidden)]
#[cfg(feature = "playlist")]
pub mod playlist;


/// A synchronous wrapper around [`download_best_quality`](crate::download_best_quality).
#[inline]
#[cfg(all(feature = "download", feature = "regex"))]
pub fn download_best_quality(video_identifier: &str) -> crate::Result<std::path::PathBuf> {
    block!(crate::download_best_quality(video_identifier))
}

/// A synchronous wrapper around [`download_worst_quality`](crate::download_worst_quality).
#[inline]
#[cfg(all(feature = "download", feature = "regex"))]
pub fn download_worst_quality(video_identifier: &str) -> crate::Result<std::path::PathBuf> {
    block!(crate::download_worst_quality(video_identifier))
}
