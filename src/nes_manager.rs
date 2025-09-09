use crate::{
    new_named_thread,
    prelude::*,
    thread_com::{ThreadCom, ThreadComError, ThreadMsg},
    NESEvent, NesmurEvent,
};
use crossbeam::channel::{self, Receiver, RecvError, Sender, TrySendError};
use nes::{
    cartridge::ROM, input_device::{NESDeviceButton, NESDeviceType}, ppu::renderer::{Renderer, RGB}, RcRef, NES, tools::NESAccess,
};
use std::{
    cell::Ref, thread::{self, JoinHandle}, time::{Duration, Instant}
};
use winit::event_loop::EventLoopProxy;

enum FrameSenderMsg {
    Data(Duration, Vec<RGB>),
    Exit,
}

struct FrameSender {
    thread_handle: Option<JoinHandle<()>>,
    pub tx: Sender<FrameSenderMsg>,
}

impl FrameSender {
    pub fn new(thread_com: &ThreadCom) -> Self {
        let (tx, rx): (Sender<FrameSenderMsg>, Receiver<FrameSenderMsg>) =
            channel::bounded::<FrameSenderMsg>(4);

        let thread_com: ThreadCom = thread_com.clone();
        let thread_handle: JoinHandle<()> = new_named_thread("nes-render", move || loop {
            let message: Result<FrameSenderMsg, RecvError> = rx.recv();
            match message {
                Ok(FrameSenderMsg::Data(frametime, pixels)) => {
                    // trace!("New frame data to send");
                    let result: Result<(), ThreadComError> = thread_com.await_send(
                        "nes-handle",
                        ThreadMsg::NewFrame(frametime, pixels),
                        Some(32),
                    );

                    if let Err(err) = result {
                        error!("Failed to send ThreadMsg::NewFrame: {:?}", err);
                    }
                }

                Ok(FrameSenderMsg::Exit) => {
                    trace!("Terminating thread...");
                    break;
                }

                Err(_) => error!("FrameSender Channel was dropped!"),
            };
        })
        .unwrap();

        FrameSender {
            thread_handle: Some(thread_handle),
            tx,
        }
    }
}

impl Drop for FrameSender {
    fn drop(&mut self) {
        if thread::panicking() {
            debug!(
                "The '{}' thread is panicking! Destroying 'nes-render' thread...",
                thread::current().name().unwrap_or("<unnamed>")
            );
            let result: Result<(), channel::SendError<FrameSenderMsg>> =
                self.tx.send(FrameSenderMsg::Exit);
            if let Err(_) = result {
                panic!("Failed to send Exit message to 'nes-render' thread! FrameSender's thread channel was disconnected!");
            }
            self.thread_handle.take().unwrap().join().unwrap();
        }
    }
}

enum NESMsg {
    Pause,
    Resume,
    Step(usize),
    Exit,
    ConnectDevice(u8, NESDeviceType),
    UpdateDeviceButton(u8, Box<dyn NESDeviceButton>, bool),
}

impl std::fmt::Debug for NESMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NESMsg::Pause => write!(f, "Pause"),
            NESMsg::Resume => write!(f, "Resume"),
            NESMsg::Step(steps) => write!(f, "Step({})", steps),
            NESMsg::Exit => write!(f, "Exit"),
            NESMsg::ConnectDevice(port, device_type) => write!(f, "ConnectDevice({}, {:?})", port, device_type),
            NESMsg::UpdateDeviceButton(port, device_button, pressed) => write!(f, "DeviceButtonPress({}, {:?}, {})", port, device_button.get_button_type_string(), pressed),
        }
    }
}

impl Clone for NESMsg {
    fn clone(&self) -> Self {
        match self {
            NESMsg::Pause => NESMsg::Pause,
            NESMsg::Resume => NESMsg::Resume,
            NESMsg::Step(steps) => NESMsg::Step(*steps),
            NESMsg::Exit => NESMsg::Exit,
            NESMsg::ConnectDevice(port, device_type) => NESMsg::ConnectDevice(*port, *device_type),
            NESMsg::UpdateDeviceButton(port, device_button, pressed) => NESMsg::UpdateDeviceButton(*port, device_button.box_clone(), *pressed),
        }
    }
}

struct NESMessenger {
    thread_handle: Option<JoinHandle<()>>,
    pub tx: Sender<NESMsg>,
}

