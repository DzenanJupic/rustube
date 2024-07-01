use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

use args::DownloadArgs;
use args::StreamFilter;
use rustube::{Error, Id, IdBuf, Stream, Video, VideoFetcher, VideoInfo};
use rustube::Callback;

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
        eprintln!("\
            If the error is caused by a change to the YouTube API, it would be great if you could \
            report this. Common indicators of an API change are:\n\
            1. repeated HTTP 403 status\n\
            2. unexpected response errors\n\
            3. deserialization errors\n\
            There's a predefined issue template in our repo: https://github.com/DzenanJupic/rustube/issues/new?assignees=&labels=youtube-api-changed&template=youtube_api_changed.yml\
        ");
    }

    res
}

async fn check(args: CheckArgs) -> Result<()> {
    args.logging.init_logger();

    let id = args.identifier.id()?;
    let (video_info, streams) = get_streams(id, &args.stream_filter).await?;
    let video_serializer = VideoSerializer::new(video_info, streams, args.output.output_level);

    let output = args
        .output
        .output_format
        .serialize_output(&video_serializer)?;
    println!("{output}");

    Ok(())
}

async fn download(args: DownloadArgs) -> Result<()> {
    args.logging.init_logger();

    let id = args.identifier.id()?;
    let (video_info, stream) = get_stream(id.as_owned(), args.stream_filter).await?;
    let download_path = download_path(args.filename, stream.mime.subtype().as_str(), args.dir, id);

    let mut pb = args.logging.init_progress_bar(stream.content_length().await?);
    let callback = Callback::new()
        .connect_on_progress_closure(|cargs| {
            // update progress bar
            pb.set(cargs.current_chunk as u64);
        });

    stream
        .download_to_with_callback(&download_path, callback)
        .await?;
    pb.finish_println(&format!("Finished downloading video to {download_path:?}\n"));

    let video_serializer = VideoSerializer::new(
        video_info,
        std::iter::once(stream),
        args.output.output_level,
    );
    let output = args.output.output_format.serialize_output(&video_serializer).unwrap();
    println!("{output}");

    Ok(())
}

async fn fetch(args: FetchArgs) -> Result<()> {
    args.logging.init_logger();

    let id = args.identifier.id()?;
    let video_info = rustube::VideoFetcher::from_id(id)?.fetch_info().await?;

    let output = args.output.output_format.serialize_output(&video_info)?;
    println!("{output}");

    Ok(())
}

async fn get_stream(id: IdBuf, stream_filter: StreamFilter) -> Result<(VideoInfo, Stream)> {
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
    let (video_info, streams) = get_video(id).await?.into_parts();

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
        .await
        .context("Could not descramble the video information")
}

pub fn download_path(
    filename: Option<PathBuf>,
    extension: &str,
    dir: Option<PathBuf>,
    video_id: Id<'_>,
) -> PathBuf {
    let filename =
        filename.unwrap_or_else(|| format!("{}.{}", video_id.as_str(), extension).into());

    let mut path = dir.unwrap_or_else(PathBuf::new);

    path.push(filename);
    path
}
