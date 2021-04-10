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
    // TODO: use something specific for a ISO 3166-1 alpha-2 identifier
    pub available_countries: Vec<String>,
    // TODO: maybe also an enum
    pub category: String,
    pub description: SimpleText,
    pub embed: Embed,
    pub external_channel_id: String,
    pub has_ypc_metadate: bool,
    pub is_unlisted: bool,
    pub length_seconds: String,
    pub live_brodcast_details: Option<LiveBroadcastDetails>,
    pub owner_channel_name: String,
    pub owner_profile_url: String,
    #[serde(with = "crate::serde_impl::date_ymd")]
    pub publish_date: DateTime<Utc>,
    #[serde(rename = "thumbnail")]
    #[serde(serialize_with = "Thumbnail::serialize_vec")]
    #[serde(deserialize_with = "Thumbnail::deserialize_vec")]
    pub thumbnails: Vec<Thumbnail>,
    pub title: SimpleText,
    #[serde(with = "crate::serde_impl::date_ymd")]
    pub upload_date: DateTime<Utc>,
    pub view_count: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    pub flash_url: String,
    pub flash_secure_url: String,
    pub iframe_url: String,
    pub height: i32,
    pub width: i32,
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


