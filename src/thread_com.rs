use crossbeam::channel::{
    self, Receiver, RecvTimeoutError, SendTimeoutError, Sender, TryRecvError, TrySendError,
};
use nes::input_device::{NESDeviceButton, NESDeviceType};
use nes::ppu::renderer::RGB;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum ThreadComError {
    Full,
    Empty,
    Timeout,
    Disconnected,
    Dropped,
    Uninitialized,
}

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
            ThreadMsg::ConnectDevice(port, device_type) => write!(f, "ConnectDevice({}, {:?})", port, device_type),
            ThreadMsg::UpdateDeviceButton(port, device_button, pressed) => write!(f, "DeviceButtonPress({}, {:?}, {})", port, device_button.get_button_type_string(), pressed),
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
            ThreadMsg::ConnectDevice(port, device_type) => ThreadMsg::ConnectDevice(*port, *device_type),
            ThreadMsg::UpdateDeviceButton(port, device_button, pressed) => ThreadMsg::UpdateDeviceButton(*port, device_button.box_clone(), *pressed),
        }
    }
}

#[derive(Debug)]
pub struct ThreadMsgObj(&'static str, ThreadMsg);

#[derive(Debug, Clone)]
pub struct ThreadCom {
    tx: Sender<ThreadMsgObj>,
    rx: Receiver<ThreadMsgObj>,
    pending: Arc<Mutex<VecDeque<ThreadMsgObj>>>,
}

impl ThreadCom {
    pub fn new(channel_size: Option<usize>) -> Self {
        let (tx, rx): (Sender<ThreadMsgObj>, Receiver<ThreadMsgObj>) = match channel_size {
            Some(size) => channel::bounded::<ThreadMsgObj>(size),
            None => channel::unbounded::<ThreadMsgObj>(),
        };
        ThreadCom {
            tx,
            rx,
            pending: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn send(&self, destination: &'static str, msg: ThreadMsg) -> Result<(), ThreadComError> {
        match self.tx.try_send(ThreadMsgObj(destination, msg)) {
            Ok(_) => Ok(()),
            Err(TrySendError::Full(_)) => Err(ThreadComError::Full),
            Err(TrySendError::Disconnected(_)) => Err(ThreadComError::Disconnected),
        }
    }

    pub fn await_send(
        &self,
        destination: &'static str,
        msg: ThreadMsg,
        timeout_millis: Option<u64>,
    ) -> Result<(), ThreadComError> {
        let message: ThreadMsgObj = ThreadMsgObj(destination, msg);
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

    pub fn get_waiting_messages(&self, destination: &'static str) -> Vec<ThreadMsg> {
        let mut messages: Vec<ThreadMsg> = Vec::new();
        let mut pending = self.pending.lock().unwrap();

        // First, drain matching messages from pending
        let mut i = 0;
        while i < pending.len() {
            if pending[i].0 == destination {
                messages.push(pending.remove(i).unwrap().1);
            } else {
                i += 1;
            }
        }

        // Then, try to receive new messages from the channel
        for msg_obj in self.rx.try_iter() {
            if msg_obj.0 == destination {
                messages.push(msg_obj.1);
            } else {
                pending.push_back(msg_obj);
            }
        }
        messages
    }

    pub fn get_message(&self, destination: &'static str) -> Result<ThreadMsg, ThreadComError> {
        let mut pending: MutexGuard<'_, VecDeque<ThreadMsgObj>> = self.pending.lock().unwrap();
        // Check pending first
        let mut i: usize = 0;
        while i < pending.len() {
            if pending[i].0 == destination {
                return Ok(pending.remove(i).unwrap().1);
            } else {
                i += 1;
            }
        }

        // Try to receive from channel
        match self.rx.try_recv() {
            Ok(msg_obj) => {
                if msg_obj.0 == destination {
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
        destination: &'static str,
        timeout_millis: Option<u64>,
    ) -> Result<ThreadMsg, ThreadComError> {
        let mut pending: MutexGuard<'_, VecDeque<ThreadMsgObj>> = self.pending.lock().unwrap();
        // Check pending first
        let mut i = 0;
        while i < pending.len() {
            if pending[i].0 == destination {
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
                            if msg_obj.0 == destination {
                                return Ok(msg_obj.1);
                            } else {
                                let mut pending: MutexGuard<'_, VecDeque<ThreadMsgObj>> =
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
                        if msg_obj.0 == destination {
                            return Ok(msg_obj.1);
                        } else {
                            let mut pending: MutexGuard<'_, VecDeque<ThreadMsgObj>> =
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

    pub fn is_rx_empty(&self, destination: &'static str) -> bool {
        let pending: MutexGuard<'_, VecDeque<ThreadMsgObj>> = self.pending.lock().unwrap();
        if pending
            .iter()
            .any(|msg_obj: &ThreadMsgObj| msg_obj.0 == destination)
        {
            return false;
        }
        drop(pending);

        for msg_obj in self.rx.try_iter() {
            let mut pending: MutexGuard<'_, VecDeque<ThreadMsgObj>> = self.pending.lock().unwrap();
            if msg_obj.0 == destination {
                pending.push_back(msg_obj);
                return false;
            } else {
                pending.push_back(msg_obj);
            }
        }
        true
    }
}
