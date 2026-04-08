//! app state

use pluton_core::{accounts::manager::AccountManager, interfaces::MailInterfaceManager};

pub struct PlutonState {
  pub accounts: AccountManager,
  pub mail_interfaces: MailInterfaceManager,
}

impl PlutonState {
  pub fn new(accounts: AccountManager, mail_interfaces: MailInterfaceManager) -> Self {
    Self {
      accounts,
      mail_interfaces,
    }
  }
}
