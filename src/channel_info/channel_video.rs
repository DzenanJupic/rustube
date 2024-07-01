use serde::{Deserialize, Deserializer, Serialize};

use crate::{video_info::player_response::video_details::Thumbnail, IdBuf};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelVideo {
    pub video_id: IdBuf,
    #[serde(deserialize_with = "deserialize_run")]
    pub title: String,
    #[serde(rename(deserialize = "lengthText"))]
    #[serde(deserialize_with = "deserialize_length")]
    pub length_seconds: u64,
    #[serde(rename = "thumbnail")]
    #[serde(deserialize_with = "Thumbnail::deserialize_vec")]
    pub thumbnails: Vec<Thumbnail>,
    #[serde(default)]
    pub author: String,
}

impl ChannelVideo {
    pub(crate) fn add_author(&mut self, author: String) -> Self {
        self.author = author;
        self.to_owned()
    }
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

fn deserialize_length<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Index {
        simple_text: String,
    }

    let index = Index::deserialize(deserializer)?;

    let parts: Vec<&str> = index.simple_text.split(':').collect();
    let mut out_sec = 0;
    let mut multiplier = 1;

    for part in parts.iter().rev() {
        let value: u64 = part.parse().unwrap();
        out_sec += value * multiplier;
        multiplier *= 60;
    }

    Ok(out_sec)
}