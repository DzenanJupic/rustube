use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{serde_as, json::JsonString};

use crate::{video_info::player_response::video_details::Thumbnail, IdBuf};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistVideo {
    #[serde(deserialize_with = "deserialize_index")]
    pub index: u64,
    pub video_id: IdBuf,
    #[serde(deserialize_with = "deserialize_run")]
    pub title: String,
    #[serde_as(as = "JsonString")]
    pub length_seconds: u64,
    #[serde(rename = "thumbnail")]
    #[serde(deserialize_with = "Thumbnail::deserialize_vec")]
    pub thumbnails: Vec<Thumbnail>,
    #[serde(rename(deserialize = "shortBylineText"))]
    #[serde(deserialize_with = "deserialize_run")]
    pub author: String,
}

fn deserialize_run<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct RunRoot {
        runs: Vec<Run>,
    }

    #[derive(Deserialize)]
    struct Run {
        text: String,
    }

    let title = RunRoot::deserialize(deserializer)?;
    Ok(title.runs.into_iter().next().map(|run| run.text).unwrap_or_default())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Run {
    text: String,
}

fn deserialize_index<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Index {
        simple_text: String,
    }

    let index = Index::deserialize(deserializer)?;
    index.simple_text.parse::<u64>().map_err(serde::de::Error::custom)
}