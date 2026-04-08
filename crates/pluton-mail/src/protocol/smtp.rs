//! SMTP (Simple Mail Transfer Protocol) - Outgoing

use lettre::{SmtpTransport, Transport, transport::smtp::authentication::Credentials};

use crate::{
  MailError,
  protocol::{OutgoingMessage, OutgoingProtocol},
};

pub struct SmtpOutgoing {
  mailer: SmtpTransport,
}

impl OutgoingProtocol for SmtpOutgoing {
  fn send_email<M: Into<OutgoingMessage>>(&mut self, email: M) -> Result<(), MailError> {
    match email.into() {
      OutgoingMessage::LettreMessage(m) => {
        self.mailer.send(&m)?;
      }
    }

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
