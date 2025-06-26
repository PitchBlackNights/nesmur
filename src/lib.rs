pub mod apu;
pub mod bus;
pub mod cpu;
pub mod ppu;

pub mod cli_parser;
pub mod setup;
pub mod prelude {
    pub use log::{debug, error, info, trace, warn};
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
