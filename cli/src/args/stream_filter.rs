use std::cmp::Ordering;

use clap::Parser;

use rustube::Stream;
use rustube::video_info::player_response::streaming_data::{AudioQuality, Quality, QualityLabel};

#[derive(Parser)]
pub struct StreamFilter {
    #[clap(
    short, long,
    about = "Download the best quality available [default]",
    conflicts_with_all(& ["worst-quality", "video-quality"]),
    )]
    #[allow(unused)]
    best_quality: bool,
    #[clap(
    short, long,
    about = "Download the worst quality available",
    conflicts_with_all(& ["best-quality", "video-quality"])
    )]
    worst_quality: bool,

    #[clap(
    long,
    conflicts_with_all(& ["no-audio", "ignore-missing-audio", "ignore-missing-video"]),
    about = "Pick a Stream, that contains no video track"
    )]
    no_video: bool,
    #[clap(
    long,
    conflicts_with_all(& ["no-video", "ignore-missing-audio", "ignore-missing-video"]),
    about = "Pick a Stream, that contains no audio track"
    )]
    no_audio: bool,

    #[clap(
    long,
    conflicts_with_all(& ["no-audio", "no-video"]),
    about = "Pick a Stream, even if it has no video track"
    )]
    ignore_missing_video: bool,
    #[clap(
    long,
    conflicts_with_all(& ["no-audio", "no-video"]),
    about = "Pick a Stream, even if it has no audio track"
    )]
    ignore_missing_audio: bool,

    #[clap(
    long,
    about = "Download the stream with this quality",
    possible_values = & ["tiny", "small", "medium", "large", "highres", "hd720", "hd1080", "hd1440", "hd2160"],
    conflicts_with_all(& ["best-quality", "worst-quality"]),
    parse(try_from_str = parse_json)
    )]
    quality: Option<Quality>,
    #[clap(
    long,
    about = "Download the stream with this quality label",
    possible_values = & ["144p", "144p60 HDR", "240p", "240p60 HDR", "360p", "360p60", "360p60 HDR",
    "480p", "480p60", "480p60 HDR", "720p", "720p50", "720p60", "720p60 HDR", "1080p", "1080p50",
    "1080p60", "1080p60 HDR", "1440p", "1440p60", "1440p60 HDR", "2160p", "2160p60", "2160p60 HDR",
    "4320p", "4320p60",
    ],
    conflicts_with_all(& ["best-quality", "worst-quality", "no-video"]),
    parse(try_from_str = parse_json)
    )]
    video_quality: Option<QualityLabel>,
    #[clap(
    long,
    about = "\
    Download the stream with this audio quality label (the lowercase values are aliases for the \
    more verbose values)\
    ",
    possible_values = & [
    "AUDIO_QUALITY_LOW", "AUDIO_QUALITY_MEDIUM", "AUDIO_QUALITY_HIGH",
    "low", "medium", "high"
    ],
    conflicts_with_all(& ["best-quality", "worst-quality", "no-audio"]),
    parse(try_from_str = parse_json)
    )]
    audio_quality: Option<AudioQuality>,
}

impl StreamFilter {
    pub fn stream_matches(&self, stream: &Stream) -> bool {
        let video_ok = !self.no_video ^ !(stream.includes_video_track || self.ignore_missing_video);
        let audio_ok = !self.no_audio ^ !(stream.includes_audio_track || self.ignore_missing_audio);
        let quality_ok = self.quality
            .map(|q| stream.quality == q)
            .unwrap_or(true);
        let video_quality_ok = self.video_quality
            .map(|ref q| stream.quality_label.contains(q))
            .unwrap_or(true);
        let audio_quality_ok = self.audio_quality
            .map(|ref q| stream.audio_quality.contains(q))
            .unwrap_or(true);

        let quality_ok = quality_ok && video_quality_ok && audio_quality_ok;

        video_ok && audio_ok && quality_ok
    }

    pub fn max_stream(&self, lhs: &Stream, rhs: &Stream) -> Ordering {
        if self.best_quality || !self.worst_quality {
            self.cmp_stream(lhs, rhs)
        } else /* if self.worst_quality */ {
            self.cmp_stream(rhs, lhs)
        }
    }

    fn cmp_stream(&self, lhs: &Stream, rhs: &Stream) -> Ordering {
        macro_rules! try_ord {
            ($lhs:expr, $rhs:expr) => {
                if let Some(ord) = Self::cmp_opts($lhs, $rhs) {
                    return ord;
                }
            };
        }

        try_ord!(&lhs.width, &rhs.width);
        try_ord!(&lhs.bitrate, &rhs.bitrate);
        try_ord!(&lhs.quality_label, &rhs.quality_label);
        try_ord!(&lhs.audio_quality, &rhs.audio_quality);

        lhs.quality.cmp(&rhs.quality)
    }

    fn cmp_opts<T: Ord>(lsh: &Option<T>, rhs: &Option<T>) -> Option<Ordering> {
        match (lsh, rhs) {
            (Some(lhs), Some(rhs)) => {
                match lhs.cmp(rhs) {
                    Ordering::Equal => None,
                    ord => Some(ord)
                }
            }
            _ => None
        }
    }
}

fn parse_json<T: for<'de> serde::Deserialize<'de>>(s: &str) -> anyhow::Result<T> {
    let args = format!("\"{}\"", s);
    Ok(serde_json::from_str(&args)?)
}
