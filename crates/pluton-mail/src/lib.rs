//! **MAIL PROTOCOL MANAGEMENT FOR PLUTON**
//!
//! all mail communication goes through here.

use thiserror::Error;

pub mod interface;
pub mod protocol;

#[derive(Error, Debug)]
pub enum MailError {
  #[error("no selected mailbox")]
  NoSelectedMailbox,

  #[error("mail not found at provided id")]
  MailNotFound,

  #[error("no mail body")]
  NoMailBody,

  #[error("couldn't parse mail")]
  MailParsingError,

  #[error("invalid sent message type")]
  InvalidSentMessageType,

  #[error(transparent)]
  NativeTLSError(#[from] native_tls::Error),

  #[error(transparent)]
  ImapError(#[from] imap::Error),

  #[error(transparent)]
  LettreSmtpTransportError(#[from] lettre::transport::smtp::Error),

  #[error(transparent)]
  UTF8Error(#[from] std::str::Utf8Error),
}

pub type MailResult<R> = Result<R, MailError>;
