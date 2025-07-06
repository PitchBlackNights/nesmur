pub mod cli_parser;
pub mod setup;
pub mod prelude {
    #[allow(unused_imports)]
    pub use log::{debug, error, info, trace, warn};
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
