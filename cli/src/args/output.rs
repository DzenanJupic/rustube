use crate::output_format::OutputFormat;
use crate::output_level::OutputLevel;

#[derive(clap::Parser)]
pub struct OutputArgs {
    /// The format in which the information should be printed
    #[clap(
    short, long = "output",
    default_value = "yaml",
    possible_values = & ["debug", "pretty-debug", "json", "pretty-json", "yaml"]
    )]
    pub output_format: OutputFormat,
    /// The amount of information printed to the terminal
    /// To get more information, different levels can be combined, by separating them with a `|`.
    /// [possible values: url, general, video-track, audio-track, video]
    #[clap(short = 'l', long = "level", default_value = "url | general | video-track | audio-track")]
    pub output_level: OutputLevel,
}
