use crossbeam::channel::{
    self, Receiver, RecvTimeoutError, SendTimeoutError, Sender, TryRecvError, TrySendError,
};
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
    Temp,
}

#[derive(Debug, Clone)]
pub struct ThreadCom {
    tx: Option<Sender<ThreadMsg>>,
    rx: Option<Receiver<ThreadMsg>>,
}

impl ThreadCom {
    pub fn new() -> Self {
        ThreadCom { tx: None, rx: None }
    }

    pub fn init_channels(&mut self) {
        let (tx, rx): (Sender<ThreadMsg>, Receiver<ThreadMsg>) = channel::unbounded::<ThreadMsg>();

        assert!(tx.capacity().is_none_or(|capacity| capacity != 0));
        assert!(rx.capacity().is_none_or(|capacity| capacity != 0));

        self.tx = Some(tx);
        self.rx = Some(rx);
    }

    fn tx(&self) -> Result<&Sender<ThreadMsg>, ThreadComError> {
        if self.tx.is_none() {
            return Err(ThreadComError::Uninitialized);
        }
        Ok(self.tx.as_ref().unwrap())
    }

    fn rx(&self) -> Result<&Receiver<ThreadMsg>, ThreadComError> {
        if self.rx.is_none() {
            return Err(ThreadComError::Uninitialized);
        }
        Ok(self.rx.as_ref().unwrap())
    }

    pub fn send(&self, msg: ThreadMsg) -> Result<(), ThreadComError> {
        let tx: &Sender<ThreadMsg> = self.tx()?;
        match tx.try_send(msg) {
            Ok(_) => Ok(()),
            Err(TrySendError::Full(_)) => Err(ThreadComError::Full),
            Err(TrySendError::Disconnected(_)) => Err(ThreadComError::Disconnected),
        }
    }

    pub fn await_send(
        &self,
        msg: ThreadMsg,
        timeout_secs: Option<u64>,
    ) -> Result<(), ThreadComError> {
        let tx: &Sender<ThreadMsg> = self.tx()?;
        match timeout_secs {
            Some(timeout_secs) => match tx.send_timeout(msg, Duration::from_secs(timeout_secs)) {
                Ok(_) => Ok(()),
                Err(SendTimeoutError::Timeout(_)) => Err(ThreadComError::Timeout),
                Err(SendTimeoutError::Disconnected(_)) => Err(ThreadComError::Disconnected),
            },
            None => match tx.send(msg) {
                Ok(_) => Ok(()),
                Err(_) => Err(ThreadComError::Disconnected),
            },
        }
    }

    pub fn get_waiting_messages(&self) -> Result<Vec<ThreadMsg>, ThreadComError> {
        let rx: &Receiver<ThreadMsg> = self.rx()?;
        Ok(rx.try_iter().collect())
    }

    pub fn get_message(&self) -> Result<ThreadMsg, ThreadComError> {
        let rx: &Receiver<ThreadMsg> = self.rx()?;
        match rx.try_recv() {
            Ok(msg) => Ok(msg),
            Err(TryRecvError::Empty) => Err(ThreadComError::Empty),
            Err(TryRecvError::Disconnected) => Err(ThreadComError::Disconnected),
        }
    }

    pub fn await_message(&self, timeout_secs: Option<u64>) -> Result<ThreadMsg, ThreadComError> {
        let rx: &Receiver<ThreadMsg> = self.rx()?;
        match timeout_secs {
            Some(timeout_secs) => match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
                Ok(msg) => Ok(msg),
                Err(RecvTimeoutError::Timeout) => Err(ThreadComError::Timeout),
                Err(RecvTimeoutError::Disconnected) => Err(ThreadComError::Disconnected),
            },
            None => match rx.recv() {
                Ok(msg) => Ok(msg),
                Err(_) => Err(ThreadComError::Dropped),
            },
        }
    }
}
