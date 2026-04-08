//! IMAP (Internet Message Access Protocol) - Incoming

use std::net::TcpStream;

use imap::Session;
use mail_parser::{Message, MessageParser};
use native_tls::TlsStream;

use crate::{
  MailError, MailResult,
  protocol::{EmailID, IncomingProtocol},
};

pub struct ImapIncoming {
  session: Session<TlsStream<TcpStream>>,
  current_mailbox: Option<String>,
}

impl ImapIncoming {
  fn select<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError> {
    self.current_mailbox = Some(mailbox.as_ref().to_string());
    self.session.select(mailbox)?;
    Ok(())
  }

  fn select_cached<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError> {
    if let Some(cm) = &self.current_mailbox {
      if cm.as_str() != mailbox.as_ref() {
        self.select(mailbox)?;
      }
    } else {
      self.select(mailbox)?;
    }
    Ok(())
  }
}

impl IncomingProtocol for ImapIncoming {
  fn list_mailboxes(&mut self) -> Result<Vec<String>, MailError> {
    let list = self.session.list(Some(""), Some("*"))?;

    let mut vec = Vec::new();
    for l in &list {
      vec.push(l.name().to_string());
    }

    Ok(vec)
  }

  fn list_email_ids_in_mailbox<S: AsRef<str>>(&mut self, mailbox: S) -> MailResult<Vec<EmailID>> {
    self.select_cached(mailbox)?;
    let set = self.session.search("ALL")?;
    let mut vec = set.iter().map(|s| *s as EmailID).collect::<Vec<EmailID>>();
    vec.sort();
    vec.reverse();
    Ok(vec)
  }

  fn get_emails_headers<S: AsRef<str>>(
    &mut self,
    mailbox: S,
    ids: &Vec<EmailID>,
  ) -> MailResult<Vec<Message<'static>>> {
    self.select_cached(mailbox)?;
    let sequence_set = ids
      .iter()
      .map(|f| format!("{}", *f))
      .collect::<Vec<String>>()
      .join(",");
    let mails = self.session.fetch(&sequence_set, "RFC822.HEADER")?;
    let mut parsed_messages = Vec::new();
    for mail in mails.iter() {
      let mail_body = mail.header().ok_or(MailError::NoMailBody)?;
      let mail_body_owned = mail_body.to_vec();
      let message = MessageParser::default()
        .parse_headers(&mail_body_owned)
        .ok_or(MailError::MailParsingError)?;
      parsed_messages.push(message.into_owned());
    }

    Ok(parsed_messages)
  }

  fn get_email_content<S: AsRef<str>>(
    &mut self,
    mailbox: S,
    id: &EmailID,
  ) -> MailResult<Message<'static>> {
    self.select_cached(mailbox)?;
    let mails = self.session.fetch(&format!("{}", id), "RFC822")?;
    if let Some(mail) = mails.first() {
      let mail_body = mail.body().ok_or(MailError::NoMailBody)?;
      let mail_body_owned = mail_body.to_vec();
      let message = MessageParser::default()
        .parse(&mail_body_owned)
        .ok_or(MailError::MailParsingError)?;
      Ok(message.into_owned())
    } else {
      Err(MailError::MailNotFound)
    }
  }
}

impl ImapIncoming {
  pub fn new_tls(domain: &str, port: u16, login: &str, password: &str) -> Result<Self, MailError> {
    let tls = native_tls::TlsConnector::builder().build()?;

    let client = imap::connect((domain, port), domain, &tls).map_err(MailError::ImapError)?;
    let session = client
      .login(login, password)
      .map_err(|(e, _)| MailError::ImapError(e))?;
    Ok(Self {
      session,
      current_mailbox: None,
    })
  }
}
