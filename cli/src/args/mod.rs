use clap::Clap;

pub use download::DownloadArgs;
pub use fetch::FetchArgs;
pub use logging::LoggingArgs;
use rustube::{Id, IdBuf, Result};
pub use stream_filter::StreamFilter;

mod download;
mod fetch;
mod logging;
mod stream_filter;

#[derive(Clap)]
#[clap(
version = "0.3.0-beta.1",
about = "\n\
A simple CLI for the rustube YouTube-downloader library.\n\
For documentation and more information about rustube or the rustube-cli checkout \
`https://github.com/DzenanJupic/rustube`.\n\n\
For help with certain sub-commands run `rustube <SUBCOMMAND> --help`. 
"
)]
pub enum Command {
    #[clap(about = "\
    Downloads a YouTube video\n\
    By default, the Stream with the best quality and both a video, and an audio track will be \
    downloaded. To specify other download behavior, have a look the the sub-command help.\
    ")]
    Download(DownloadArgs),
    #[clap(about = "\
    Fetches information about a YouTube video\n\
    Fetching information does not require the video to actually be downloadable. So this also works \
    when a video is, i.e., an offline stream. The downside is that you don't have access to the \
    stream  URLs, so there's no way for you to download the video using only the returned \
    information.
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
