use crate::args::{Identifier, LoggingArgs};

#[derive(clap::Clap)]
pub struct FetchArgs {
    #[clap(flatten)]
    pub identifier: Identifier,
    #[clap(flatten)]
    pub logging: LoggingArgs,
}
