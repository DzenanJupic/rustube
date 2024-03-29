use crate::args::{Identifier, LoggingArgs};
use crate::args::output::OutputArgs;

#[derive(clap::Parser)]
pub struct FetchArgs {
    #[clap(flatten)]
    pub identifier: Identifier,
    #[clap(flatten)]
    pub logging: LoggingArgs,
    #[clap(flatten)]
    pub output: OutputArgs,
}
