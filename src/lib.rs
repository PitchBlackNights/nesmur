pub mod cli_parser;
pub mod setup;
pub mod prelude {
    #[allow(unused_imports)]
    pub use log::{debug, error, info, trace, warn};
}
pub mod shared_ctx;
pub mod thread_com;
pub mod ui;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub const INITIAL_WINDOW_WIDTH: u32 = 800;
pub const INITIAL_WINDOW_HEIGHT: u32 = 700;

#[derive(Debug)]
pub enum NESState {
    Stopped,
    Running,
    Paused,
    Stepping,
}

#[derive(Debug)]
pub enum NesmurEvent {
    NES(NESEvent),
}

#[derive(Debug)]
pub enum NESEvent {
    Start,
    Stop,
    Pause,
    Resume,
    Step,
}

#[macro_export]
macro_rules! gl_error {
    ($gl:expr) => {
        let gl: &glow::Context = &$gl;
        // It's not unused stupid compiler
        #[allow(unused_unsafe)]
        let gl_error: u32 = unsafe { gl.get_error() };
        if gl_error != 0 {
            log::log!(
                log::Level::Error,
                "[====OpenGL====] GL_ERROR: {:#X} -- {}:{}",
                gl_error,
                file!(),
                line!()
            );
        }
    };
}
