pub mod memory;
pub mod cpu;

pub mod cli_parser;
pub mod setup;
pub mod prelude;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
