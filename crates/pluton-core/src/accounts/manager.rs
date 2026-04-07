use securestore::SecretsManager;

use crate::{
  accounts::{Account, AccountCollection, credentials::Credentials},
  filepaths::mailboxes_file,
};

#[derive(Debug, Clone)]
pub struct AccountError;

pub struct AccountManager {
  mailboxes: AccountCollection,
  passwords: SecretsManager,
}

impl AccountManager {
  pub fn new(passwords: SecretsManager) -> Self {
    let mailboxes: AccountCollection;
    let loaded_data = AccountCollection::load_from_disk(mailboxes_file());
    if loaded_data.is_err() {
      mailboxes = AccountCollection::new();
    } else {
      mailboxes = loaded_data.unwrap();
    }
    Self {
      mailboxes,
      passwords,
    }
  }

  pub fn add(&mut self, mailbox: Account, creds: Credentials) -> Result<(), AccountError> {
    let address = mailbox.id.clone();
    if self.mailboxes.0.contains_key(&address) {
      return Err(AccountError);
    }
    self.mailboxes.0.insert(address, mailbox);
    if let Some(imap) = creds.imap {
      self.passwords.set(&format!("imap-{}", address), imap);
    }
    if let Some(smtp) = creds.smtp {
      self.passwords.set(&format!("smtp-{}", address), smtp);
    }

    Ok(())
  }

  pub fn save(self) -> Result<(), AccountError> {
    let mb_saved = self.mailboxes.save_to_disk(mailboxes_file());
    let pw_saved = self.passwords.save();
    if mb_saved.is_err() || pw_saved.is_err() {
      Err(AccountError)
    } else {
      Ok(())
    }
  }
}
