//! Handles command line parsing for the application.
//!
//! It defines the structure and functions necessary to interpret user input.

// Import custom Environment Variables generated at compile time
use crate::ENV_VARS;
use clap::{
    arg, crate_authors, crate_description, crate_name, crate_version, value_parser, ArgMatches,
    Command,
};
use std::{cmp, env, error::Error, process, sync::LazyLock};

static LONG_VERSION: LazyLock<String> = LazyLock::new(|| {
    format!(
        " v{}\nAuthor(s): {}\nDescription: {}\nRepository: {}",
        crate_version!(),
        crate_authors!(", "),
        crate_description!(),
        env!("CARGO_PKG_REPOSITORY")
    )
});

static SHORT_VERSION: LazyLock<String> = LazyLock::new(|| format!(" v{}", crate_version!(),));

#[derive(Debug)]
#[allow(dead_code)]
/// Struct to describe passed command-line arguments
pub struct Args {
    pub verbose: u8,
}

impl Args {
    /// Parses CLI arguments and options for the application
    pub fn parse() -> Result<Args, Box<dyn Error>> {
        // Possible arguments
        // -v   --verbose       Turns on verbose logging (Max level of 2)
        //     --debug-info     Prints out debug info about the binary
        let matches: ArgMatches = Self::command()
            .ignore_errors(true)
            .arg(
                arg!(
                    -v --verbose ... "Turns on verbose logging (Max level of 2)"
                )
                .value_parser(value_parser!(u8).range(0..=2)),
            )
            .arg(arg!(
                --"debug-info" "Prints out debug info about the binary"
            ))
            .get_matches();

        if matches.get_flag("debug-info") {
            println!("{}", Self::debug_info());
            process::exit(0);
        }

        let verbose: u8 = if cfg!(debug_assertions) {
            cmp::max(1, matches.get_count("verbose"))
        } else {
            matches.get_count("verbose")
        };

        // Returns parsed arguments
        Ok(Args { verbose })
    }

    /// Defines the base Command struct and its relevant information
    pub fn command() -> Command {
        // crate_name!() = env!("CARGO_PKG_NAME");
        // crate_version!() = env!("CARGO_PKG_VERSION");
        // crate_authors!(sep) = env!("CARGO_PKG_AUTHORS") + custom_separator;
        // crate_description!() = env!("CARGO_PKG_DESCRIPTION");

        let cmd: Command = Command::new(crate_name!())
            .author(crate_authors!(", "))
            .about(crate_description!())
            // .long_about(crate_description!())
            .version(SHORT_VERSION.as_str())
            .long_version(LONG_VERSION.as_str());
        cmd
    }

    /// Display version message.
    pub fn print_version() {
        println!("{}", Self::command().render_version());
    }

    /// Display help message.
    pub fn print_help() {
        println!("{}", Self::command().render_help())
    }

    /// Prints debug info about host and binary
    fn debug_info() -> String {
        let env_list: Vec<&str> = vec![
            "BIN_NAME",
            "CARGO_PKG_VERSION",
            "VERGEN_BUILD_TIMESTAMP",
            "VERGEN_GIT_SHA",
            "VERGEN_GIT_COMMIT_TIMESTAMP",
            "VERGEN_GIT_BRANCH",
            "VERGEN_CARGO_TARGET_TRIPLE",
            "VERGEN_RUSTC_CHANNEL",
            "VERGEN_RUSTC_COMMIT_DATE",
            "VERGEN_RUSTC_COMMIT_HASH",
            "VERGEN_RUSTC_SEMVER",
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
                            match env::current_exe() {
                                Ok(value) => value.display().to_string(),
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

        info_text
    }
}
