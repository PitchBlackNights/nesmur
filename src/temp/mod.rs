pub mod nes_manager;
pub mod thread_com;

use nes::ppu::renderer::RGB;
use std::{
    path::PathBuf,
    thread::{Builder, JoinHandle},
};

#[derive(Debug, PartialEq, Eq)]
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
    Start(PathBuf),
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
            NESEvent::Start(path) => write!(f, "Start({:?})", path),
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
