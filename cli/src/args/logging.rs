use std::fmt::Arguments;

use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use fern::FormatCallback;
use log::{LevelFilter, Record};
use strum::EnumString;

#[derive(Parser)]
pub struct LoggingArgs {
    /// Sets the log-level of rustube [default: Info]
    /// (-v = Error, ..., -vvvvv = Trace)
    /// (other crates have log level Warn)
    #[clap(long, short, parse(from_occurrences))]
    verbose: u8,

    /// When to log coloredd
    #[clap(long, default_value = "always", possible_values = & ["always", "never"], value_name = "WHEN")]
    color: ColorUsage,

    /// Turn off logging for all crates
    #[clap(long, short, conflicts_with = "verbose")]
    quiet: bool,
}

impl LoggingArgs {
    pub fn init_logger(&self) {
        if self.quiet { return; }

        let formatter = self.log_msg_formatter();

        fern::Dispatch::new()
            .level(log::LevelFilter::Warn)
            .level_for("rustube", self.level_filter())
            .format(formatter)
            .chain(std::io::stdout())
            .apply()
            .expect("The global logger was already initialized");
    }

    fn log_msg_formatter(&self) -> fn(FormatCallback, &Arguments, &Record) {
        macro_rules! msg_formatter {
            ($out:ident, $message:ident, $log_level:expr) => {
                $out.finish(format_args!(
                    "{:<5}: {}",
                    $log_level,
                    $message
                ))
            };
        }

        match self.color {
            ColorUsage::Always => {
                |out: FormatCallback, message: &Arguments, record: &Record| {
                    static COLORS: ColoredLevelConfig = ColoredLevelConfig {
                        error: Color::Red,
                        warn: Color::Yellow,
                        info: Color::Green,
                        debug: Color::BrightBlue,
                        trace: Color::White,
                    };

                    msg_formatter!(out, message, COLORS.color(record.level()));
                }
            }
            ColorUsage::Never => {
                |out: FormatCallback, message: &Arguments, record: &Record| {
                    msg_formatter!(out, message, record.level());
                }
            }
        }
    }

    fn level_filter(&self) -> log::LevelFilter {
        match self.verbose {
            1 => LevelFilter::Error,
            2 => LevelFilter::Warn,
            0 | 3 => LevelFilter::Info,
            4 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    }
}

#[derive(Parser, EnumString)]
#[strum(serialize_all = "kebab-case")]
enum ColorUsage {
    Always,
    Never,
}
