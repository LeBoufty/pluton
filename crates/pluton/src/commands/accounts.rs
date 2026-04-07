use std::{collections::HashMap, sync::Mutex};

use pluton_core::accounts::{
  credentials::Credentials, Account, IncomingConnection, OutgoingConnection,
};
use tauri::State;

use crate::{
  commands::{CommandError, CommandResult},
  state::PlutonState,
};

#[tauri::command]
pub fn accounts_get_all(
  state: State<'_, Mutex<PlutonState>>,
) -> CommandResult<HashMap<u64, Account>> {
  let state = state.lock().map_err(|_| CommandError::InvalidMutexAccess)?;
  Ok(state.accounts.get_accounts().0.clone())
}

#[tauri::command]
pub fn accounts_add(
  state: State<'_, Mutex<PlutonState>>,
  username: String,
  incoming: IncomingConnection,
  outgoing: OutgoingConnection,
  imap_password: Option<String>,
  smtp_password: Option<String>,
) -> CommandResult<()> {
  let mut state = state.lock().map_err(|_| CommandError::InvalidMutexAccess)?;

  let new = Account::new(username, incoming, outgoing);
  state.accounts.add(
    new,
    Credentials {
      imap: imap_password,
      smtp: smtp_password,
    },
  )?;
  state.accounts.save()?;
  Ok(())
}

#[tauri::command]
pub fn accounts_delete(state: State<'_, Mutex<PlutonState>>, id: u64) -> CommandResult<()> {
  let mut state = state.lock().map_err(|_| CommandError::InvalidMutexAccess)?;
  state.accounts.remove(id)?;
  state.accounts.save()?;
  Ok(())
}
