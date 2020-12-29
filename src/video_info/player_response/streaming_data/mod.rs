use std::ops::Range;

use chrono::{DateTime, Utc};
use mime::Mime;
use serde::Deserialize;
use serde_with::{DefaultOnNull, json::JsonString};
use serde_with::serde_as;
use url::Url;

mod serde_impl;

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    // todo: remove the field adaptive_formats, and deserialize all formats into formats
    pub adaptive_formats: Vec<RawFormat>,
    #[serde_as(as = "JsonString")]
    pub expires_in_seconds: u64,
    pub formats: Vec<RawFormat>,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawFormat {
    #[serde(rename = "type")]
    pub format_type: Option<FormatType>,
    #[serde_as(as = "JsonString")]
    pub approx_duration_ms: u64,
    pub audio_channels: Option<u8>,
    pub audio_quality: Option<AudioQuality>,
    #[serde(default)]
    #[serde_as(as = "Option<DefaultOnNull<JsonString>>")]
    pub audio_sample_rate: Option<u64>,
    pub average_bitrate: Option<u64>,
    pub bitrate: Option<u64>,
    pub color_info: Option<ColorInfo>,
    #[serde(default)]
    #[serde_as(as = "Option<JsonString>")]
    pub content_length: Option<u64>,
    #[serde(default)]
    pub fps: u8,
    pub height: Option<u64>,
    pub high_replication: Option<bool>,
    #[serde(default)]
    #[serde_as(as = "Option<serde_impl::range::Range>")]
    pub index_range: Option<Range<u64>>,
    #[serde(default)]
    #[serde_as(as = "Option<serde_impl::range::Range>")]
    pub init_range: Option<Range<u64>>,
    pub itag: u64,
    #[serde(with = "serde_impl::serde_micro_secs")]
    pub last_modified: DateTime<Utc>,
    pub loudness_db: Option<f64>,
    #[serde(with = "serde_impl::mime_type")]
    pub mime_type: MimeType,
    pub projection_type: ProjectionType,
    pub quality: Quality,
    pub quality_label: Option<QualityLabel>,
    #[serde(flatten)]
    #[serde(with = "serde_impl::signature_cipher")]
    pub signature_cipher: SignatureCipher,
    pub width: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SignatureCipher {
    pub url: Url,
    pub s: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum FormatType {
    #[serde(rename = "FORMAT_STREAM_TYPE_OTF")]
    Otf,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorInfo {
    primaries: ColorInfoPrimary,
    transfer_characteristics: TransferCharacteristics,
    matrix_coefficients: MatrixCoefficients,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum ColorInfoPrimary {
    #[serde(rename = "COLOR_PRIMARIES_BT709")]
    BT709
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum TransferCharacteristics {
    #[serde(rename = "COLOR_TRANSFER_CHARACTERISTICS_BT709")]
    BT709
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum MatrixCoefficients {
    #[serde(rename = "COLOR_MATRIX_COEFFICIENTS_BT709")]
    BT709
}

#[derive(Clone, Debug)]
pub struct MimeType {
    pub mime: Mime,
    pub codecs: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProjectionType {
    Rectangular,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum AudioQuality {
    #[serde(rename = "AUDIO_QUALITY_LOW")]
    Low,
    #[serde(rename = "AUDIO_QUALITY_MEDIUM")]
    Medium,
    #[serde(rename = "AUDIO_QUALITY_HIGH")]
    High,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    Tiny,
    Small,
    Medium,
    Large,
    Hd720,
    Hd1080,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum QualityLabel {
    #[serde(rename = "144p")]
    P144,
    #[serde(rename = "240p")]
    P240,
    #[serde(rename = "360p")]
    P360,
    #[serde(rename = "480p")]
    P480,
    #[serde(rename = "720p")]
    P720,
    #[serde(rename = "1080p")]
    P1080,
}
