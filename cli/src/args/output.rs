use crate::output_format::OutputFormat;
use crate::output_level::OutputLevel;

#[derive(clap::Parser)]
pub struct OutputArgs {
    #[clap(
    short, long = "output",
    default_value = "yaml",
    possible_values = & ["debug", "pretty-debug", "json", "pretty-json", "yaml"],
    about = "The format in which the information should be printed"
    )]
    pub output_format: OutputFormat,
    #[clap(
    short = 'l', long = "level",
    default_value = "url | general | video-track | audio-track",
    about = "\
    The amount of information printed to the terminal\n\
    To get more information, different levels can be combined, by separating them with a `|`. \
    [possible values: url, general, video-track, audio-track, video]\
    "
    )]
    pub output_level: OutputLevel,
}
