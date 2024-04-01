use std::sync::mpsc::{self, Receiver, Sender};

pub struct Requester<T, R> {
    tx: Sender<Request<T, R>>,
}

pub struct Responder<T, R> {
    rx: Receiver<Request<T, R>>,
}

impl<T, R> Requester<T, R> {
    pub fn send(&self, data: T) -> Result<Response<R>, SendError<T>> {
        let (response_tx, response_rx) = mpsc::channel();
        match self.tx.send(Request {
            data: Some(data),
            tx: response_tx,
        }) {
            Ok(_) => Ok(Response { rx: response_rx }),
            Err(e) => Err(SendError(e.0.data.unwrap())),
        }
    }
}

impl<T, R> Responder<T, R> {
    pub fn try_recv(&self) -> Result<Request<T, R>, TryRecvError> {
        match self.rx.try_recv() {
            Ok(r) => Ok(r),
            Err(mpsc::TryRecvError::Disconnected) => Err(TryRecvError::Disconnected {}),
            Err(mpsc::TryRecvError::Empty) => Err(TryRecvError::Empty {}),
        }
    }

    pub fn recv(&self) -> Result<Request<T, R>, RecvError> {
        match self.rx.recv() {
            Ok(r) => Ok(r),
            Err(_) => Err(RecvError {}),
        }
    }
}

pub fn channel<T, R>() -> (Requester<T, R>, Responder<T, R>) {
    let (requester_tx, responder_rx) = mpsc::channel();
    (
        Requester { tx: requester_tx },
        Responder { rx: responder_rx },
    )
}

pub struct Request<T, R> {
    pub data: Option<T>,
    tx: Sender<R>,
}

pub struct Response<R> {
    rx: Receiver<R>,
}

impl<T, R> Request<T, R> {
    pub fn respond(&self, data: R) -> Result<(), SendError<R>> {
        match self.tx.send(data) {
            Ok(_) => Ok(()),
            Err(e) => Err(SendError(e.0)),
        }
    }
}

impl<R> Response<R> {
    pub fn recv(&self) -> Result<R, RecvError> {
        match self.rx.recv() {
            Ok(r) => Ok(r),
            Err(_) => Err(RecvError {}),
        }
    }

    pub fn try_recv(&self) -> Result<R, TryRecvError> {
        match self.rx.try_recv() {
            Ok(r) => Ok(r),
            Err(mpsc::TryRecvError::Disconnected) => Err(TryRecvError::Disconnected {}),
            Err(mpsc::TryRecvError::Empty) => Err(TryRecvError::Empty {}),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RecvError {}

#[derive(Debug, Clone, Copy)]
pub enum TryRecvError {
    Empty,
    Disconnected,
}

#[derive(Debug, Clone, Copy)]
pub struct SendError<T>(pub T);
