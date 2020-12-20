use std::borrow::Cow;
use std::lazy::SyncLazy;
use std::ops::Deref;

use regex::Regex;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, Unexpected};
use url::Url;

pub type IdBuf = Id<'static>;

// todo: check patterns with regex debugger
/// A list of possible YouTube video identifiers.
/// 
/// ## Guarantees:
/// - each pattern contains an `id` group that will always capture when the pattern matches
/// - The captured id will always match following regex (defined in [ID_PATTERN]): `^[a-zA-Z0-9_-]{11}$`
pub static ID_PATTERNS: [&SyncLazy<Regex>; 4] = [
    &WATCH_URL_PATTERN,
    &EMBED_URL_PATTERN,
    &SHARE_URL_PATTERN,
    &ID_PATTERN
];

pub static WATCH_URL_PATTERN: SyncLazy<Regex> = SyncLazy::new(||
    // watch url    (i.e. https://youtube.com/watch?v=video_id)
    Regex::new(r"^(https?://)?(www\.)?youtube.\w\w\w?/watch\?v=(?P<id>[a-zA-Z0-9_-]{11})(&.*)?$").unwrap()
);
pub static EMBED_URL_PATTERN: SyncLazy<Regex> = SyncLazy::new(||
    // embed url    (i.e. https://youtube.com/embed/video_id)
    Regex::new(r"^(https?://)?(www\.)?youtube.\w\w\w?/embed/(?P<id>[a-zA-Z0-9_-]{11})\\?(\?.*)?$").unwrap()
);
pub static SHARE_URL_PATTERN: SyncLazy<Regex> = SyncLazy::new(||
    // share url    (i.e. https://youtu.be/video_id)
    Regex::new(r"^(https?://)?youtu\.be/(?P<id>[a-zA-Z0-9_-]{11})$").unwrap()
);
pub static ID_PATTERN: SyncLazy<Regex> = SyncLazy::new(||
    // id          (i.e. video_id)
    Regex::new("^(?P<id>[a-zA-Z0-9_-]{11})$").unwrap()
);

/// A wrapper around a Cow<'a, str> that makes sure the video id, which is contained, always
/// has the correct format.
/// 
/// 
/// ## Guaranties:
/// Since YouTube does not guarantee a consistent video-id format, these guarantees can change in 
/// major version updates. If your application depends on them, make sure to check this section on 
/// regular bases!
/// 
/// - The id will always match following regex (defined in [ID_PATTERN]): `^[a-zA-Z0-9_-]{11}$`
/// - The id can always be used as a valid url segment
/// - The id can always be used as a valid url parameter
/// 
/// ## Ownership
/// All available constructors except for [`Id::deserialize_owned`] and [`Id::from_string`] will
/// create the borrowed version with the lifetime of the input. Therefore no allocation is required.
/// 
/// If you require [`Id`] to be owned (`Id<'static`>), you can use [`Id::as_owned`] or 
/// [`Id::into_owned`], which both can easily be chained. You can also use [`IdBuf`], which is
/// an alias for `Id<'static>`, to make functions and types less verbose. 
#[derive(Clone, Debug, serde::Serialize, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Id<'a>(Cow<'a, str>);

impl<'a> Id<'a> {
    pub fn from_raw(raw: &'a str) -> Option<Self> {
        ID_PATTERNS
            .iter()
            .find_map(|pattern|
                pattern
                    .captures(raw)
                    .map(|c| {
                        // will never panic due to guarantees by [`ID_PATTERNS`]
                        let id = c.name("id").unwrap().as_str();
                        Self(Cow::Borrowed(id))
                    })
            )
    }

    #[inline]
    pub fn is_borrowed(&self) -> bool {
        self.0.is_borrowed()
    }

    #[inline]
    pub fn is_owned(&self) -> bool {
        self.0.is_owned()
    }

    #[inline]
    pub fn make_owned(&mut self) -> &mut Self {
        if let Cow::Borrowed(id) = self.0 {
            self.0 = Cow::Owned(id.to_owned());
        }
        self
    }

    #[inline]
    pub fn into_owned(self) -> IdBuf {
        match self.0 {
            Cow::Owned(id) => Id(Cow::Owned(id)),
            Cow::Borrowed(id) => Id(Cow::Owned(id.to_owned()))
        }
    }

    #[inline]
    pub fn as_owned(&self) -> IdBuf {
        self
            .clone()
            .into_owned()
    }

    #[inline]
    pub fn as_borrowed(&'a self) -> Self {
        Self(Cow::Borrowed(&self.0))
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    #[inline]
    pub fn watch_url(&self) -> Url {
        Url::parse_with_params(
            "https://youtube.com/watch?",
            &[("v", self.as_str())],
        ).unwrap()
    }

    #[inline]
    pub fn embed_url(&self) -> Url {
        let mut url = Url::parse("https://youtube.com/embed")
            .unwrap();
        url
            .path_segments_mut()
            .unwrap()
            .push(self.as_str());
        url
    }

    #[inline]
    pub fn share_url(&self) -> Url {
        let mut url = Url::parse("https://youtu.be")
            .unwrap();
        url
            .path_segments_mut()
            .unwrap()
            .push(self.as_str());
        url
    }
}

impl Id<'static> {
    #[inline]
    pub fn from_string(id: String) -> Result<Self, String> {
        match ID_PATTERN.is_match(id.as_str()) {
            true => Ok(Self(Cow::Owned(id))),
            false => Err(id)
        }
    }

    #[inline]
    pub fn deserialize_owned<'de, D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        Ok(
            Id::deserialize(deserializer)?
                .into_owned()
        )
    }
}

impl<'de> Deserialize<'de> for Id<'de> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let raw = <&'de str>::deserialize(deserializer)?;
        Self::from_raw(raw)
            .ok_or(D::Error::invalid_value(
                Unexpected::Str(raw),
                &"expected a valid youtube video identifier",
            ))
    }
}

impl<'a> AsRef<str> for Id<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> Deref for Id<'a> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
