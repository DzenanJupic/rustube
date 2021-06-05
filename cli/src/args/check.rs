use crate::args::{Identifier, LoggingArgs, StreamFilter};
use crate::args::output::OutputArgs;

#[derive(clap::Clap)]
pub struct CheckArgs {
    #[clap(flatten)]
    pub identifier: Identifier,
    #[clap(flatten)]
    pub stream_filter: StreamFilter,
    #[clap(flatten)]
    pub logging: LoggingArgs,
    #[clap(flatten)]
    pub output: OutputArgs,
}
