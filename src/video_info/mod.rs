//! All the types, that hold video information.

use serde::{Deserialize, Serialize};

use player_response::PlayerResponse;

pub mod player_response;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct VideoInfo {
    pub player_response: PlayerResponse,

    #[serde(skip)]
    pub is_age_restricted: bool,
}
