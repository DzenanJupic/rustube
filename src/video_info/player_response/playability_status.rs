use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayabilityStatus {
    pub status: Status,
    pub playable_in_embed: bool,
    pub miniplayer: MiniPlayer,
    #[serde(default)]
    pub messages: Vec<Reason>,
    pub context_params: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniPlayer {
    pub miniplayer_renderer: MiniplayerRenderer
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniplayerRenderer {
    pub playback_mode: PlaybackMode
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlaybackMode {
    #[serde(rename = "PLAYBACK_MODE_ALLOW")]
    Allow
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Ok,
    Unplayable,
    LoginRequired,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Reason {
    #[serde(rename = "Join this channel to get access to members-only content like this video, and other exclusive perks.")]
    MembersOnly,
    #[serde(rename = "This live stream recording is not available.")]
    RecordingNotAvailable,
    #[serde(rename = "This is a private video. Please sign in to verify that you may see it.")]
    PrivateVideo,
}
