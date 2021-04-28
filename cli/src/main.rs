#![feature(option_result_contains, bool_to_option)]

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Clap;

use rustube::{Error, Id, IdBuf, Stream, Video, VideoFetcher};

use crate::args::{CheckArgs, Command, DownloadArgs, FetchArgs, QualityFilter, StreamFilter};

mod args;

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
    let id = args.identifier.id()?;
    let download_path = download_path(args.filename, args.dir, id.as_borrowed());
    let stream = get_stream(id, args.stream_filter, args.quality_filter).await?;

    stream.download_to(&download_path).await?;

    Ok(download_path)
}

async fn fetch(args: FetchArgs) -> Result<Video> {
    let id = args.identifier.id()?;
    get_video(id).await
}

async fn check(args: CheckArgs) -> Result<Vec<Stream>> {
    let id = args.identifier.id()?;
    let video = get_video(id).await?;

    (!video.streams().is_empty())
        .then_some(video.into_streams())
        .ok_or(Error::NoStreams)
        .context("There are not streams available")
}

async fn get_stream(
    id: IdBuf,
    stream_filter: StreamFilter,
    quality_filter: QualityFilter,
) -> Result<Stream> {
    get_streams(id, &stream_filter, &quality_filter)
        .await?
        .max_by(|lhs, rhs| stream_filter.max_stream(lhs, rhs))
        .ok_or(Error::NoStreams)
        .context("There are no streams, that match all your criteria")
}

async fn get_streams<'a>(
    id: IdBuf,
    stream_filter: &'a StreamFilter,
    quality_filter: &'a QualityFilter,
) -> Result<impl Iterator<Item=Stream> + 'a> {
    let streams = get_video(id)
        .await?
        .into_streams()
        .into_iter()
        .filter(move |stream| stream_filter.stream_matches(stream))
        .filter(move |stream| quality_filter.stream_matches(stream));
    Ok(streams)
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
