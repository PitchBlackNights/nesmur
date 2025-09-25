//! Thread communication utilities for nesmur
//!
//! This module provides the `ThreadCom` struct and associated types and traits
//! for communication between threads in nesmur. It is used to send and receive
//! messages between different parts of the emulator, such as the CPU, PPU,
//! and input devices. The communication is done through channels, and the module
//! provides both synchronous and asynchronous message sending and receiving methods.
//!
//! The module is divided into three main parts:
//! 1. Type and trait definitions: Defines the `ThreadComError` enum for
//!    representing errors that can occur during thread communication, and the
//!    `ThreadMsg` enum for representing the different types of messages that
//!    can be sent between threads.
//! 2. `ThreadMsgObj` struct: A wrapper around the `ThreadMsg` enum that also
//!    includes the destination of the message. This is used as the payload for
//!    the channels used in the `ThreadCom` struct.
//! 3. `ThreadCom` struct: The main struct for thread communication. It
//!    includes the sender and receiver ends of the channel, as well as a
//!    pending message queue. It provides methods for sending and receiving
//!    messages, both blocking and non-blocking, and for checking the
//!    status of the message queue.

use crossbeam::channel::{
    self, Receiver, RecvTimeoutError, SendTimeoutError, Sender, TryRecvError, TrySendError,
};
use nes::{
    input_device::{NESDeviceButton, NESDeviceType},
    ppu::renderer::RGB,
};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, MutexGuard},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub enum ThreadComError {
    Full,
    Empty,
    Timeout,
    Disconnected,
    Dropped,
    Uninitialized,
}

/// Defines thread messages
/// CURRENTLY ONLY USED FOR COMMUNICATION BETWEEN `NESManager` and the 'nes' thread
pub enum ThreadMsg {
    Pause,
    Resume,
    Step(usize),
    NewFrame(Duration, Vec<RGB>),
    Stop,
    SteppingFinished,
    ConnectDevice(u8, NESDeviceType),
    UpdateDeviceButton(u8, Box<dyn NESDeviceButton>, bool),
}

impl std::fmt::Debug for ThreadMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreadMsg::NewFrame(frametime, pixels) => {
                write!(
                    f,
                    "NewFrame({:.03}ms, [RGB(u8, u8, u8); {}])",
                    frametime.as_micros() as f64 / 1000.0,
                    pixels.len()
                )
            }
            ThreadMsg::Stop => write!(f, "Stop"),
            ThreadMsg::Pause => write!(f, "Pause"),
            ThreadMsg::Resume => write!(f, "Resume"),
            ThreadMsg::Step(steps) => write!(f, "Step({})", steps),
            ThreadMsg::SteppingFinished => write!(f, "SteppingFinished"),
            ThreadMsg::ConnectDevice(port, device_type) => {
                write!(f, "ConnectDevice({}, {:?})", port, device_type)
            }
            ThreadMsg::UpdateDeviceButton(port, device_button, pressed) => write!(
                f,
                "DeviceButtonPress({}, {:?}, {})",
                port,
                device_button.get_button_type_string(),
                pressed
            ),
        }
    }
}

impl Clone for ThreadMsg {
    fn clone(&self) -> Self {
        match self {
            ThreadMsg::Pause => ThreadMsg::Pause,
            ThreadMsg::Resume => ThreadMsg::Resume,
            ThreadMsg::Step(steps) => ThreadMsg::Step(*steps),
            ThreadMsg::NewFrame(duration, pixels) => ThreadMsg::NewFrame(*duration, pixels.clone()),
            ThreadMsg::Stop => ThreadMsg::Stop,
            ThreadMsg::SteppingFinished => ThreadMsg::SteppingFinished,
            ThreadMsg::ConnectDevice(port, device_type) => {
                ThreadMsg::ConnectDevice(*port, *device_type)
            }
            ThreadMsg::UpdateDeviceButton(port, device_button, pressed) => {
                ThreadMsg::UpdateDeviceButton(*port, device_button.box_clone(), *pressed)
            }
        }
    }
}

