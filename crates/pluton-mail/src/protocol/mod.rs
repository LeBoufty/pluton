//! all protocols that can be used

use crate::MailResult;

pub mod imap;
pub mod smtp;

/// identifier used for emails
pub type EmailID = u64;

/// generic incoming protocol.
///
/// this implements Send so that Tauri can pass it between its threads
pub trait IncomingProtocol: Send {
  // mailboxes
  fn list_mailboxes(&mut self) -> MailResult<Vec<String>>;

  // emails
  fn list_email_ids_in_mailbox(&mut self, mailbox: &str) -> MailResult<Vec<EmailID>>;
  fn get_emails_headers(
    &mut self,
    mailbox: &str,
    ids: &Vec<EmailID>,
  ) -> MailResult<Vec<mail_parser::Message<'static>>>;

  // email
  fn get_email_content(
    &mut self,
    mailbox: &str,
    id: &EmailID,
  ) -> MailResult<mail_parser::Message<'static>>;
}

/// generic outgoing protocol.
///
/// this implements Send so that Tauri can pass it between its threads
pub trait OutgoingProtocol: Send {
  fn send_email<M: Into<OutgoingMessage>>(&mut self, email: M) -> MailResult<()>;
}

pub enum OutgoingMessage {
  LettreMessage(lettre::Message),
}

impl From<lettre::Message> for OutgoingMessage {
  fn from(value: lettre::Message) -> Self {
    Self::LettreMessage(value)
  }
}
