use serde::{Deserialize, Serialize};

use crate::video_info::player_response::video_details::Thumbnail;

pub mod channel_video;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelInfo {
    pub title: String,
    pub description: String,
    pub keywords: Option<String>,
    #[serde(rename(deserialize = "urlCanonical"))]
    pub channel_url: String,
    #[serde(deserialize_with = "Thumbnail::deserialize_vec")]
    pub thumbnail: Vec<Thumbnail>,
    pub available_countries: Vec<String>
}