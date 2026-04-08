//! everything related to pluton in particular

use std::collections::HashSet;

use mail_parser::Message;

use crate::{
  MailResult,
  interface::{MailInterface, MailInterfacer},
  protocol::{EmailID, IncomingProtocol, OutgoingProtocol},
};

impl<I: IncomingProtocol, O: OutgoingProtocol> MailInterfacer for MailInterface<I, O> {
  fn awaken(&mut self) -> MailResult<()> {
    self.awake = true;
    Ok(())
  }

  fn sleep(&mut self) -> MailResult<()> {
    self.awake = false;
    Ok(())
  }

  fn update(&mut self) -> MailResult<()> {
    if !self.awake {
      return Ok(());
    }

    self.refresh()?;

    Ok(())
  }

  fn refresh(&mut self) -> MailResult<()> {
    Ok(())
  }

  fn get_email_headers_in_mailbox(
    &mut self,
    mailbox: &str,
  ) -> MailResult<&Vec<(EmailID, Message<'static>)>> {
    if !self.emails_ids.contains_key(mailbox) {
      self.emails_ids.insert(mailbox.to_string(), HashSet::new());
    }
    if !self.emails.contains_key(mailbox) {
      self.emails.insert(mailbox.to_string(), Vec::new());
    }

    if let Some(emails) = self.emails.get_mut(mailbox)
      && let Some(emails_ids) = self.emails_ids.get_mut(mailbox)
    {
      let searched_ids = self.incoming.list_email_ids_in_mailbox(&mailbox)?;
      let mut ids_to_fetch = Vec::new();
      for searched_id in searched_ids {
        if !emails_ids.contains(&searched_id) {
          emails_ids.insert(searched_id.clone());
          ids_to_fetch.push(searched_id);
        }
      }

      let fetched_emails = self.incoming.get_emails_headers(&mailbox, &ids_to_fetch)?;
      let fetched_zipped = ids_to_fetch
        .iter()
        .zip(fetched_emails)
        .collect::<Vec<(&EmailID, Message<'static>)>>();
      for (new_id, new_mail) in fetched_zipped {
        emails.push((new_id.clone(), new_mail));
      }

      Ok(emails)
    } else {
      unreachable!()
    }
  }
}
