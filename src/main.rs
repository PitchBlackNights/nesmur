use nesmur::{cli_parser::Args, prelude::*, setup};

fn main() {
    let _args: Args = setup::setup_logger_and_args();
    info!("Starting Emulator...");
}
