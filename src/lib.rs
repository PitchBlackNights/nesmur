use std::thread::{Builder, JoinHandle};

use nes::ppu::renderer::RGB;

pub mod cli_parser;
pub mod setup;
pub mod prelude {
    #[allow(unused_imports)]
    pub use log::{debug, error, info, trace, warn};
}
pub mod nes_manager;
pub mod shared_ctx;
pub mod theme;
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

pub enum NESEvent {
    Start,
    Stop,
    Pause,
    Resume,
    Step,
    NewFrame(Vec<RGB>),
    SteppingFinished,
}

impl std::fmt::Debug for NESEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NESEvent::NewFrame(pixels) => {
                write!(f, "NewFrame([RGB(u8, u8, u8); {}])", pixels.len())
            }
            NESEvent::Start => write!(f, "Start"),
            NESEvent::Stop => write!(f, "Stop"),
            NESEvent::Pause => write!(f, "Pause"),
            NESEvent::Resume => write!(f, "Resume"),
            NESEvent::Step => write!(f, "Step"),
            NESEvent::SteppingFinished => write!(f, "SteppingFinished"),
        }
    }
}

pub fn new_named_thread<F, T>(name: &str, func: F) -> std::io::Result<JoinHandle<T>>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    Builder::new().name(name.to_string()).spawn(func)
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
