//! Handles command line parsing for the application.
//!
//! It defines the structure and functions necessary to interpret user input.

use crate::ENV_VARS;
use clap::{CommandFactory, Parser};
use std::sync::LazyLock;

static LONG_VERSION: LazyLock<String> = LazyLock::new(|| -> String {
    format!(
        "v{}\nAuthor(s): {}\nDescription: {}\nRepository: {}",
        clap::crate_version!(),
        clap::crate_authors!(", "),
        clap::crate_description!(),
        option_env!("CARGO_PKG_REPOSITORY").unwrap_or("{UNKNONW}")
    )
});
static SHORT_VERSION: LazyLock<String> = LazyLock::new(|| format!("v{}", clap::crate_version!()));

fn parse_verbose(s: &str) -> Result<u8, String> {
    let mut n: u8 = s.parse().unwrap();
    // If 'debug_assertions' are enabled, then force verbose to be at least '1'
    if cfg!(debug_assertions) {
        n = std::cmp::max(1, n);
    }
    Ok(n)
}

fn parse_debug_info(s: &str) -> Result<bool, String> {
    if s == "true" {
        Cli::print_debug_info();
        std::process::exit(0);
    }
    Ok(false)
}

#[derive(Parser, Debug)]
#[command(
    about, version = (*SHORT_VERSION).as_str(),
    long_version = (*LONG_VERSION).as_str(),
    propagate_version = true,
)]
pub struct Cli {
    /// Turns on verbose logging
    #[arg(
        short, long, required = false,
        action = clap::ArgAction::Count,
        value_parser = parse_verbose,
    )]
    pub verbose: u8,

    /// Print debug info about the host and binary
    #[arg(
        long, required = false,
        value_parser = parse_debug_info
    )]
    pub debug_info: bool,
}

impl Cli {
    /// Prints the short version message
    pub fn _print_version() {
        println!("{}", Self::command().render_version())
    }

    /// Prints the long version message
    pub fn _print_long_version() {
        println!("{}", Self::command().render_long_version())
    }

    /// Prints the help message
    pub fn _print_help() {
        println!("{}", Self::command().render_help())
    }

    /// Prints debug info about the host and binary
    fn print_debug_info() {
        let env_list: Vec<&str> = vec![
            "BIN_NAME",
            "CARGO_PKG_VERSION",
            "VERGEN_BUILD_TIMESTAMP",
            "VERGEN_CARGO_TARGET_TRIPLE",
            "VERGEN_GIT_BRANCH",
            "VERGEN_GIT_COMMIT_TIMESTAMP",
            "VERGEN_GIT_SHA",
            "VERGEN_RUSTC_CHANNEL",
            "VERGEN_RUSTC_SEMVER",
            "VERGEN_RUSTC_COMMIT_DATE",
            "VERGEN_RUSTC_COMMIT_HASH",
        ];
        let mut info_text: String = String::from("");

        for environ in env_list {
            let mut trunc_environ: String = environ.to_string();
            trunc_environ.replace_range(..environ.find("_").unwrap() + 1, "");

            let concat_string: String = match ENV_VARS.get(environ) {
                Some(value) => format!("{trunc_environ} = {value}\n"),
                None => {
                    if environ == "BIN_NAME" {
                        format!(
                            "BIN_NAME = {}\n",
                            match std::env::current_exe() {
                                Ok(value) =>
                                    value.file_name().unwrap().to_string_lossy().to_string(),
                                Err(_) => "UNKNOWN".to_string(),
                            }
                        )
                    } else {
                        format!("{trunc_environ} = NOT_SET\n")
                    }
                }
            };

            info_text.push_str(&concat_string);
        }

        println!("{}", info_text);
    }
}
