use std::cmp::Ordering;

use anyhow::Context;
use clap::Clap;

use rustube::Stream;
use rustube::video_info::player_response::streaming_data::{AudioQuality, Quality, QualityLabel};

#[derive(Clap)]
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
    conflicts_with_all(& ["best-quality", "worst-quality"]),
    parse(try_from_str = parse_json)
    )]
    quality: Option<Quality>,
    #[clap(
    long,
    about = "Download the stream with this quality label",
    conflicts_with_all(& ["best-quality", "worst-quality", "no-video"]),
    parse(try_from_str = parse_json)
    )]
    video_quality: Option<QualityLabel>,
    #[clap(
    long,
    about = "Download the stream with this audio quality label",
    conflicts_with_all(& ["best-quality", "worst-quality", "no-audio"]),
    parse(try_from_str = parse_json)
    )]
    audio_quality: Option<AudioQuality>,
}

impl StreamFilter {
    pub fn stream_matches(&self, stream: &Stream) -> bool {
        let no_video_ok = !self.no_video ^ !stream.includes_video_track;
        let no_audio_ok = !self.no_audio ^ !stream.includes_audio_track;
        let ignore_video_ok = self.ignore_missing_video || stream.includes_video_track;
        let ignore_audio_ok = self.ignore_missing_audio || stream.includes_audio_track;
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

        no_video_ok && no_audio_ok && ignore_video_ok && ignore_audio_ok && quality_ok
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

fn parse_json<'de, T: serde::Deserialize<'de>>(s: &'de str) -> anyhow::Result<T> {
    serde_json::from_str(s)
        .context("Failed to parse the quality")
}
