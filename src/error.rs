use std::borrow::Cow;

use thiserror::Error;

use crate::video_info::player_response::playability_status::PlayabilityStatus;

#[derive(Error, Debug)]
pub enum Error {
    #[error("the provided raw Id does not match any known Id-pattern")]
    BadIdFormat,
    #[cfg(feature = "fetch")]
    #[error("the video you requested is unavailable:\n{0:?}")]
    VideoUnavailable(PlayabilityStatus),
    #[cfg(feature = "download")]
    #[error("the video contains no streams")]
    NoStreams,

    #[error(transparent)]
    #[cfg(feature = "fetch")]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    #[cfg(feature = "fetch")]
    Request(#[from] reqwest::Error),
    #[error("YouTube returned an unexpected response: `{0}`")]
    UnexpectedResponse(Cow<'static, str>),
    #[error(transparent)]
    #[cfg(feature = "fetch")]
    QueryDeserialization(#[from] serde_qs::Error),
    #[error(transparent)]
    #[cfg(feature = "fetch")]
    JsonDeserialization(#[from] serde_json::Error),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error("the itag `{0}` returned by YouTube is not in the known collection of itags")]
    UnknownItag(u64),

    #[error("{0}")]
    Custom(Cow<'static, str>),
    #[error("a potentially dangerous error occurred: {0}")]
    Fatal(String),
    #[error(
    "the error, which occurred is not meant an error, but is used for internal comunication.\
            This error should never be propagated to the public API."
    )]
    Internal(&'static str),
}
