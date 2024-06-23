use thiserror::Error;
// use std::thread::JoinError;
// use wasm_bindgen::JsValue;
use workflow_core::channel::{ChannelError, RecvError, SendError, TryRecvError, TrySendError};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error: {0}")]
    Custom(String),

    #[error("{0}")]
    Eframe(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ParseInt")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Channel send() error")]
    SendError,

    #[error("Channel recv() error")]
    RecvError,

    #[error("Channel try_send() error")]
    TrySendError,

    #[error("Channel try_recv() error")]
    TryRecvError,

    #[error("Channel error: {0}")]
    ChannelError(String),

    #[error(transparent)]
    FasterHex(#[from] faster_hex::Error),

    #[error(transparent)]
    Core(#[from] sparkle_core::error::Error),

    #[error("Node is not synced")]
    NodeNotSynced,

    #[error("Expecting listener id")]
    ListenerId,

    #[error(transparent)]
    Rpc(#[from] kaspa_rpc_core::RpcError),

    #[error(transparent)]
    Wrpc(#[from] kaspa_wrpc_client::error::Error),

    #[error(
        "RPC client version mismatch, please upgrade you client (needs: v{0}, connected to: v{1})"
    )]
    RpcApiVersion(String, String),

    #[error("Invalid network type - expected: {0} connected to: {1}")]
    InvalidNetworkType(String, String),
}

impl Error {
    pub fn custom<T: Into<String>>(msg: T) -> Self {
        Error::Custom(msg.into())
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Error::SendError
    }
}

impl<T> From<TrySendError<T>> for Error {
    fn from(_: TrySendError<T>) -> Self {
        Error::TrySendError
    }
}

impl From<RecvError> for Error {
    fn from(_: RecvError) -> Self {
        Error::RecvError
    }
}

impl From<TryRecvError> for Error {
    fn from(_: TryRecvError) -> Self {
        Error::TryRecvError
    }
}

impl<T> From<ChannelError<T>> for Error {
    fn from(err: ChannelError<T>) -> Self {
        Error::ChannelError(err.to_string())
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for Error {
    fn from(err: std::sync::mpsc::SendError<T>) -> Self {
        Error::custom(err.to_string())
    }
}

