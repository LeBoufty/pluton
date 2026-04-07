use std::{
    fs::{create_dir, exists},
    path::PathBuf,
};

use dirs::data_local_dir;

fn plutonmail_folder() -> PathBuf {
    data_local_dir().unwrap().as_path().join("pluton")
}

// ACCOUNTS

fn accounts_folder() -> PathBuf {
    plutonmail_folder().join("accounts")
}

pub fn key_file() -> PathBuf {
    accounts_folder().join("secrets.key")
}

pub fn pw_file() -> PathBuf {
    accounts_folder().join("secrets.json")
}

pub fn mailboxes_file() -> PathBuf {
    accounts_folder().join("mailboxes.json")
}

pub fn create_folders() -> () {
    if !exists(plutonmail_folder()).unwrap() {
        _ = create_dir(plutonmail_folder());
    }
    if !exists(accounts_folder()).unwrap() {
        _ = create_dir(accounts_folder());
    }
    ()
}
