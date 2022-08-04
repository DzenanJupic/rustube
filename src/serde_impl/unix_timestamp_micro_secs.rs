use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserializer, Serializer};
use serde::de::{Error, Unexpected};
use serde_with::{DeserializeAs, SerializeAs};
use serde_with::json::JsonString;

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, <D as Deserializer<'de>>::Error> where
    D: Deserializer<'de> {
    let micro_seconds: i64 = JsonString::deserialize_as(deserializer)?;
    Utc
        .timestamp_millis_opt(micro_seconds / 1000)
        .single()
        .ok_or_else(|| D::Error::invalid_value(
            Unexpected::Signed(micro_seconds),
            &"a valid UNIX time stamp in microseconds",
        ))
}

pub(crate) fn serialize<S>(time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
    let micro_seconds: i64 = time.timestamp_millis() * 1000;
    JsonString::serialize_as(&micro_seconds, serializer)
}

pub(crate) mod option {
    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        super::deserialize(deserializer).map(Some)
    }

    pub(crate) fn serialize<S>(time: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        match time {
            Some(dt) => super::serialize(dt, serializer),
            None => serializer.serialize_none(),
        }
    }
}
