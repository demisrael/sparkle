use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("User abort")]
    UserAbort,

    #[error(transparent)]
    HttpError(#[from] workflow_http::error::Error),

    #[error(transparent)]
    Core(#[from] sparkle_core::error::Error),

    #[error(transparent)]
    SparkleRpcClient(#[from] sparkle_rpc_client::error::Error),

    #[error(transparent)]
    Wallet(#[from] kaspa_wallet_core::error::Error),

    #[error(transparent)]
    KaspaRpcClient(#[from] kaspa_wrpc_client::error::Error),

    #[error("Indexer error: {0}")]
    IndexerError(String),
}

impl Error {
    pub fn custom<T: Into<String>>(msg: T) -> Self {
        Error::Custom(msg.into())
    }
}
