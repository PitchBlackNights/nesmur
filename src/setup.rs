//! Handles setup and initialization routines for nesmur
//!
//! This module is responsible for configuring the environment, parsing command-line arguments,
//! and initializing the logging system used throughout the application.

use crate::{cli_parser::Args, prelude::*};
use chrono::format::{DelayedFormat, StrftimeItems};
use colored::*;
use log::{LevelFilter, Record};
use std::{
    io::Write,
    thread::{self, Thread},
};

/// Sets up the program by:
/// 1. Parsing command arguments
/// 2. Initialize the logger
pub fn setup_logger_and_args() -> Args {
    let _args: Args = match Args::parse() {
        Ok(arguments) => arguments,
        Err(error) => {
            println!("Error: {error}");
            Args::print_help();

            #[cfg(windows)]
            std::process::exit(160);
            #[cfg(unix)]
            std::process::exit(22);
        }
    };

    init_logger(_args.verbose);
    trace!("Logger was enabled successfully.");
    debug!("Passed Arguments: {_args:?}");
    _args
}

/// Initializes the logger
fn init_logger(verbose_level: u8) {
    let mut builder: env_logger::Builder = env_logger::Builder::new();

    // Determine log level based on build mode and verbosity flag
    let log_level_filter: LevelFilter = if verbose_level == 1 {
        LevelFilter::Debug
    } else if verbose_level >= 2 {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };

    builder
        .format(
            move |buf: &mut env_logger::fmt::Formatter, record: &Record<'_>| {
                let timestamp: DelayedFormat<StrftimeItems<'_>> =
                    chrono::Local::now().format("%H:%M:%S.%3f");

                let mut target: String = record.target().to_string();
                let upto: usize = target
                    .char_indices()
                    .map(|(i, _)| i)
                    .nth(
                        target
                            .chars()
                            .position(|c: char| c == ':')
                            .unwrap_or(target.len()),
                    )
                    .unwrap_or(target.len());
                target.truncate(upto);

                let module_path: String = record.module_path().unwrap_or("UNKNOWN").to_string();
                let level: String = record.level().to_string();

                let current_thread: Thread = thread::current();
                let thread_name: &str = current_thread.name().unwrap_or("<unnamed>");

                // Log output format
                let log_output: String = if verbose_level >= 2 {
                    format!(
                        "[{}] [{}/{}] [{}]: {}",
                        timestamp,
                        module_path,
                        level,
                        thread_name,
                        record.args()
                    )
                } else {
                    format!(
                        "[{}] [{}/{}] [{}]: {}",
                        timestamp,
                        target,
                        level,
                        thread_name,
                        record.args()
                    )
                };

                // Apply severity color to the whole log line
                let colored_log: ColoredString = match record.level() {
                    log::Level::Error => log_output.bright_red().bold(),
                    log::Level::Warn => log_output.bright_yellow(),
                    log::Level::Info => log_output.normal(),
                    log::Level::Debug => log_output.bright_blue(),
                    log::Level::Trace => log_output.bright_black(),
                };

                writeln!(buf, "{colored_log}")
            },
        )
        .filter(None, log_level_filter)
        .init();
}
