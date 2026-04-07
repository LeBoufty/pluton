use pluton_core::accounts::manager::AccountManager;

pub struct PlutonState {
  pub accounts: AccountManager,
}

impl PlutonState {
  pub fn new(accounts: AccountManager) -> Self {
    Self { accounts }
  }
}
