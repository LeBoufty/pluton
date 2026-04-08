use pluton_core::PlutonCoreError;
use pluton_mail::MailError;
use thiserror::Error;

pub mod accounts;
pub mod mails;

/// serialized command errors for typescript error management
#[derive(Debug, Error)]
pub enum CommandError {
    #[error("fatal error while trying to access app state")]
    InvalidMutexAccess,

    #[error("the account ID that was provided is invalid")]
    InvalidAccountID,

    #[error(transparent)]
    PlutonCoreError(#[from] PlutonCoreError),

    #[error(transparent)]
    PlutonMailError(#[from] MailError),
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
