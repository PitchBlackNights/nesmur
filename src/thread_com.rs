use crossbeam::channel::{
    self, Receiver, RecvTimeoutError, SendTimeoutError, Sender, TryRecvError, TrySendError,
};
use nes::ppu::renderer::RGB;
use std::time::Duration;

#[derive(Debug)]
pub enum ThreadComError {
    Full,
    Empty,
    Timeout,
    Disconnected,
    Dropped,
    Uninitialized,
}

#[derive(Debug)]
pub enum ThreadMsg {
    NewFrame(Duration, Vec<RGB>),
    Stop,
}

#[derive(Debug, Clone)]
pub struct ThreadCom {
    tx: Sender<ThreadMsg>,
    rx: Receiver<ThreadMsg>,
}

impl ThreadCom {
    pub fn new(channel_size: Option<usize>) -> Self {
        let (tx, rx): (Sender<ThreadMsg>, Receiver<ThreadMsg>) = match channel_size {
            Some(size) => channel::bounded::<ThreadMsg>(size),
            None => channel::unbounded::<ThreadMsg>(),
        };

        ThreadCom { tx, rx }
    }

    pub fn send(&self, msg: ThreadMsg) -> Result<(), ThreadComError> {
        match self.tx.try_send(msg) {
            Ok(_) => Ok(()),
            Err(TrySendError::Full(_)) => Err(ThreadComError::Full),
            Err(TrySendError::Disconnected(_)) => Err(ThreadComError::Disconnected),
        }
    }

    pub fn await_send(
        &self,
        msg: ThreadMsg,
        timeout_millis: Option<u64>,
    ) -> Result<(), ThreadComError> {
        match timeout_millis {
            Some(timeout_millis) => {
                match self
                    .tx
                    .send_timeout(msg, Duration::from_millis(timeout_millis))
                {
                    Ok(_) => Ok(()),
                    Err(SendTimeoutError::Timeout(_)) => Err(ThreadComError::Timeout),
                    Err(SendTimeoutError::Disconnected(_)) => Err(ThreadComError::Disconnected),
                }
            }
            None => match self.tx.send(msg) {
                Ok(_) => Ok(()),
                Err(_) => Err(ThreadComError::Disconnected),
            },
        }
    }

    pub fn get_waiting_messages(&self) -> Vec<ThreadMsg> {
        self.rx.try_iter().collect()
    }

    pub fn get_message(&self) -> Result<ThreadMsg, ThreadComError> {
        match self.rx.try_recv() {
            Ok(msg) => Ok(msg),
            Err(TryRecvError::Empty) => Err(ThreadComError::Empty),
            Err(TryRecvError::Disconnected) => Err(ThreadComError::Disconnected),
        }
    }

    pub fn await_message(&self, timeout_millis: Option<u64>) -> Result<ThreadMsg, ThreadComError> {
        match timeout_millis {
            Some(timeout_millis) => {
                match self.rx.recv_timeout(Duration::from_millis(timeout_millis)) {
                    Ok(msg) => Ok(msg),
                    Err(RecvTimeoutError::Timeout) => Err(ThreadComError::Timeout),
                    Err(RecvTimeoutError::Disconnected) => Err(ThreadComError::Disconnected),
                }
            }
            None => match self.rx.recv() {
                Ok(msg) => Ok(msg),
                Err(_) => Err(ThreadComError::Dropped),
            },
        }
    }

    pub fn is_rx_empty(&self) -> bool {
        self.rx.is_empty()
    }
}
