use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use futures::StreamExt;
use mime::Mime;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

use crate::{Result, TryCollect};
use crate::error::Error;
use crate::itags::ItagProfile;
use crate::player_response::streaming_data::{AudioQuality, ColorInfo, FormatType, MimeType, ProjectionType, Quality, QualityLabel, RawFormat, SignatureCypher};

#[derive(Clone, Debug)]
pub struct Stream {
    pub mime: Mime,
    pub codecs: Vec<String>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub bit_rate: Option<u64>,
    pub is_dash: bool,
    pub abr: Option<&'static str>,
    pub resolution: Option<&'static str>,
    pub is_3d: bool,
    pub is_hdr: bool,
    pub is_live: bool,
    pub format_type: Option<FormatType>,
    pub approx_duration_ms: u64,
    pub audio_channels: Option<u8>,
    pub audio_quality: Option<AudioQuality>,
    pub audio_sample_rate: Option<u64>,
    pub average_bitrate: Option<u64>,
    pub bitrate: Option<u64>,
    pub color_info: Option<ColorInfo>,
    pub content_length: u64,
    pub fps: u8,
    pub height: Option<u64>,
    pub high_replication: Option<bool>,
    pub index_range: Option<Range<u64>>,
    pub init_range: Option<Range<u64>>,
    pub is_otf: bool,
    pub itag: u64,
    pub last_modified: DateTime<Utc>,
    pub loudness_db: Option<f64>,
    pub projection_type: ProjectionType,
    pub quality: Quality,
    pub quality_label: Option<QualityLabel>,
    pub signature_cipher: SignatureCypher,
    pub width: Option<u64>,
    client: Arc<Client>,
}


impl Stream {
    pub fn from_raw_format(raw_format: RawFormat, client: Arc<Client>) -> Result<Self> {
        // let url = raw_format.url?;
        let (video_codec, audio_codec) = Self::parse_codecs(
            &raw_format.mime_type
        )?;
        let itag_profile = ItagProfile::from_itag(raw_format.itag);

        Ok(Self {
            mime: raw_format.mime_type.mime,
            codecs: raw_format.mime_type.codecs,
            video_codec,
            audio_codec,
            bit_rate: raw_format.bitrate,
            is_dash: itag_profile.is_dash,
            abr: itag_profile.abr,
            resolution: itag_profile.resolution,
            is_3d: itag_profile.is_3d,
            is_hdr: itag_profile.is_hdr,
            is_live: itag_profile.is_live,
            format_type: raw_format.format_type,
            approx_duration_ms: raw_format.approx_duration_ms,
            audio_channels: raw_format.audio_channels,
            audio_quality: raw_format.audio_quality,
            audio_sample_rate: raw_format.audio_sample_rate,
            average_bitrate: raw_format.average_bitrate,
            bitrate: raw_format.bitrate,
            color_info: raw_format.color_info,
            content_length: raw_format.content_length,
            fps: raw_format.fps,
            height: raw_format.height,
            high_replication: raw_format.high_replication,
            index_range: raw_format.index_range,
            init_range: raw_format.init_range,
            is_otf: raw_format.format_type.contains(&FormatType::Otf),
            itag: raw_format.itag,
            last_modified: raw_format.last_modified,
            loudness_db: raw_format.loudness_db,
            projection_type: raw_format.projection_type,
            quality: raw_format.quality,
            quality_label: raw_format.quality_label,
            signature_cipher: raw_format.signature_cipher,
            width: raw_format.width,
            client,
        })
    }

    pub async fn download(&self) -> Result<PathBuf> {
        let path = self.file_path();
        let mut file = tokio::fs::File::create(&path).await?;
        file.set_len(0).await?;

        let res = self.client
            .get(self.signature_cipher.url.as_str())
            .send()
            .await?;

        if res.status().is_success() {
            let mut stream = res.bytes_stream();

            while let Some(chunk) = stream.next().await {
                file
                    .write_all(&chunk?)
                    .await?;
            }

            return Ok(path);
        }

        // todo: seq_download
        // todo: download in ranges
        // todo: blocking download

        Err(Error::Other)
    }

    #[inline]
    fn parse_codecs(MimeType { mime, codecs }: &MimeType) -> Result<(Option<String>, Option<String>)> {
        if !Self::is_adaptive(codecs) {
            let (video, audio) = codecs.iter().try_collect()?;
            Ok((Some(video.to_owned()), Some(audio.to_owned())))
        } else if Self::includes_video_track(codecs, mime) {
            Ok((codecs.get(0).cloned(), None))
        } else if Self::includes_audio_track(codecs, mime) {
            Ok((None, codecs.get(0).cloned()))
        } else {
            Ok((None, None))
        }
    }

    #[inline]
    fn is_adaptive(codecs: &Vec<String>) -> bool {
        codecs.len() % 2 != 0
    }

    #[inline]
    fn includes_video_track(codecs: &Vec<String>, mime: &Mime) -> bool {
        Self::is_progressive(codecs) || mime.type_() == "video"
    }

    #[inline]
    fn includes_audio_track(codecs: &Vec<String>, mime: &Mime) -> bool {
        Self::is_progressive(codecs) || mime.type_() == "audio"
    }

    #[inline]
    fn is_progressive(codecs: &Vec<String>) -> bool {
        !Self::is_adaptive(codecs)
    }

    fn file_path(&self) -> PathBuf {
        // let file_name = percent_decode_str(self.try_into() url.as_str())
        //     .decode_utf8_lossy()
        //     .to_string();
        // todo
        let file_name = format!("video.mp4");

        let mut path = PathBuf::from(file_name);
        path.set_extension(self.mime.subtype().as_str());
        path
    }
}
