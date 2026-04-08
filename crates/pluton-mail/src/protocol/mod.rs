//! all protocols that can be used

use mail_parser::Message;

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
  ) -> MailResult<Vec<Message<'static>>>;

  // email
  fn get_email_content(&mut self, mailbox: &str, id: &EmailID) -> MailResult<Message<'static>>;
}

/// generic outgoing protocol.
///
/// this implements Send so that Tauri can pass it between its threads
pub trait OutgoingProtocol: Send {
  type Message;

  fn send_email(&mut self, email: &<Self as OutgoingProtocol>::Message) -> MailResult<()>;
}
