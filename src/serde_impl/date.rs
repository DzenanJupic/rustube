use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserializer, Serializer};
use serde::de::{Error, Unexpected};
use serde_with::{DeserializeAs, SerializeAs};
use serde_with::json::JsonString;

static FORMAT: &'static str = "%Y-%m-%d";

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, <D as Deserializer<'de>>::Error> where
    D: Deserializer<'de> {
    let date: String = JsonString::deserialize_as(deserializer)?;
    Utc
        .datetime_from_str(&date, FORMAT)
        .ok()
        .ok_or(D::Error::invalid_value(
            Unexpected::Str(&date),
            &"Expected a valid date string",
        ))
}

pub(crate) fn serialize<S>(time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
    let date: String = time.format(FORMAT).to_string();
    JsonString::serialize_as(&date, serializer)
}
