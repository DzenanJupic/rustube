use std::path::PathBuf;

use clap::Parser;

use crate::args::Identifier;
use crate::args::logging::LoggingArgs;
use crate::args::output::OutputArgs;
use crate::args::stream_filter::StreamFilter;

#[derive(Parser)]
pub struct DownloadArgs {
    #[clap(flatten)]
    pub identifier: Identifier,
    #[clap(flatten)]
    pub stream_filter: StreamFilter,
    #[clap(flatten)]
    pub logging: LoggingArgs,
    #[clap(flatten)]
    pub output: OutputArgs,

    /// Where to download the video to [default: .]
    #[clap(short, long)]
    pub dir: Option<PathBuf>,
    /// The filename of the video file [default: <VIDEO_ID>.mp4]
    /// If the file already exists, it will be removed, even if the download fails!
    #[clap(short, long)]
    pub filename: Option<PathBuf>,
}
