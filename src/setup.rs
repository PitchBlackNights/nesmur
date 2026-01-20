//! Handles setup and initialization routines
//!
//! This module is responsible for:
//! - Configuring the environment
//! - Parsing command-line arguments
//! - Initializing the logging system

use crate::{cli::Cli, logging, prelude::*};
use clap::Parser;

/// Sets up the program by:
/// 1. Parsing command arguments
/// 2. Initialize the logger
pub fn initial_setup() -> Cli {
    let cli: Cli = Cli::parse();
    logging::init_logger(cli.verbose);
    trace!("Logger was enabled successfully.");
    debug!("Passed Arguments: {:?}", cli);
    cli
}
