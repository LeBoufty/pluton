use std::{
  collections::HashMap,
  fs::{read_to_string, write},
  io::Error,
  path::PathBuf,
};

use serde::{Deserialize, Serialize};

pub mod credentials;
pub mod manager;

#[derive(Debug, Serialize, Deserialize)]
pub struct IMAPIncomingConnection {
  pub server: String,
  pub port: u16,
  pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IncomingConnection {
  IMAP(IMAPIncomingConnection),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SMTPOutgoingConnection {
  pub server: String,
  pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutgoingConnection {
  SMTP(SMTPOutgoingConnection),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
  pub id: u64,
  pub username: String,
  pub incoming: IncomingConnection,
}

type AccountInnerCollection = HashMap<u64, Account>;

pub struct AccountCollection(AccountInnerCollection);

impl AccountCollection {
  pub fn new() -> Self {
    Self(AccountInnerCollection::new())
  }

  pub fn load_from_disk(f: PathBuf) -> Result<Self, Error> {
    let data = read_to_string(f);
    if data.is_err() {
      return Err(data.err().unwrap());
    }
    let content = data.as_deref().unwrap();
    let collec: AccountInnerCollection = serde_json::from_str(content).unwrap();
    Ok(Self(collec))
  }

  pub fn save_to_disk(&self, f: PathBuf) -> Result<(), serde_json::Error> {
    let data = serde_json::to_string(&self.0);
    if data.is_err() {
      return Err(data.err().unwrap());
    }
    _ = write(f, data.unwrap());
    Ok(())
  }
}
