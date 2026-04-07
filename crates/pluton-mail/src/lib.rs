use thiserror::Error;

use crate::protocol::{IncomingProtocol, OutgoingProtocol};

pub mod protocol;

#[derive(Error, Debug)]
pub enum MailError {
  #[error("no selected mailbox")]
  NoSelectedMailbox,

  #[error("no mail at id {0}")]
  NoMailAtId(u32),

  #[error("native tls error: {0}")]
  NativeTLSError(native_tls::Error),

  #[error("imap error: {0}")]
  ImapError(imap::Error),
}

pub struct MailAccount<I: IncomingProtocol, O: OutgoingProtocol> {
  pub incoming_interface: I,
  pub outgoing_interface: O,
}
