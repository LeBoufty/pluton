//! a wrapper around protocols

use std::collections::HashMap;

use mail_parser::Message;

use crate::{
  MailResult,
  protocol::{EmailID, IncomingProtocol, OutgoingProtocol},
};

pub mod trait_impl;

/// generic mail interface, use this for storage
pub type AnyMailInterface = Box<dyn MailInterfacer>;

/// mail interfacer trait, used for interface genericization
pub trait MailInterfacer: Send {
  /// wake interfacer up
  fn awaken(&mut self) -> MailResult<()>;
  /// put interfacer to sleep
  fn sleep(&mut self) -> MailResult<()>;
  /// update interfacer automatically
  fn update(&mut self) -> MailResult<()>;
  /// manual refresh from user
  fn refresh(&mut self) -> MailResult<()>;

  // -- incoming
  // mailboxes
  fn get_mailboxes(&mut self) -> MailResult<Vec<String>>;
  fn get_email_headers_in_mailbox(
    &mut self,
    mailbox: &str,
  ) -> MailResult<&Vec<(EmailID, Message<'static>)>>;

  // emails
  fn get_email_content(&mut self, mailbox: &str, email: &EmailID) -> MailResult<&Message<'static>>;
}

/// templated mail interface, don't use this except for instanciation
pub struct MailInterface<I: IncomingProtocol, O: OutgoingProtocol> {
  pub incoming: I,
  pub outgoing: O,

  awake: bool,

  /// email id cache
  emails_ids: HashMap<String, HashMap<EmailID, usize>>,
  /// email cache
  ///
  /// emails here may be headers, or complete emails if they were fetched
  emails: HashMap<String, Vec<(EmailID, Message<'static>)>>,
}

impl<I: IncomingProtocol, O: OutgoingProtocol> MailInterface<I, O> {
  pub fn new(incoming: I, outgoing: O) -> MailResult<Self> {
    let s = Self {
      incoming,
      outgoing,
      awake: false,
      emails_ids: HashMap::new(),
      emails: HashMap::new(),
    };

    Ok(s)
  }
}
