use securestore::SecretsManager;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
  PlutonCoreError, PlutonCoreResult,
  accounts::{Account, AccountCollection, credentials::Credentials},
  filepaths::accounts_file,
};

pub struct AccountManager {
  accounts: AccountCollection,
  passwords: SecretsManager,
}

impl AccountManager {
  pub fn new(passwords: SecretsManager) -> Self {
    let accounts: AccountCollection;
    let loaded_data = AccountCollection::load_from_disk(accounts_file());
    if loaded_data.is_err() {
      accounts = AccountCollection::new();
    } else {
      accounts = loaded_data.unwrap();
    }
    Self {
      accounts,
      passwords,
    }
  }

  pub fn get_accounts(&self) -> &AccountCollection {
    &self.accounts
  }

  pub fn get_password(&self, key: &str) -> PlutonCoreResult<String> {
    let v = self.passwords.get(key)?;
    Ok(v)
  }

  pub fn add(&mut self, mailbox: Account, creds: Credentials) -> PlutonCoreResult<()> {
    let address = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();
    if self.accounts.0.contains_key(&address) {
      return Err(PlutonCoreError::AccountAlreadyExists);
    }
    self.accounts.0.insert(address, mailbox);
    if let Some(imap) = creds.imap {
      self.passwords.set(&format!("imap-{}", address), imap);
    }
    if let Some(smtp) = creds.smtp {
      self.passwords.set(&format!("smtp-{}", address), smtp);
    }

    Ok(())
  }

  pub fn remove(&mut self, id: u64) -> PlutonCoreResult<()> {
    self.accounts.0.remove(&id);
    match self.passwords.remove(&format!("imap-{}", id)) {
      Ok(_) => (),
      Err(e) => match e.kind() {
        securestore::ErrorKind::SecretNotFound => (),
        _ => return Err(PlutonCoreError::SecureStoreError(e)),
      },
    };
    match self.passwords.remove(&format!("smtp-{}", id)) {
      Ok(_) => (),
      Err(e) => match e.kind() {
        securestore::ErrorKind::SecretNotFound => (),
        _ => return Err(PlutonCoreError::SecureStoreError(e)),
      },
    };
    Ok(())
  }

  pub fn save(&self) -> PlutonCoreResult<()> {
    self.accounts.save_to_disk(accounts_file())?;
    self.passwords.save()?;
    Ok(())
  }
}
