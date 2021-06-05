use std::path::PathBuf;

use clap::Clap;

use crate::args::Identifier;
use crate::args::logging::LoggingArgs;
use crate::args::stream_filter::StreamFilter;

#[derive(Clap)]
pub struct DownloadArgs {
    #[clap(flatten)]
    pub identifier: Identifier,
    #[clap(flatten)]
    pub stream_filter: StreamFilter,
    #[clap(flatten)]
    pub logging: LoggingArgs,

    #[clap(
    short,
    long,
    about = "Where to download the video to [default: .]"
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
