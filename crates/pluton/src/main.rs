// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::exists;

use pluton_core::{
  accounts::manager::AccountManager,
  filepaths::{create_folders, key_file, pw_file},
};
use securestore::{KeySource, SecretsManager};

fn main() {
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
  _ = accounts.save();

  app_lib::run();
}
