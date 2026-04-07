use std::collections::{HashMap, HashSet};

use lettre::Message;

use crate::MailError;

pub mod imap;

pub trait IncomingProtocol {
  // mailboxes
  fn current_mailbox(&self) -> Result<String, MailError>;
  fn list_mailboxes(&mut self) -> Result<Vec<String>, MailError>;
  fn select<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError>;
  fn examine<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError>;
  fn create<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError>;
  fn delete<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError>;
  fn rename<S: AsRef<str>>(&mut self, from: S, to: S) -> Result<(), MailError>;
  fn subscribe<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError>;
  fn unsubscribe<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError>;
  fn subscriptions<S: AsRef<str>>(&mut self) -> Result<String, MailError>;

  // email
  fn list_emails_in_selected(&mut self) -> Result<Vec<u32>, MailError>;
  fn get_unique(&mut self, id: u32) -> Result<String, MailError>;
  fn get_range(&mut self, start: u32, end: u32) -> Result<HashMap<u32, String>, MailError>;
  /// appends draft. if no draft box was selected, an error should be raised
  fn append_draft<S: AsRef<str>, B: AsRef<u8>>(&mut self, content: B) -> Result<(), MailError>;
  fn move_email_from_selected<S: AsRef<str>>(
    &mut self,
    id: u32,
    mailbox: S,
  ) -> Result<(), MailError>;
  fn search<S: AsRef<str>>(&mut self, query: S) -> Result<HashSet<u32>, MailError>;

  // helpers
  fn move_email<S: AsRef<str>>(&mut self, id: u32, from: S, to: S) -> Result<(), MailError> {
    let current = self.current_mailbox()?;
    self.select(from)?;
    self.move_email_from_selected(id, to)?;
    self.select(current)?;
    Ok(())
  }
}

pub trait OutgoingProtocol {
  type Message;

  fn send_email(&mut self, email: Message) -> Result<(), MailError>;
}
