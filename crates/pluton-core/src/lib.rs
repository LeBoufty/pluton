use thiserror::Error;

pub mod accounts;
pub mod filepaths;

#[derive(Debug, Error)]
pub enum PlutonCoreError {
  #[error("account already exists")]
  AccountAlreadyExists,

  #[error(transparent)]
  SerdeJSONError(#[from] serde_json::Error),

  #[error(transparent)]
  IOError(#[from] std::io::Error),

  #[error(transparent)]
  SecureStoreError(#[from] securestore::Error),
}

pub type PlutonCoreResult<R> = Result<R, PlutonCoreError>;
