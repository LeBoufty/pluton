use std::net::TcpStream;

use imap::Session;
use native_tls::TlsStream;

use crate::{MailError, protocol::IncomingProtocol};

pub struct ImapIncoming {
  session: Session<TlsStream<TcpStream>>,
  current_mailbox: Option<String>,
}

impl IncomingProtocol for ImapIncoming {
  fn current_mailbox(&self) -> Result<String, MailError> {
    if let Some(cm) = &self.current_mailbox {
      Ok(cm.clone())
    } else {
      Err(MailError::NoSelectedMailbox)
    }
  }

  fn list_mailboxes(&mut self) -> Result<Vec<String>, MailError> {
    let list = self
      .session
      .list(Some(""), Some("*"))
      .map_err(MailError::ImapError)?;

    let mut vec = Vec::new();
    for l in &list {
      vec.push(l.name().to_string());
    }

    Ok(vec)
  }

  fn select<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError> {
    self.session.select(mailbox).map_err(MailError::ImapError)?;
    Ok(())
  }

  fn examine<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError> {
    self
      .session
      .examine(mailbox)
      .map_err(MailError::ImapError)?;
    Ok(())
  }

  fn create<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError> {
    self.session.create(mailbox).map_err(MailError::ImapError)?;
    Ok(())
  }

  fn delete<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError> {
    self.session.delete(mailbox).map_err(MailError::ImapError)?;
    Ok(())
  }

  fn rename<S: AsRef<str>>(&mut self, from: S, to: S) -> Result<(), MailError> {
    self
      .session
      .rename(from, to)
      .map_err(MailError::ImapError)?;
    Ok(())
  }

  fn subscribe<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError> {
    self
      .session
      .subscribe(mailbox)
      .map_err(MailError::ImapError)?;
    Ok(())
  }

  fn unsubscribe<S: AsRef<str>>(&mut self, mailbox: S) -> Result<(), MailError> {
    self
      .session
      .unsubscribe(mailbox)
      .map_err(MailError::ImapError)?;
    Ok(())
  }

  fn subscriptions<S: AsRef<str>>(&mut self) -> Result<String, MailError> {
    todo!()
  }

  fn list_emails_in_selected(&mut self) -> Result<Vec<u32>, MailError> {
    let set = self.session.search("ALL").map_err(MailError::ImapError)?;
    let mut vec = set.iter().map(|s| *s).collect::<Vec<u32>>();
    vec.sort();
    Ok(vec)
  }

  fn get_unique(&mut self, id: u32) -> Result<String, MailError> {
    let mails = self
      .session
      .fetch(&format!("{}", id), "RFC822")
      .map_err(MailError::ImapError)?;
    if let Some(mail) = mails.first() {
      // TODO: this is super unsafe for a billion reasons
      Ok(str::from_utf8(mail.body().unwrap()).unwrap().to_string())
    } else {
      Err(MailError::NoMailAtId(id))
    }
  }

  fn get_range(
    &mut self,
    _start: u32,
    _end: u32,
  ) -> Result<std::collections::HashMap<u32, String>, MailError> {
    todo!()
  }

  fn append_draft<S: AsRef<str>, B: AsRef<u8>>(&mut self, _content: B) -> Result<(), MailError> {
    todo!()
  }

  fn move_email_from_selected<S: AsRef<str>>(
    &mut self,
    _id: u32,
    _mailbox: S,
  ) -> Result<(), MailError> {
    todo!()
  }

  fn search<S: AsRef<str>>(
    &mut self,
    _query: S,
  ) -> Result<std::collections::HashSet<u32>, MailError> {
    todo!()
  }
}

impl ImapIncoming {
  pub fn new_tls(domain: &str, port: u16, login: &str, password: &str) -> Result<Self, MailError> {
    let tls = native_tls::TlsConnector::builder()
      .build()
      .map_err(MailError::NativeTLSError)?;

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
