use std::collections::HashMap;

use securestore::SecretsManager;

use crate::mailbox::{MailboxCollection, SMTPMailbox, load_mailbox_data, save_mailbox_data};

const MAILBOXES_FILE: &str = "data/mailboxes.json";

#[derive(Debug, Clone)]
pub struct AccountError;

pub struct Accounts {
    mailboxes: MailboxCollection,
    passwords: SecretsManager,
}

impl Accounts {
    pub fn new(passwords: SecretsManager) -> Self {
        let mailboxes: MailboxCollection;
        let loaded_data = load_mailbox_data(MAILBOXES_FILE);
        if loaded_data.is_err() {
            mailboxes = HashMap::new();
        } else {
            mailboxes = loaded_data.unwrap();
        }
        Self {
            mailboxes,
            passwords,
        }
    }

    pub fn add(&mut self, mailbox: SMTPMailbox, password: String) -> Result<(), AccountError> {
        let address = mailbox.address.clone();
        if self.mailboxes.contains_key(&address) {
            return Err(AccountError);
        }
        self.mailboxes.insert(address.clone(), mailbox);
        self.passwords.set(address.as_str(), password);
        Ok(())
    }

    pub fn save(self) -> Result<(), AccountError> {
        let mb_saved = save_mailbox_data(self.mailboxes, MAILBOXES_FILE);
        let pw_saved = self.passwords.save();
        if mb_saved.is_err() || pw_saved.is_err() {
            Err(AccountError)
        } else {
            Ok(())
        }
    }
}
