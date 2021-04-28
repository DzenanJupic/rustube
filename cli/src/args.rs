use std::cmp::Ordering;
use std::path::PathBuf;

use anyhow::Context;
use clap::Clap;

use rustube::{
    Id,
    IdBuf, Result, Stream, video_info::player_response::streaming_data::{AudioQuality, Quality, QualityLabel},
};

#[derive(Clap)]
pub enum Command {
    #[clap(about = "\
    Downloads a YouTube video\n\
    By default, the Stream with the best quality and both a video, and an audio track will be \
    downloaded\
    ")]
    Download(DownloadArgs),
    #[clap(about = "\
    Fetches information about a video, and prints it\n\
    Contrary to the name, this will actually fetch and descramble the video information, so you can \
    directly use all Stream URLs to access the video online\
    ")]
    Fetch(FetchArgs),
    #[clap(about = "\
    Checks whether or not a video can be downloaded and if so, prints all available streams\n\
    The check includes fetching, parsing, and descrambling the video data, and also ensuring there \
    is at least one Stream\n\
    Since the video information gets descrambled, you can use all Stream URLs to access the video \
    online\
    ")]
    Check(CheckArgs),
}

#[derive(Clap)]
pub struct DownloadArgs {
    #[clap(flatten)]
    pub identifier: Identifier,
    #[clap(flatten)]
    pub quality_filter: QualityFilter,
    #[clap(flatten)]
    pub stream_filter: StreamFilter,

    #[clap(
    short,
    long,
    about = "Where to download the video to [default: working directory]"
    )]
    pub dir: Option<PathBuf>,
    #[clap(
    short, long,
    about = "\
    The filename of the video file [default: <VIDEO_ID>.mp4]\n\
    If the file already exists, it will be removed, even if the download fails!\
    "
    )]
    pub filename: Option<PathBuf>,
}

#[derive(Clap)]
pub struct FetchArgs {
    #[clap(flatten)]
    pub identifier: Identifier,
    #[clap(flatten)]
    pub quality: QualityFilter,
    #[clap(flatten)]
    pub stream: StreamFilter,
}

#[derive(Clap)]
pub struct CheckArgs {
    #[clap(flatten)]
    pub identifier: Identifier,
}

#[derive(Clap)]
pub struct Identifier {
    #[clap(about = "An arbitrary video identifier, like the videos URL or the video id")]
    identifier: String,
}

impl Identifier {
    pub fn id(&self) -> Result<IdBuf> {
        Ok(
            Id::from_raw(&self.identifier)?
                .into_owned()
        )
    }
}

#[derive(Clap)]
pub struct StreamFilter {
    #[clap(
    short, long,
    about = "Download the best quality available [default]",
    default_value_if("worst-quality", None, "true"),
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
    conflicts_with_all(& ["ignore-missing-audio", "no-audio", "no-video"]),
    about = "Pick a Stream, even if it has no video track"
    )]
    ignore_missing_video: bool,
    #[clap(
    long,
    conflicts_with_all(& ["ignore-missing-video", "no-audio", "no-video"]),
    about = "Pick a Stream, even if it has no audi track"
    )]
    ignore_missing_audio: bool,
}

impl StreamFilter {
    pub fn stream_matches(&self, stream: &Stream) -> bool {
        let no_video_ok = !self.no_video ^ !stream.includes_video_track;
        let no_audio_ok = !self.no_audio ^ !stream.includes_audio_track;
        let ignore_video_ok = self.ignore_missing_video || stream.includes_video_track;
        let ignore_audio_ok = self.ignore_missing_audio || stream.includes_audio_track;

        no_video_ok && no_audio_ok && ignore_video_ok && ignore_audio_ok
    }

    pub fn max_stream(&self, lhs: &Stream, rhs: &Stream) -> Ordering {
        if self.best_quality {
            self.cmp_stream(lhs, rhs)
        } else if self.worst_quality {
            self.cmp_stream(rhs, lhs)
        } else {
            unreachable!("clap should set best_quality by default");
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

#[derive(Clap)]
pub struct QualityFilter {
    #[clap(
    short, long,
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
    audi_quality: Option<AudioQuality>,
}

impl QualityFilter {
    pub fn stream_matches(&self, stream: &Stream) -> bool {
        let quality_ok = self.quality
            .map(|q| stream.quality == q)
            .unwrap_or(true);
        let video_quality_ok = self.video_quality
            .map(|ref q| stream.quality_label.contains(q))
            .unwrap_or(true);
        let audio_quality_ok = self.audi_quality
            .map(|ref q| stream.audio_quality.contains(q))
            .unwrap_or(true);

        quality_ok && video_quality_ok && audio_quality_ok
    }
}

fn parse_json<'de, T: serde::Deserialize<'de>>(s: &'de str) -> anyhow::Result<T> {
    serde_json::from_str(s)
        .context("Failed to parse the quality")
}
