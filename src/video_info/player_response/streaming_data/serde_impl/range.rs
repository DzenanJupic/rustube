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
