use clap::Clap;

pub use download::DownloadArgs;
pub use logging::LoggingArgs;
use rustube::{Id, IdBuf, Result};
pub use stream_filter::StreamFilter;

mod download;
mod logging;
mod stream_filter;

#[derive(Clap)]
#[clap(
version = "0.3.0-beta.1",
about = "\
A simple CLI for the rustube YouTube downloader.\n\
For more information about rustube or the rustube-cli checkout `https://github.com/DzenanJupic/rustube`.\
"
)]
pub enum Command {
    #[clap(about = "\
    Downloads a YouTube video\n\
    By default, the Stream with the best quality and both a video, and an audio track will be \
    downloaded\
    ")]
    Download(DownloadArgs),
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
