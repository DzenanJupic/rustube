use std::sync::Arc;

use serde::{Deserialize, Serialize};

use playability_status::PlayabilityStatus;
use streaming_data::StreamingData;
use video_details::VideoDetails;
#[cfg(any(feature = "microformat", doc))]
#[doc(cfg(feature = "microformat"))]
use microformat::Microformat;

pub mod video_details;
pub mod streaming_data;
pub mod playability_status;
#[cfg(any(feature = "microformat", doc))]
#[doc(cfg(feature = "microformat"))]
pub mod microformat;


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub assets: Option<Assets>,
    // todo:
    // attestation: _,
    // auxiliaryUi: _,
    // captions: _,
    // cards: _,
    // endscreen: _,
    // messages: _,
    #[cfg(any(feature = "microformat", doc))]
    #[doc(cfg(feature = "microformat"))]
    pub microformat: Microformat,
    pub playability_status: PlayabilityStatus,
    // playbackTracking: _,
    // playerConfig: _,
    // response_context: ResponseContext,
    // storyboards: _,
    pub streaming_data: Option<StreamingData>,
    pub video_details: Arc<VideoDetails>,
    pub tracking_params: String,
}

#[derive(
Clone, Default, Debug, derive_more::Display,
Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash
)]
pub struct Assets {
    pub js: String
}
