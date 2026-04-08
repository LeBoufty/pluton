//! everything related to pluton in particular

use std::collections::HashMap;

use mail_parser::Message;

use crate::{
  MailResult,
  interface::{MailInterface, MailInterfacer},
  protocol::{EmailID, IncomingProtocol, OutgoingProtocol},
};

impl<I: IncomingProtocol, O: OutgoingProtocol> MailInterface<I, O> {
  fn email_cache_for_mailbox_prelude(&mut self, mailbox: &str) -> MailResult<()> {
    if !self.emails_ids.contains_key(mailbox) {
      self.emails_ids.insert(mailbox.to_string(), HashMap::new());
    }
    if !self.emails.contains_key(mailbox) {
      self.emails.insert(mailbox.to_string(), Vec::new());
    }

    Ok(())
  }
}

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

  fn get_mailboxes(&mut self) -> MailResult<Vec<String>> {
    let v = self.incoming.list_mailboxes()?;
    Ok(v)
  }

  fn get_email_headers_in_mailbox(
    &mut self,
    mailbox: &str,
  ) -> MailResult<&Vec<(EmailID, Message<'static>)>> {
    self.email_cache_for_mailbox_prelude(mailbox)?;

    if let Some(emails) = self.emails.get_mut(mailbox)
      && let Some(emails_ids) = self.emails_ids.get_mut(mailbox)
    {
      let searched_ids = self.incoming.list_email_ids_in_mailbox(&mailbox)?;
      let mut ids_to_fetch = Vec::new();
      for searched_id in searched_ids {
        if !emails_ids.contains_key(&searched_id) {
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
        emails_ids.insert(*new_id, emails.len() - 1);
      }

      Ok(emails)
    } else {
      unreachable!()
    }
  }

  fn get_email_content(&mut self, mailbox: &str, email: &EmailID) -> MailResult<&Message<'static>> {
    self.email_cache_for_mailbox_prelude(mailbox)?;

    if let Some(emails) = self.emails.get_mut(mailbox)
      && let Some(emails_ids) = self.emails_ids.get_mut(mailbox)
    {
      if let Some(message_index) = emails_ids.get(email) {
        // if everything is write, this unwrap should never happen
        // if it does, it means one email was improperly added to cache
        let v = emails.get(*message_index).unwrap();
        Ok(&v.1)
      } else {
        let message = self.incoming.get_email_content(mailbox, email)?;
        emails.push((*email, message.clone()));
        emails_ids.insert(*email, emails.len() - 1);
        Ok(&emails.last().unwrap().1)
      }
    } else {
      unreachable!()
    }
  }
}