impl NESMessenger {
    fn new(thread_com: &ThreadCom) -> Self {
        let (tx, rx): (Sender<NESMsg>, Receiver<NESMsg>) = channel::unbounded::<NESMsg>();

        let thread_com: ThreadCom = thread_com.clone();
        let thread_handle: JoinHandle<()> = new_named_thread("nes-messenger", move || {
            fn send_msg(thread_com: &ThreadCom, msg: ThreadMsg) {
                let result: Result<(), ThreadComError> =
                    thread_com.await_send("nes", msg.clone(), None);
                if let Err(err) = result {
                    error!("Failed to send ThreadMsg::{:?}: {:?}", msg, err);
                }
            }

            loop {
                let message: Result<NESMsg, RecvError> = rx.recv();
                match message {
                    Ok(message) => match message {
                        NESMsg::Pause => send_msg(&thread_com, ThreadMsg::Pause),
                        NESMsg::Resume => send_msg(&thread_com, ThreadMsg::Resume),
                        NESMsg::Step(steps) => send_msg(&thread_com, ThreadMsg::Step(steps)),
                        NESMsg::Exit => {
                            trace!("Terminating thread...");
                            break;
                        }
                        NESMsg::ConnectDevice(port, device_type) => send_msg(&thread_com, ThreadMsg::ConnectDevice(port, device_type)),
                        NESMsg::UpdateDeviceButton(port, device_button, pressed) => send_msg(&thread_com, ThreadMsg::UpdateDeviceButton(port, device_button, pressed)),
                    },
                    Err(_) => error!("NESMessenger Channel was dropped!"),
                }
            }
        })
        .unwrap();

        NESMessenger {
            thread_handle: Some(thread_handle),
            tx,
        }
    }
}

impl Drop for NESMessenger {
    fn drop(&mut self) {
        if thread::panicking() {
            debug!(
                "The '{}' thread is panicking! Destroying 'nes-messenger' thread...",
                thread::current().name().unwrap_or("<unnamed>")
            );
            let result: Result<(), channel::SendError<NESMsg>> = self.tx.send(NESMsg::Exit);
            if let Err(_) = result {
                panic!("Failed to send Exit message to 'nes-messenger' thread! NESMessenger's thread channel was disconnected!");
            }
            self.thread_handle.take().unwrap().join().unwrap();
        }
    }
}

pub struct NESManager {
    nes_thread: Option<JoinHandle<()>>,
    thread_com: ThreadCom,
    event_loop_proxy: EventLoopProxy<NesmurEvent>,
    nes_messenger: Option<NESMessenger>,
    pub framerate: f32,
    pub frametime: f32,
    frametimes: Vec<f32>,
    frametimes_index: usize,
}

