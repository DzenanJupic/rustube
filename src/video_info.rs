use serde::Deserialize;
use serde_with::{json::JsonString, serde_as};

use crate::player_response::PlayerResponse;

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct VideoInfo {
    #[serde_as(as = "JsonString")]
    pub player_response: PlayerResponse,
    #[serde(rename = "adaptive_fmts")]
    pub adaptive_fmts_raw: Option<String>,

    #[serde(skip)]
    pub is_age_restricted: bool,
}
