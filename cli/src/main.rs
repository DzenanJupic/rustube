#![feature(option_result_contains, bool_to_option)]

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

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

    let res = match command {
        Command::Check(args) => check(args).await,
        Command::Download(args) => download(args).await,
        Command::Fetch(args) => fetch(args).await,
    };

    if let Err(ref err) = res {
        log::error!("{}\n", err);
    }

    res
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
    let (video_info, stream) = get_stream(id, args.stream_filter).await?;

    stream.download_to(download_path).await?;

    let video_serializer = VideoSerializer::new(
        video_info,
        std::iter::once(stream),
        args.output.output_level,
    );
    let output = args.output.output_format.serialize_output(&video_serializer)?;
    println!("{}", output);

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
) -> Result<(VideoInfo, Stream)> {
    let (video_info, streams) = get_streams(id, &stream_filter).await?;

    let stream = streams
        .max_by(|lhs, rhs| stream_filter.max_stream(lhs, rhs))
        .ok_or(Error::NoStreams)
        .context("There are no streams, that match all your criteria")?;

    Ok((video_info, stream))
}

async fn get_streams(
    id: IdBuf,
    stream_filter: &'_ StreamFilter,
) -> Result<(VideoInfo, impl Iterator<Item=Stream> + '_)> {
    let (video_info, streams) = get_video(id)
        .await?
        .into_parts();

    let streams = streams
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
