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
        #[inline(always)]
        fn format_msg(
            out: FormatCallback,
            level: impl std::fmt::Display,
            record: &Record,
            msg: &Arguments,
        ) {
            out.finish(format_args!(
                "{:<5} [{}:{}]: {}",
                level, record.target(), record.line().unwrap_or_default(), msg,
            ))
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

                    format_msg(
                        out,
                        COLORS.color(record.level()),
                        record,
                        message,
                    );
                }
            }
            ColorUsage::Never => {
                |out: FormatCallback, message: &Arguments, record: &Record| {
                    format_msg(
                        out,
                        record.level(),
                        record,
                        message,
                    );
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
