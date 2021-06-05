#![feature(option_result_contains, bool_to_option)]

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Clap;

use args::DownloadArgs;
use args::StreamFilter;
use rustube::{Error, Id, IdBuf, Stream, Video, VideoFetcher, VideoInfo};

use crate::args::{CheckArgs, Command, FetchArgs};
use crate::video_serializer::VideoSerializer;

mod args;
mod output_format;
mod output_level;
mod stream_serializer;
mod video_serializer;

#[tokio::main]
async fn main() -> Result<()> {
    let command: Command = Command::parse();

    match command {
        Command::Check(args) => check(args).await?,
        Command::Download(args) => download(args).await?,
        Command::Fetch(args) => fetch(args).await?,
    }

    Ok(())
}

async fn check(args: CheckArgs) -> Result<()> {
    args.logging.init_logger();

    let id = args.identifier.id()?;
    let (video_info, streams) = get_streams(id, &args.stream_filter).await?;
    let video_serializer = VideoSerializer::new(video_info, streams, args.output.output_level);

    let output = args.output.output_format.serialize_output(&video_serializer)?;
    println!("{}", output);

    Ok(())
}

async fn download(args: DownloadArgs) -> Result<()> {
    args.logging.init_logger();

    let id = args.identifier.id()?;
    let download_path = download_path(args.filename, args.dir, id.as_borrowed());
    let stream = get_stream(id, args.stream_filter).await?;

    stream.download_to(download_path).await?;

    Ok(())
}

async fn fetch(args: FetchArgs) -> Result<()> {
    args.logging.init_logger();

    let id = args.identifier.id()?;
    let video_info = rustube::VideoFetcher::from_id(id)?
        .fetch_info()
        .await?;

    let output = args.output.output_format.serialize_output(&video_info)?;
    println!("{}", output);

    Ok(())
}

async fn get_stream(
    id: IdBuf,
    stream_filter: StreamFilter,
) -> Result<Stream> {
    get_streams(id, &stream_filter)
        .await?.1
        .max_by(|lhs, rhs| stream_filter.max_stream(lhs, rhs))
        .ok_or(Error::NoStreams)
        .context("There are no streams, that match all your criteria")
}

async fn get_streams<'a>(
    id: IdBuf,
    stream_filter: &'a StreamFilter,
) -> Result<(VideoInfo, impl Iterator<Item=Stream> + 'a)> {
    let video = get_video(id)
        .await?;

    let video_info = video.video_info().clone();
    let streams = video
        .into_streams()
        .into_iter()
        .filter(move |stream| stream_filter.stream_matches(stream));

    Ok((video_info, streams))
}

async fn get_video(id: IdBuf) -> Result<Video> {
    VideoFetcher::from_id(id)?
        .fetch()
        .await
        .context("Could not fetch the video information")?
        .descramble()
        .context("Could not descramble the video information")
}

pub fn download_path(filename: Option<PathBuf>, dir: Option<PathBuf>, video_id: Id<'_>) -> PathBuf {
    let filename = filename
        .unwrap_or_else(|| format!("{}.mp4", video_id.as_str()).into());

    let mut path = dir.unwrap_or_else(PathBuf::new);

    path.push(filename);
    path
}
