use std::sync::Mutex;

use mail_parser::Message;
use pluton_core::accounts::AccountID;
use tauri::State;

use crate::{
  commands::{CommandError, CommandResult},
  state::PlutonState,
};

#[tauri::command]
pub fn get_mailboxes_for_id(
  state: State<'_, Mutex<PlutonState>>,
  account_id: AccountID,
) -> CommandResult<Vec<String>> {
  let mut state = state.lock().map_err(|_| CommandError::InvalidMutexAccess)?;
  let interfaces = &mut state.mail_interfaces;
  let interface = interfaces
    .get_mut(&account_id)
    .map_err(|_| CommandError::InvalidAccountID)?;
  let mailboxes = interface
    .get_mailboxes()
    .map_err(|e| CommandError::PlutonMailError(e))?;
  Ok(mailboxes.clone())
}

#[tauri::command]
pub fn get_email_headers_for_box(
  state: State<'_, Mutex<PlutonState>>,
  account_id: AccountID,
  mailbox: &str,
) -> CommandResult<Vec<(u64, Message<'static>)>> {
  let mut state = state.lock().map_err(|_| CommandError::InvalidMutexAccess)?;
  let interfaces = &mut state.mail_interfaces;
  let interface = interfaces
    .get_mut(&account_id)
    .map_err(|_| CommandError::InvalidAccountID)?;
  let emails = interface
    .get_email_headers_in_mailbox(mailbox)
    .map_err(|e| CommandError::PlutonMailError(e))?;
  Ok(emails.clone())
}
