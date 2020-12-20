#[derive(Clone, Copy, Debug)]
pub enum Error {
    BadIdFormat,
    VideoUnavailable,

    RequestFailed,
    UnexpectedResponse,
    UnknownItag,

    MembersOnly,
    RecordingUnavailable,
    VideoPrivate,

    Other,
}

// todo: error handling

impl From<std::option::NoneError> for Error {
    fn from(_: std::option::NoneError) -> Self {
        Self::Other
    }
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Self::RequestFailed
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Self::Other
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(_: std::str::Utf8Error) -> Self {
        Self::UnexpectedResponse
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Self::Other
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::Other
    }
}

impl From<reqwest::header::ToStrError> for Error {
    fn from(_: reqwest::header::ToStrError) -> Self {
        Self::Other
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::Other
    }
}

impl From<url::ParseError> for Error {
    fn from(_: url::ParseError) -> Self {
        Self::Other
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(_: tokio::task::JoinError) -> Self {
        Self::Other
    }
}

impl From<serde_qs::Error> for Error {
    fn from(_: serde_qs::Error) -> Self {
        Self::Other
    }
}
