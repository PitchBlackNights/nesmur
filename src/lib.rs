//! Main library file for nesmur

pub mod cli;
pub mod setup;
pub mod prelude {
    #[allow(unused_imports)]
    pub use log::{debug, error, info, trace, warn};
}
pub mod app;
pub mod input;
pub mod logging;
pub mod ui;
pub mod widgets;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub const PERSISTENT_DATA_PATH: &str = "./app_data.ron";
