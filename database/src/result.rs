use crate::error::StoreError;

pub type StoreResult<T> = std::result::Result<T, StoreError>;

pub trait StoreResultExtensions<T> {
    /// Unwrap or assert that the error is key not fund in which case `None` is returned
    fn unwrap_option(self) -> Option<T>;
}

impl<T> StoreResultExtensions<T> for StoreResult<T> {
    fn unwrap_option(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(StoreError::KeyNotFound(_)) => None,
            Err(err) => panic!("Unexpected store error: {err:?}"),
        }
    }
}

pub trait StoreResultEmptyTuple {
    /// Unwrap or assert that the error is key already exists
    fn unwrap_or_exists(self);
}

impl StoreResultEmptyTuple for StoreResult<()> {
    fn unwrap_or_exists(self) {
        match self {
            Ok(_) => (),
            Err(StoreError::KeyAlreadyExists(_)) | Err(StoreError::HashAlreadyExists(_)) => (),
            Err(err) => panic!("Unexpected store error: {err:?}"),
        }
    }
}
