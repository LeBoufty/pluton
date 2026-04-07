use pluton_core::PlutonCoreError;
use thiserror::Error;

pub mod accounts;

#[derive(Debug, Error)]
pub enum CommandError {
  #[error("fatal error while trying to access app state")]
  InvalidMutexAccess,

  #[error(transparent)]
  PlutonCoreError(#[from] PlutonCoreError),
}

impl From<CommandError> for String {
  fn from(value: CommandError) -> Self {
    value.to_string()
  }
}

// Add this for Tauri compatibility
impl serde::Serialize for CommandError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

pub type CommandResult<R> = Result<R, CommandError>;
