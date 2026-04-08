use std::{fs::exists, sync::Mutex};

use pluton_core::{
  accounts::manager::AccountManager,
  filepaths::{create_folders, key_file, pw_file},
  interfaces::MailInterfaceManager,
};
use securestore::{KeySource, SecretsManager};
use tauri::Manager;

use crate::state::PlutonState;

pub mod commands;
pub mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  create_folders();
  if !exists(key_file()).unwrap() || !exists(pw_file()).unwrap() {
    println!("No key found. Generating.");
    let pvtkey = KeySource::Csprng;
    let manager_result = SecretsManager::new(pvtkey);
    if manager_result.is_err() {
      eprintln!(
        "Couldn't generate new secrets manager : {:?}",
        manager_result.err().unwrap()
      );
      return;
    }
    let manager = manager_result.unwrap();
    let saved = manager.save_as(pw_file());
    let exported = manager.export_key(key_file());
    if saved.is_err() {
      eprintln!("{:?}", saved.err().unwrap());
    }
    if exported.is_err() {
      eprintln!("{:?}", exported.err().unwrap());
    }
  }
  let manager = SecretsManager::load(pw_file(), key_file()).unwrap();
  let accounts = AccountManager::new(manager);
  accounts.save().unwrap();
  let mail_interfaces = MailInterfaceManager::from_accounts(&accounts).unwrap();

  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
        app.manage(Mutex::new(PlutonState::new(accounts, mail_interfaces)));
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::accounts::accounts_get_all,
      commands::accounts::accounts_add,
      commands::accounts::accounts_delete,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
