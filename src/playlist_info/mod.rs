use serde::{Deserialize, Serialize};

use crate::video_info::player_response::video_details::Thumbnail;

pub(crate) mod req_json;
pub mod playlist_video;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistInfo {
    #[serde(rename = "thumbnail")]
    #[serde(serialize_with = "Thumbnail::serialize_vec")]
    #[serde(deserialize_with = "Thumbnail::deserialize_vec")]
    pub thumbnails: Vec<Thumbnail>,
    pub title: String,
    pub description: String,
    #[serde(rename(deserialize = "urlCanonical"))]
    pub page_url: String
}