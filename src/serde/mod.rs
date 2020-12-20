pub mod mime_type {
    use std::lazy::SyncLazy;
    use std::str::FromStr;

    use mime::Mime;
    use regex::Regex;
    use serde::{Deserialize, Deserializer};
    use serde::de::{Error, Unexpected};

    use crate::player_response::streaming_data::MimeType;
    use crate::TryCollect;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<MimeType, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        static PATTERN: SyncLazy<Regex> = SyncLazy::new(||
            Regex::new(r#"(\w+/\w+);\scodecs="([a-zA-Z-0-9.,\s]*)""#).unwrap()
        );

        // deserializing into a &str gives back an error
        let s = String::deserialize(deserializer)?;

        let (mime_type, codecs) = PATTERN
            .captures(&s)
            .ok_or_else(|| D::Error::invalid_value(
                Unexpected::Str(&s),
                &"Expected a valid mime type with the format <TYPE>/<SUBTYPE>",
            ))?
            .iter()
            // skip group 0, which is the whole match
            .skip(1)
            .try_collect()
            .map(|(m, c)| m.map(|m| c.map(|c| (m.as_str(), c.as_str()))))
            .flatten()
            .flatten()
            .ok_or_else(|| D::Error::invalid_value(
                Unexpected::Str(&s),
                &"Expected a valid mime type with the format <TYPE>/<SUBTYPE>",
            ))?;

        let mime = Mime::from_str(mime_type)
            .map_err(|_| D::Error::invalid_value(
                Unexpected::Str(mime_type),
                &r#"Expected a valid mime type with the format `(\w+/\w+);\scodecs="([a-zA-Z-0-9.,\s]*)"`"#,
            ))?;

        let codecs = codecs
            .split(", ")
            .map(str::to_owned)
            .collect();

        Ok(MimeType {
            mime,
            codecs,
        })
    }
}

pub mod range {
    use serde::{Deserialize, Deserializer};
    use serde_with::{DeserializeAs, json::JsonString, serde_as};

    #[serde_as]
    #[derive(Deserialize)]
    pub struct Range {
        #[serde_as(as = "JsonString")]
        start: u64,
        #[serde_as(as = "JsonString")]
        end: u64,
    }

    impl<'de> DeserializeAs<'de, std::ops::Range<u64>> for Range {
        fn deserialize_as<D>(deserializer: D) -> Result<std::ops::Range<u64>, D::Error>
            where
                D: Deserializer<'de> {
            let range = Range::deserialize(deserializer)?;
            Ok(std::ops::Range { start: range.start, end: range.end })
        }
    }
}

pub mod serde_micro_secs {
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
}

pub mod signature_cypher {
    use serde::{Deserialize, Deserializer};
    use serde::de::{Error, Unexpected};

    use crate::player_response::streaming_data::SignatureCypher;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SignatureCypher, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        serde_qs::from_str::<SignatureCypher>(s.as_str())
            .map_err(|_| D::Error::invalid_value(
                Unexpected::Str(s.as_str()),
                &"Expected a valid SignatureCypher",
            ))
    }
}


