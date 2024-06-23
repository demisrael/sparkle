use crate::key::DbKey;
use sparkle_core::hash::Hash;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("key {0} not found in store")]
    KeyNotFound(DbKey),

    #[error("key {0} already exists in store")]
    KeyAlreadyExists(String),

    /// Specialization of key not found for the common `Hash` case.
    /// Added for avoiding the `String` allocation
    #[error("hash {0} already exists in store")]
    HashAlreadyExists(Hash),

    #[error("data inconsistency: {0}")]
    DataInconsistency(String),

    #[error("rocksdb error {0}")]
    DbError(#[from] rocksdb::Error),

    #[error("bincode error {0}")]
    DeserializationError(#[from] Box<bincode::ErrorKind>),

    #[error(transparent)]
    FdBudget(#[from] kaspa_utils::fd_budget::Error),
}
