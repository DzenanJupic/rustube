use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::video_info::player_response::video_details::Thumbnail;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Microformat {
    pub player_microformat_renderer: PlayerMicroformatRenderer,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMicroformatRenderer {
    #[serde(rename = "thumbnail")]
    #[serde(serialize_with = "Thumbnail::serialize_vec")]
    #[serde(deserialize_with = "Thumbnail::deserialize_vec")]
    pub thumbnails: Vec<Thumbnail>,
    pub embed: Embed,
    pub title: SimpleText,
    pub description: SimpleText,
    pub length_seconds: String,
    pub owner_profile_url: String,
    pub external_channel_id: String,
    pub available_countries: Vec<String>,
    pub is_unlisted: bool,
    pub has_ypc_metadate: bool,
    pub view_count: i32,
    pub category: String,
    #[serde(with = "crate::serde_impl::date")]
    pub publish_date: DateTime<Utc>,
    pub owner_channel_name: String,
    #[serde(with = "crate::serde_impl::date")]
    pub upload_date: DateTime<Utc>,
    pub live_brodcast_details: Option<LiveBroadcastDetails>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    pub iframe_url: String,
    pub flash_url: String,
    pub width: i32,
    pub height: i32,
    pub flash_secure_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SimpleText {
    simple_text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct LiveBroadcastDetails {
    is_live_now: bool,
    start_simestamp: DateTime<Utc>,
}


