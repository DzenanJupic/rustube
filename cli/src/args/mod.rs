use clap::Clap;

pub use check::CheckArgs;
pub use download::DownloadArgs;
pub use fetch::FetchArgs;
pub use logging::LoggingArgs;
use rustube::{Id, IdBuf, Result};
pub use stream_filter::StreamFilter;

mod check;
mod download;
mod fetch;
mod logging;
mod output;
mod stream_filter;

#[derive(Clap)]
#[clap(
version = "0.3.1",
about = "\n\
A simple CLI for the rustube YouTube-downloader library.\n\
For documentation and more information about rustube or the rustube-cli checkout \
`https://github.com/DzenanJupic/rustube`.\n\n\
For help with certain subcommands run `rustube <SUBCOMMAND> --help`. 
"
)]
pub enum Command {
    #[clap(about = "\
    Checks if a video can be downloaded and fetches information about it\n\
    This command is similar to fetch, in the way that it also fetches information about a video, \
    but, other then fetch, will also decrypt all stream URLs. Therefore you can use the returned \
    URLs for downloading the video. This of course means that the video has to be downloadable.\n\
    By default this command will check for any streams that contain a video and an audio track. \
    To specify other behavior, like checking for a stream with a particular quality, have a look at \
    the subcommand help.\
    ")]
    Check(CheckArgs),
    #[clap(about = "\
    Downloads a YouTube video\n\
    By default, the Stream with the best quality and both a video, and an audio track will be \
    downloaded. To specify other download behavior, have a look the the subcommand help.\
    ")]
    Download(DownloadArgs),
    #[clap(about = "\
    Fetches information about a YouTube video\n\
    Fetching information does not require the video to actually be downloadable. So this also works \
    when a video is, i.e., an offline stream. The downside is that you often don't have access to \
    the stream URLs. Some videos come with pre-decrypted urls, in which case you can also use these \
    to download the video, but if the video url is encrypted there's no way for you to download the \
    video using only the returned information. To get decrypted URLs, have a look at `check`.\n\
    For most use cases it's recommended to use `check` instead, since it gives you both more \
    control and more information.\
    ")]
    Fetch(FetchArgs),
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
