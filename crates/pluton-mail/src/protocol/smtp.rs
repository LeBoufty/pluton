//! SMTP (Simple Mail Transfer Protocol) - Outgoing

use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};

use crate::{MailError, protocol::OutgoingProtocol};

pub struct SmtpOutgoing {
  mailer: SmtpTransport,
}

impl OutgoingProtocol for SmtpOutgoing {
  type Message = Message;

  fn send_email(&mut self, email: &Message) -> Result<(), MailError> {
    self.mailer.send(email)?;
    Ok(())
  }
}

impl SmtpOutgoing {
  pub fn new(relay: &str, login: &str, password: &str) -> Result<Self, MailError> {
    let creds = Credentials::new(login.to_owned(), password.to_owned());
    let mailer = SmtpTransport::relay(relay)?.credentials(creds).build();
    Ok(Self { mailer })
  }
}