impl NESManager {
    pub fn new(event_loop_proxy: &EventLoopProxy<NesmurEvent>) -> Self {
        NESManager {
            nes_thread: None,
            thread_com: ThreadCom::new(Some(10)),
            event_loop_proxy: event_loop_proxy.clone(),
            nes_messenger: None,
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
        self.nes_messenger = Some(NESMessenger::new(&self.thread_com));
        let thread_com: ThreadCom = self.thread_com.clone();

        self.nes_thread = Some(new_named_thread("nes", move || {
            let frame_sender_obj: FrameSender = FrameSender::new(&thread_com);
            let frame_sender: Sender<FrameSenderMsg> = frame_sender_obj.tx.clone();

            let rom_bytes: Vec<u8> = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/AccuracyCoin.nes")).unwrap();
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
                    Err(TrySendError::Full(_)) => warn!("FrameSender TX channel was full when trying to send new frame data!"),
                    Err(TrySendError::Disconnected(_)) => error!("FrameSender channel was disconnected when trying to send new frame data!"),
                };
            });

            let mut paused: bool = false;
            let mut stepping: bool = false;
            let mut steps_left: usize = 0;
            'nes_loop: loop {
                if !thread_com.is_rx_empty("nes") {
                    let messages: Vec<ThreadMsg> = thread_com.get_waiting_messages("nes");
                    // debug!("New messages for NES to handle: {}", messages.len());
                    for message in messages.iter() {
                        match message {
                            ThreadMsg::Stop => {
                                trace!("Stopped");
                                nes.cpu.running = false;
                                break 'nes_loop;
                            }
                            ThreadMsg::Pause => {
                                trace!("Paused");
                                paused = true;
                            }
                            ThreadMsg::Resume => {
                                trace!("Resumed");
                                paused = false;
                            }
                            ThreadMsg::Step(steps) => {
                                stepping = true;
                                steps_left += steps;
                                trace!("Stepping {}(+{}) steps", steps_left, steps);
                            }
                            ThreadMsg::ConnectDevice(port, device_type) => {
                                nes.connect_input_device(*port, *device_type);
                                trace!("Connected {:?} to port {}", device_type, port);
                            }
                            ThreadMsg::UpdateDeviceButton(port, device_button, pressed) => {
                                if *port == 2 && nes.device2.is_some() {
                                    nes.device2_mut().set_button_pressed_status(device_button.box_clone(), *pressed);
                                } else if nes.device1.is_some() {
                                    nes.device1_mut().set_button_pressed_status(device_button.box_clone(), *pressed);
                                }
                            }
                            _ => error!("NES received a '{:?}' message, which it cannot proccess. Ignoring message", message),
                        };
                    }
                }

                if !paused || stepping {
                    let nes_running: bool = nes.step(|_| {});
                    if !nes_running {
                        error!("The NES stopped on its own!");
                        break 'nes_loop;
                    }
                    if stepping {
                        if steps_left != 0 {
                            steps_left -= 1;
                        } else {
                            trace!("Finished stepping");
                            stepping = false;
                            let result: Result<(), ThreadComError> = thread_com.await_send("nes-handle", ThreadMsg::SteppingFinished, None);
                            if let Err(err) = result {
                                error!("Failed to send ThreadMsg::SteppingFinished message to 'nes-handle'! - {:?}", err);
                            }
                        }
                    }
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

        debug!("Stopping NES...");
        match self.thread_com.await_send("nes", ThreadMsg::Stop, None) {
            Ok(_) => {
                // debug!("Sent STOP message to NES");
            }
            Err(ThreadComError::Disconnected) => {
                panic!("ThreadCom channel was disconnected before the NES could be stopped!");
            }
            _ => panic!("This shouldn't happen!"),
        };
        if let Err(err) = self.nes_thread.take().unwrap().join() {
            error!("The 'nes' thread panicked: {:?}", err);
        }

        let mut nes_messenger: NESMessenger = self.nes_messenger.take().unwrap();
        let result: Result<(), channel::SendError<NESMsg>> = nes_messenger.tx.send(NESMsg::Exit);
        if let Err(_) = result {
            panic!("Failed to send Exit message to 'nes-messenger' thread! NESMessenger's thread channel was disconnected!");
        }
        if let Err(err) = nes_messenger.thread_handle.take().unwrap().join() {
            error!("The 'nes-messenger' thread panicked: {:?}", err);
        }
    }

    pub fn handle_nes_messages(&mut self) {
        if self.nes_thread.is_none() || self.thread_com.is_rx_empty("nes-handle") {
            return;
        }

        let messages: Vec<ThreadMsg> = self.thread_com.get_waiting_messages("nes-handle");
        for message in messages.iter() {
            match message {
                ThreadMsg::NewFrame(frametime, pixels) => {
                    // debug!("New frame data received");
                    self.event_loop_proxy.send_event(NesmurEvent::NES(NESEvent::NewFrame(pixels.to_owned()))).unwrap();

                    let frametime: f32 = ((*frametime).as_micros() as f64 / 1000.0) as f32;
                    if self.frametimes.len() != self.frametimes.capacity() {
                        self.frametimes.push(frametime);
                    } else {
                        self.frametimes[self.frametimes_index] = frametime;
                    }
                    self.frametimes_index = (self.frametimes_index + 1) % self.frametimes.capacity();
                    self.frametime = self.frametimes.iter().sum::<f32>() / self.frametimes.len() as f32;
                    self.framerate = 1000.0 / self.frametime;
                }

                ThreadMsg::SteppingFinished => {
                    self.event_loop_proxy.send_event(NesmurEvent::NES(NESEvent::SteppingFinished)).unwrap();
                }

                _ => error!("NESManager received a '{:?}' message, which it cannot proccess. Ignoring message", message),
            }
        }
    }

    fn send_nes_message(&self, msg: NESMsg) {
        if self.nes_thread.is_none() || self.nes_messenger.is_none() {
            return;
        }

        let nes_messenger: &NESMessenger = self.nes_messenger.as_ref().unwrap();
        let result: Result<(), TrySendError<NESMsg>> = nes_messenger.tx.try_send(msg.clone());
        match result {
            Ok(_) => {}
            Err(TrySendError::Full(_)) => {
                warn!(
                    "NESMessenger TX channel was full when trying to send {:?} message!",
                    msg
                )
            }
            Err(TrySendError::Disconnected(_)) => {
                error!(
                    "NESMessenger channel was disconnected when trying to send {:?} message!",
                    msg
                )
            }
        };
    }

    pub fn pause(&self) {
        self.send_nes_message(NESMsg::Pause);
    }

    pub fn resume(&self) {
        self.send_nes_message(NESMsg::Resume);
    }

    pub fn step(&self, steps: usize) {
        self.send_nes_message(NESMsg::Step(steps));
    }

    pub fn connect_device(&self, port: u8, device_type: NESDeviceType) {
        self.send_nes_message(NESMsg::ConnectDevice(port, device_type));
    }

    pub fn update_device_button(&self, port: u8, device_button: Box<dyn NESDeviceButton>, pressed: bool) {
        self.send_nes_message(NESMsg::UpdateDeviceButton(port, device_button, pressed));
    }
}
