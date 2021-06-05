use crate::output_format::OutputFormat;

#[derive(clap::Clap)]
pub struct OutputArgs {
    #[clap(
    short, long = "output",
    default_value = "yaml",
    possible_values = & ["debug", "pretty-debug", "json", "pretty-json", "yaml"],
    about = "The format in which the information should be printed"
    )]
    pub output_format: OutputFormat,
}
