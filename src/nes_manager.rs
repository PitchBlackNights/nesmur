use crate::{
    new_named_thread,
    prelude::*,
    thread_com::{ThreadCom, ThreadComError, ThreadMsg},
};
use crossbeam::channel::{self, Receiver, RecvError, Sender, TrySendError};
use nes::{
    cartridge::ROM,
    ppu::renderer::{Renderer, RGB},
    RcRef, NES,
};
use std::{
    cell::Ref,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

enum FrameSenderMsg {
    Data(Duration, Vec<RGB>),
    Exit,
}

struct FrameSender {
    thread_handle: Option<JoinHandle<()>>,
}

impl FrameSender {
    pub fn new() -> Self {
        FrameSender {
            thread_handle: None,
        }
    }

    pub fn start_frame_sender(&mut self, thread_com: &ThreadCom) -> Sender<FrameSenderMsg> {
        let (tx, rx): (Sender<FrameSenderMsg>, Receiver<FrameSenderMsg>) =
            channel::bounded::<FrameSenderMsg>(4);

        let thread_com: ThreadCom = thread_com.clone();
        self.thread_handle = Some(new_named_thread("nes-render", move || loop {
            let message: Result<FrameSenderMsg, RecvError> = rx.recv();
            match message {
                Ok(FrameSenderMsg::Data(frametime, pixels)) => {
                    trace!("New frame data to send");
                    let result: Result<(), ThreadComError> =
                        thread_com.await_send(ThreadMsg::NewFrame(frametime, pixels), Some(32));

                    if let Err(err) = result {
                        error!("Failed to send ThreadMsg::NewFrame: {:?}", err);
                    }
                }

                Ok(FrameSenderMsg::Exit) => {
                    trace!("Terminating thread...");
                    break;
                }

                Err(_) => {
                    // error!("FrameSender Channel was dropped!");
                }
            };
        })
        .unwrap());

        tx
    }
}

impl Drop for FrameSender {
    fn drop(&mut self) {
        if thread::panicking() {
            error!("FrameSender's parent thread is panicking! Destroying 'nes-render' thread...");
            // DESTROY THREAD
        }
    }
}

pub struct NESManager {
    nes_thread: Option<JoinHandle<()>>,
    thread_com: ThreadCom,
    pub framerate: f32,
    pub frametime: f32,
    frametimes: Vec<Duration>,
    frametimes_index: usize,
}

impl NESManager {
    pub fn new() -> Self {
        NESManager {
            nes_thread: None,
            thread_com: ThreadCom::new(Some(10)),
            framerate: 0.0,
            frametime: 0.0,
            frametimes: Vec::with_capacity(120),
            frametimes_index: 0,
        }
    }

    pub fn start_nes(&mut self) {
        assert!(
            self.nes_thread.is_none(),
            "Ran `NESManager.start_nes()` when an NES instance is currently running!"
        );

        let thread_com: ThreadCom = self.thread_com.clone();

        self.nes_thread = Some(new_named_thread("nes", move || {
            let mut frame_sender_obj: FrameSender = FrameSender::new();
            let frame_sender: Sender<FrameSenderMsg> = frame_sender_obj.start_frame_sender(&thread_com);

            let rom_bytes: Vec<u8> = std::fs::read(concat!("CARGO_MANIFEST_DIR", "/smb.nes")).unwrap();
            let rom: ROM = ROM::new(&rom_bytes).unwrap();
            let mut nes: NES = NES::new(rom);
            let mut last_frame: Instant = Instant::now();

            let cb_frame_sender: Sender<FrameSenderMsg> = frame_sender.clone();
            nes.render_callback(move |renderer: RcRef<Renderer>, _, _| {
                let renderer: Ref<Renderer> = renderer.borrow();

                let now: Instant = Instant::now();
                let frametime: Duration = now - last_frame;
                last_frame = now;

                let result: Result<(), TrySendError<FrameSenderMsg>> = cb_frame_sender.try_send(FrameSenderMsg::Data(frametime, renderer.pixels.clone()));
                match result {
                    Ok(_) => {},
                    Err(TrySendError::Full(_)) => warn!("FrameSender TX channel is full when trying to send new frame data!"),
                    Err(TrySendError::Disconnected(_)) => error!("FrameSender channel is disconnected when trying to send new frame data!"),
                };
            });

            'nes_loop: loop {
                if !thread_com.is_rx_empty() {
                    let messages: Vec<ThreadMsg> = thread_com.get_waiting_messages();
                    for message in messages.iter() {
                        match message {
                            ThreadMsg::Stop => {
                                nes.cpu.running = false;
                                break 'nes_loop;
                            },

                            _ => error!("NES received a '{:?}' message, which it cannot proccess. Ignoring message", message),
                        }
                    }
                }

                let running: bool = nes.step(|_| {});
                if !running {
                    warn!("The NES stopped on its own!");
                    break;
                }
            }

            frame_sender.send(FrameSenderMsg::Exit).expect("FrameSender channel was disconnected before it was closed properly!");
            trace!("Terminating thread...");
        }).unwrap());
    }

    pub fn stop_nes(&mut self) {
        if self.nes_thread.is_none() {
            return;
        }

        let thread_com: ThreadCom = self.thread_com.clone();
        debug!("Stopping NES...");

        new_named_thread("nes-stop", move || {
            match thread_com.await_send(ThreadMsg::Stop, None) {
                Ok(_) => debug!("Sent STOP message to NES"),
                Err(ThreadComError::Disconnected) => {
                    panic!("ThreadCom channel was disconnected before the NES could be stopped!")
                }
                _ => panic!("This shouldn't happen!"),
            };
        })
        .unwrap();

        if let Some(handle) = self.nes_thread.take() {
            handle.join().unwrap();
        }
    }

    pub fn handle_nes_messages(&mut self) {
        if self.nes_thread.is_none() || self.thread_com.is_rx_empty() {
            return;
        }

        let messages: Vec<ThreadMsg> = self.thread_com.get_waiting_messages();
        for message in messages.iter() {
            match message {
                ThreadMsg::NewFrame(frametime, _) => {
                    debug!("New frame data received");
                    self.frametimes[self.frametimes_index] = *frametime;
                    self.frametimes_index = (self.frametimes_index + 1) % self.frametimes.capacity();
                    self.frametime = (self.frametimes.iter().map(|t| t.as_micros() as f64 / 1000.0).sum::<f64>() / self.frametimes.len() as f64) as f32;
                    self.framerate = 1000.0 / self.frametime;
                }
                _ => error!("NESManager received a '{:?}' message, which it cannot proccess. Ignoring message", message),
            }
        }
    }
}
