#![feature(option_result_contains, bool_to_option)]

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Clap;

use rustube::{Error, Id, IdBuf, Stream, Video};
use rustube::video_info::player_response::streaming_data::{AudioQuality, Quality, QualityLabel};

#[derive(Clap)]
enum Command {
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
    Checks weather or not a video can be downloaded and if so, prints all available streams\n\
    The check includes fetching, parsing, and descrambling the video data, and also ensuring there \
    is at least one Stream\n\
    Since the video information gets descrambled, you can use all Stream URLs to access the video \
    online\
    ")]
    Check(CheckArgs),
}

#[derive(Clap)]
struct DownloadArgs {
    #[clap(about = "An arbitrary video identifier, like the videos URL or the video id")]
    identifier: String,

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

    #[clap(
    short,
    long,
    about = "Where to download the video to [default: working directory]"
    )]
    dir: Option<PathBuf>,
    #[clap(
    short, long,
    about = "\
    The filename of the video file [default: <VIDEO_ID>.mp4]\n\
    If the file already exists, it will be removed, even if the download fails!\
    "
    )]
    filename: Option<PathBuf>,
}

#[derive(Clap)]
struct FetchArgs {
    #[clap(about = "An arbitrary video identifier, like the videos URL or the video id")]
    identifier: String
}

#[derive(Clap)]
struct CheckArgs {
    #[clap(about = "An arbitrary video identifier, like the videos URL or the video id")]
    identifier: String
}

#[tokio::main]
async fn main() -> Result<()> {
    let command: Command = Command::parse();

    match command {
        Command::Download(args) => {
            let path = download(args).await?;
            println!("Successfully downloaded the video to `{:?}`", path);
        }
        Command::Fetch(args) => {
            let video = fetch(args).await?;
            println!("{:#?}", video);
        }
        Command::Check(args) => {
            let streams = check(args).await?;
            println!("The video can be downloaded\nThere are following streams:\n{:#?}", streams);
        }
    }

    Ok(())
}

async fn download(args: DownloadArgs) -> Result<PathBuf> {
    let id = extract_id(&args.identifier)?;

    let stream = get_video(id)
        .await?
        .into_streams()
        .into_iter()
        .filter(|stream| {
            let no_video_ok = (args.no_video && !stream.includes_video_track) || !args.no_video;
            let no_audio_ok = (args.no_audio && !stream.includes_audio_track) || !args.no_audio;
            let ignore_video_ok = (!args.ignore_missing_video && stream.includes_video_track) || args.ignore_missing_video;
            let ignore_audi_ok = (!args.ignore_missing_audio && stream.includes_audio_track) || args.ignore_missing_audio;

            let quality_ok = args.quality
                .map(|q| stream.quality == q)
                .unwrap_or(true);
            let quality_label_ok = args.video_quality
                .map(|q| stream.quality_label.contains(&q))
                .unwrap_or(true);
            let audio_quality_ok = args.audi_quality
                .map(|q| stream.audio_quality.contains(&q))
                .unwrap_or(true);

            no_video_ok && no_audio_ok && ignore_video_ok && ignore_audi_ok && quality_ok && quality_label_ok && audio_quality_ok
        })
        .max_by(|stream0, stream1| {
            if args.worst_quality {
                cmp_quality(&args, stream1, stream0)
            } else {
                cmp_quality(&args, stream0, stream1)
            }
        })
        .ok_or(Error::NoStreams)
        .context("There are no streams, that match all your criteria")?;

    let res = match (args.dir, args.filename) {
        (Some(dir), Some(filename)) => {
            let path = dir.join(filename);
            stream
                .download_to(&path)
                .await
                .map(|_| dir)
        }
        (Some(dir), None) => {
            stream
                .download_to_dir(dir)
                .await
        }
        (None, Some(filename)) => {
            stream
                .download_to(&filename)
                .await
                .map(|_| filename)
        }
        (None, None) => {
            stream
                .download()
                .await
        }
    };

    res.context("Failed to download the video")
}

async fn fetch(args: FetchArgs) -> Result<Video> {
    let id = extract_id(&args.identifier)?;
    get_video(id).await
}

async fn check(args: CheckArgs) -> Result<Vec<Stream>> {
    let id = extract_id(&args.identifier)?;
    let video = get_video(id).await?;

    (!video.streams().is_empty())
        .then_some(video.into_streams())
        .ok_or(Error::NoStreams)
        .context("There are not streams available")
}

async fn get_video(id: IdBuf) -> Result<Video> {
    Ok(
        Video::from_id(id)
            .await
            .context("Could not fetch or descramble the video information")?
    )
}

fn extract_id(s: &str) -> Result<IdBuf> {
    Ok(
        Id::from_raw(s)
            .context("The supplied video identifier could not be parsed")?
            .into_owned()
    )
}

fn cmp_quality(args: &DownloadArgs, stream0: &Stream, stream1: &Stream) -> std::cmp::Ordering {
    if args.no_video {
        stream0.audio_quality.cmp(&stream1.audio_quality)
    } else if args.no_audio {
        stream0.quality_label.cmp(&stream1.quality_label)
    } else {
        match (stream0.quality_label, stream1.quality_label) {
            (Some(q0), Some(q1)) => {
                return q0.cmp(&q1);
            }
            _ => {}
        }

        match (stream0.audio_quality, stream1.audio_quality) {
            (Some(q0), Some(q1)) => {
                return q0.cmp(&q1);
            }
            _ => {}
        }

        stream0.quality.cmp(&stream1.quality)
    }
}

fn parse_json<'de, T: serde::Deserialize<'de>>(s: &'de str) -> Result<T> {
    serde_json::from_str(s)
        .context("Failed to parse the quality")
}
