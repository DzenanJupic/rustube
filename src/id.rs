use std::borrow::Cow;
use std::cmp::Ordering;
use std::lazy::SyncLazy;

use regex::Regex;
#[cfg(feature = "serde")]
use serde::{
    de::{Error as SerdeError, Unexpected},
    Deserialize, Deserializer, Serialize,
};
use url::Url;

use crate::{Error, Result};

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
/// All available constructors except for [`Id::deserialize`] and [`Id::from_string`] will
/// create the borrowed version with the lifetime of the input. Therefore no allocation is required.
/// 
/// If you don't need 'static deserialization, you can use [`Id::deserialize_borrowed`], which will
/// create an `Id<'de>`.
/// 
/// If you require [`Id`] to be owned (`Id<'static`>), you can use [`Id::as_owned`] or 
/// [`Id::into_owned`], which both can easily be chained. You can also use [`IdBuf`], which is
/// an alias for `Id<'static>`, to make functions and types less verbose. 
#[derive(Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Id<'a>(Cow<'a, str>);

impl<'a> Id<'a> {
    pub fn from_raw(raw: &'a str) -> Result<Self> {
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
            .ok_or(Error::BadIdFormat)
    }

    #[inline]
    pub fn from_str(id: &'a str) -> Result<Self> {
        match ID_PATTERN.is_match(id) {
            true => Ok(Self(Cow::Borrowed(id))),
            false => Err(Error::BadIdFormat)
        }
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

    /// Creates an `&IdBuf` from an arbitrary `Id`.
    /// 
    /// By just returning a reference, `&IdBuf` cannot outlive `Id`, even if the original 
    /// string might still live:
    /// ```compile_fail
    ///# use rustube::Id;
    /// let string = String::from("12345678910");
    /// // create a borrowed Id bound to the live time of string 
    /// let id: Id = Id::from_raw(&string).unwrap();
    /// // create a reference to an IdBuf bound to the lifetime of id
    /// let id_static: &Id<'static> = id.as_static();
    /// // give ownership of id away | this **must** also invalidate id_static
    /// let id_buf = id.into_owned();
    /// // trying to access id_static now, throws a compile time error
    /// let str_static = id_static.as_str();
    /// ```
    #[inline]
    pub fn as_static(&'a self) -> &'a IdBuf {
        // SAFETY:
        // This method returns a reference with the lifetime of 'a.
        // Therefore the returned IdBuf cannot outlive self (also have a look at the doc-test). 
        unsafe { std::mem::transmute::<&'a Id<'a>, &'a Id<'static>>(&self) }
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

impl IdBuf {
    #[inline]
    pub fn from_string(id: String) -> Result<Self, String> {
        match ID_PATTERN.is_match(id.as_str()) {
            true => Ok(Self(Cow::Owned(id))),
            false => Err(id)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Id<'de> {
    #[inline]
    pub fn deserialize_borrowed<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de> {
        let raw = <&'de str>::deserialize(deserializer)?;
        Self::from_raw(raw)
            .map_err(|_| D::Error::invalid_value(
                Unexpected::Str(raw),
                &"expected a valid youtube video identifier",
            ))
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Id<'static> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de> {
        let raw = <&'de str>::deserialize(deserializer)?;
        Id::from_raw(raw)
            .map(|id: Id<'de>| id.into_owned())
            .map_err(|_| D::Error::invalid_value(
                Unexpected::Str(raw),
                &"expected a valid youtube video identifier",
            ))
    }
}


impl core::fmt::Display for Id<'_> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl core::ops::Deref for Id<'_> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl core::convert::AsRef<str> for Id<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<T> core::cmp::PartialEq<T> for Id<'_>
    where
        T: core::convert::AsRef<str> {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        core::cmp::PartialEq::eq(
            self.as_str(),
            other.as_ref(),
        )
    }
}

impl core::cmp::Eq for Id<'_> {}

impl core::cmp::Ord for Id<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<T> core::cmp::PartialOrd<T> for Id<'_>
    where
        T: AsRef<str> {
    #[inline]
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        core::cmp::PartialOrd::partial_cmp(
            self.as_str(),
            other.as_ref(),
        )
    }
}
