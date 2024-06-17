use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error: {0}")]
    Custom(String),

    #[error("Http error: {0}")]
    HttpError(#[from] workflow_http::error::Error),

    #[error(transparent)]
    Core(#[from] sparkle_core::error::Error),

    #[error(transparent)]
    RpcClient(#[from] sparkle_rpc_client::error::Error),
}

impl Error {
    pub fn custom<T: Into<String>>(msg: T) -> Self {
        Error::Custom(msg.into())
    }
}
