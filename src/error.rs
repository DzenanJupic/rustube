use std::borrow::Cow;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("the provided raw Id does not match any known Id-pattern")]
    BadIdFormat,
    #[error("the video you requested is unavailable")]
    VideoUnavailable,
    #[error("could not download the video")]
    DownloadFailed,

    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("YouTube returned an unexpected response: `{0}`")]
    UnexpectedResponse(Cow<'static, str>),
    #[error(transparent)]
    QueryDeserialization(#[from] serde_qs::Error),
    #[error(transparent)]
    JsonDeserialization(#[from] serde_json::Error),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error("the itag `{0}` returned by YouTube is not in the known collection of itags")]
    UnknownItag(u64),

    #[error("the requested video is for members only")]
    MembersOnly,
    #[error("the requested video is private")]
    RecordingUnavailable,
    #[error("the requested video is private")]
    VideoPrivate,

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
