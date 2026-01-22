//! Main library file for nesmur

#![feature(map_try_insert)]

pub mod cli;
pub mod setup;
pub mod prelude {
    #[allow(unused_imports)]
    pub use log::{debug, error, info, trace, warn};
}
pub mod app;
#[cfg(debug_assertions)]
pub mod debug;
pub mod events;
pub mod input;
pub mod logging;
pub mod ui;
pub mod widgets;

mod temp;
pub use self::temp::*;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub const PERSISTENT_DATA_PATH: &str = "./app_data.ron";
pub const INITIAL_SIZE_HEIGHT: f32 = 600.0;
pub const INITIAL_SIZE_WIDTH: f32 = 592.0;
