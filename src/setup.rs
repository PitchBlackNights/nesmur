//! Handles setup and initialization routines for mds-installer
//!
//! This module is responsible for configuring the environment, parsing command-line arguments,
//! and initializing the logging system used throughout the application.

use crate::{cli::Cli, prelude::*};
use clap::Parser;
use colored::*;
use log::{Level, LevelFilter, Record};
use std::{
    io::Write,
    thread::{self, Thread},
};

// TODO: Setup better filtering system
static VERBOSE_FILTER: std::sync::LazyLock<Vec<Vec<&'static str>>> =
    std::sync::LazyLock::new(|| {
        vec![
            // verbose = 0
            vec![],
        ]
    });

/// Sets up the program by:
/// 1. Parsing command arguments
/// 2. Initialize the logger
pub fn setup_logger_and_cli() -> Cli {
    let cli: Cli = Cli::parse();
    init_logger(cli.verbose);
    trace!("Logger was enabled successfully.");
    debug!("Passed Arguments: {:?}", cli);
    cli
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
            move |buf: &mut env_logger::fmt::Formatter,
                  record: &Record<'_>|
                  -> Result<(), std::io::Error> {
                let module_path: String = record.module_path().unwrap_or("UNKNOWN").to_string();
                let mut target: String = record.target().to_string();
                let level: String = record.level().to_string();

                let filter_str: String = if target != module_path {
                    format!("{}/{}/{}", target, module_path, level)
                } else {
                    format!("{}/{}", module_path, level)
                };
                if record.level() >= Level::Info
                    && VERBOSE_FILTER[(verbose_level as usize).min(VERBOSE_FILTER.len() - 1)]
                        .iter()
                        .any(|s: &&str| filter_str.starts_with(s))
                {
                    return Ok(());
                }

                if verbose_level >= 2 && target != module_path {
                    target = format!("{}/{}", target, module_path)
                }
                let current_thread: Thread = thread::current();
                let thread_name: &str = current_thread.name().unwrap_or("<unnamed>");

                // Log output format
                let log_output: String = format!(
                    "[{}] [{}/{}]: {}",
                    thread_name,
                    target,
                    level,
                    record.args()
                );

                // Apply severity color to the whole log line
                let colored_log: ColoredString = match record.level() {
                    log::Level::Error => log_output.bright_red().bold(),
                    log::Level::Warn => log_output.bright_yellow(),
                    log::Level::Info => log_output.normal(),
                    log::Level::Debug => log_output.bright_blue(),
                    log::Level::Trace => log_output.bright_black(),
                };

                writeln!(buf, "{}", colored_log)
            },
        )
        .filter(None, log_level_filter)
        .init();
}
