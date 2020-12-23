use chrono::{DateTime, TimeZone, Utc};
use serde::de::{Error, Unexpected};
use serde::Deserializer;
use serde_with::DeserializeAs;
use serde_with::json::JsonString;

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, <D as Deserializer<'de>>::Error> where
    D: Deserializer<'de> {
    let micro_seconds: i64 = JsonString::deserialize_as(deserializer)?;
    Utc
        .timestamp_millis_opt(micro_seconds / 1000)
        .single()
        .ok_or(D::Error::invalid_value(
            Unexpected::Signed(micro_seconds),
            &"Expected a valid UNIX time stamp in microseconds",
        ))
}
