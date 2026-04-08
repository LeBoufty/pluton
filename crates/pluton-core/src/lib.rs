//! **CORE LIBRARY FOR PLUTON**
//!
//! contains:
//! - mail interfaces with the app
//! - account and secret managers

use pluton_mail::MailError;
use thiserror::Error;

pub mod accounts;
pub mod filepaths;
pub mod interfaces;

#[derive(Debug, Error)]
pub enum PlutonCoreError {
  #[error("account already exists")]
  AccountAlreadyExists,

  #[error("account not found")]
  AccountNotFound,

  #[error(transparent)]
  PlutonMailError(#[from] MailError),

  #[error(transparent)]
  SerdeJSONError(#[from] serde_json::Error),

  #[error(transparent)]
  IOError(#[from] std::io::Error),

  #[error(transparent)]
  SecureStoreError(#[from] securestore::Error),
}

pub type PlutonCoreResult<R> = Result<R, PlutonCoreError>;
