use securestore::SecretsManager;

use crate::mailbox::MailboxCollection;

pub struct Accounts {
    mailboxes: MailboxCollection,
    passwords: SecretsManager,
}

impl Accounts {
    pub fn new(passwords: SecretsManager) -> Self {
        Self {
            mailboxes: passwords.keys().map(),
        }
    }
}