#[derive(Debug)]
pub struct MessagePacket(&'static str, ThreadMsg);

/// Handles sending and receiving messages between threads
#[derive(Debug, Clone)]
pub struct ThreadCom {
    tx: Sender<MessagePacket>,
    rx: Receiver<MessagePacket>,
    pending: Arc<Mutex<VecDeque<MessagePacket>>>,
}

impl ThreadCom {
    pub fn new(channel_size: Option<usize>) -> Self {
        let (tx, rx): (Sender<MessagePacket>, Receiver<MessagePacket>) = match channel_size {
            Some(size) => channel::bounded::<MessagePacket>(size),
            None => channel::unbounded::<MessagePacket>(),
        };
        ThreadCom {
            tx,
            rx,
            pending: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn send(&self, target: &'static str, msg: ThreadMsg) -> Result<(), ThreadComError> {
        match self.tx.try_send(MessagePacket(target, msg)) {
            Ok(_) => Ok(()),
            Err(TrySendError::Full(_)) => Err(ThreadComError::Full),
            Err(TrySendError::Disconnected(_)) => Err(ThreadComError::Disconnected),
        }
    }

    pub fn await_send(
        &self,
        target: &'static str,
        msg: ThreadMsg,
        timeout_millis: Option<u64>,
    ) -> Result<(), ThreadComError> {
        let message: MessagePacket = MessagePacket(target, msg);
        match timeout_millis {
            Some(timeout_millis) => {
                match self
                    .tx
                    .send_timeout(message, Duration::from_millis(timeout_millis))
                {
                    Ok(_) => Ok(()),
                    Err(SendTimeoutError::Timeout(_)) => Err(ThreadComError::Timeout),
                    Err(SendTimeoutError::Disconnected(_)) => Err(ThreadComError::Disconnected),
                }
            }
            None => match self.tx.send(message) {
                Ok(_) => Ok(()),
                Err(_) => Err(ThreadComError::Disconnected),
            },
        }
    }

    pub fn get_waiting_messages(&self, target: &'static str) -> Vec<ThreadMsg> {
        let mut messages: Vec<ThreadMsg> = Vec::new();
        let mut pending: MutexGuard<'_, VecDeque<MessagePacket>> = self.pending.lock().unwrap();

        // First, drain matching messages from pending
        let mut i: usize = 0;
        while i < pending.len() {
            if pending[i].0 == target {
                messages.push(pending.remove(i).unwrap().1);
            } else {
                i += 1;
            }
        }

        // Then, try to receive new messages from the channel
        for msg_obj in self.rx.try_iter() {
            if msg_obj.0 == target {
                messages.push(msg_obj.1);
            } else {
                pending.push_back(msg_obj);
            }
        }
        messages
    }

    pub fn get_message(&self, target: &'static str) -> Result<ThreadMsg, ThreadComError> {
        let mut pending: MutexGuard<'_, VecDeque<MessagePacket>> = self.pending.lock().unwrap();
        // Check pending first
        let mut i: usize = 0;
        while i < pending.len() {
            if pending[i].0 == target {
                return Ok(pending.remove(i).unwrap().1);
            } else {
                i += 1;
            }
        }

        // Try to receive from channel
        match self.rx.try_recv() {
            Ok(msg_obj) => {
                if msg_obj.0 == target {
                    Ok(msg_obj.1)
                } else {
                    pending.push_back(msg_obj);
                    Err(ThreadComError::Empty)
                }
            }
            Err(TryRecvError::Empty) => Err(ThreadComError::Empty),
            Err(TryRecvError::Disconnected) => Err(ThreadComError::Disconnected),
        }
    }

    pub fn await_message(
        &self,
        target: &'static str,
        timeout_millis: Option<u64>,
    ) -> Result<ThreadMsg, ThreadComError> {
        let mut pending: MutexGuard<'_, VecDeque<MessagePacket>> = self.pending.lock().unwrap();
        // Check pending first
        let mut i: usize = 0;
        while i < pending.len() {
            if pending[i].0 == target {
                return Ok(pending.remove(i).unwrap().1);
            } else {
                i += 1;
            }
        }
        drop(pending); // Release lock before blocking

        match timeout_millis {
            Some(timeout_millis) => {
                let deadline: Instant = Instant::now() + Duration::from_millis(timeout_millis);
                loop {
                    let now: Instant = Instant::now();
                    let remaining: Duration = if deadline > now {
                        deadline - now
                    } else {
                        Duration::from_millis(0)
                    };

                    match self.rx.recv_timeout(remaining) {
                        Ok(msg_obj) => {
                            if msg_obj.0 == target {
                                return Ok(msg_obj.1);
                            } else {
                                let mut pending: MutexGuard<'_, VecDeque<MessagePacket>> =
                                    self.pending.lock().unwrap();
                                pending.push_back(msg_obj);
                                // Continue loop until timeout
                            }
                        }
                        Err(RecvTimeoutError::Timeout) => return Err(ThreadComError::Timeout),
                        Err(RecvTimeoutError::Disconnected) => {
                            return Err(ThreadComError::Disconnected)
                        }
                    }
                }
            }
            None => loop {
                match self.rx.recv() {
                    Ok(msg_obj) => {
                        if msg_obj.0 == target {
                            return Ok(msg_obj.1);
                        } else {
                            let mut pending: MutexGuard<'_, VecDeque<MessagePacket>> =
                                self.pending.lock().unwrap();
                            pending.push_back(msg_obj);
                            // Continue loop
                        }
                    }
                    Err(_) => return Err(ThreadComError::Dropped),
                }
            },
        }
    }

    pub fn is_rx_empty(&self, target: &'static str) -> bool {
        let pending: MutexGuard<'_, VecDeque<MessagePacket>> = self.pending.lock().unwrap();
        if pending
            .iter()
            .any(|msg_obj: &MessagePacket| msg_obj.0 == target)
        {
            return false;
        }
        drop(pending);

        for msg_obj in self.rx.try_iter() {
            let mut pending: MutexGuard<'_, VecDeque<MessagePacket>> = self.pending.lock().unwrap();
            if msg_obj.0 == target {
                pending.push_back(msg_obj);
                return false;
            } else {
                pending.push_back(msg_obj);
            }
        }
        true
    }
}
