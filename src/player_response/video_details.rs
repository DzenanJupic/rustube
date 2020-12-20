use serde::{Deserialize, Deserializer};
use serde_with::{json::JsonString, serde_as};
use url::Url;

use crate::IdBuf;

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub allow_ratings: bool,
    pub author: String,
    pub average_rating: f64,
    // todo: add Type ChannelId
    pub channel_id: String,
    pub is_crawlable: bool,
    pub is_live_content: bool,
    pub is_owner_viewing: bool,
    pub is_private: bool,
    pub is_unplugged_corpus: bool,
    pub key_words: Option<Vec<String>>,
    #[serde_as(as = "JsonString")]
    pub length_seconds: u64,
    pub short_description: String,
    #[serde(rename = "thumbnail")]
    #[serde(deserialize_with = "Thumbnail::deserialize_vec")]
    pub thumbnails: Vec<Thumbnail>,
    pub title: String,
    #[serde(deserialize_with = "IdBuf::deserialize_owned")]
    pub video_id: IdBuf,
    #[serde_as(as = "JsonString")]
    pub view_count: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Thumbnail {
    pub width: u64,
    pub height: u64,
    pub url: Url,
}

impl Thumbnail {
    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<Self>, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        #[derive(Deserialize)]
        struct Thumbnails { thumbnails: Vec<Thumbnail> }

        Ok(
            Thumbnails::deserialize(deserializer)?
                .thumbnails
        )
    }
}
