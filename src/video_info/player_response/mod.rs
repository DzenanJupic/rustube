use std::sync::Arc;

use serde::Deserialize;

use playability_status::PlayabilityStatus;
use streaming_data::StreamingData;
use video_details::VideoDetails;

pub mod video_details;
pub mod streaming_data;
pub mod playability_status;


#[derive(Clone, Debug, Deserialize)]
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
    // microformat: _,
    pub playability_status: PlayabilityStatus,
    // playbackTracking: _,
    // playerConfig: _,
    // response_context: ResponseContext,
    // storyboards: _,
    pub streaming_data: Option<StreamingData>,
    // trackingParams: "CAAQu2kiEwi__L_qyNftAhWWplUKHVxxAhI=",
    pub video_details: Arc<VideoDetails>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Assets {
    pub js: String
}
