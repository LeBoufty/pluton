use std::{
  collections::HashMap,
  fs::{read_to_string, write},
  io::Error,
  path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::PlutonCoreResult;

pub mod credentials;
pub mod manager;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IMAPIncomingConnection {
  pub server: String,
  pub port: u16,
  pub address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IncomingConnection {
  IMAP(IMAPIncomingConnection),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SMTPOutgoingConnection {
  pub server: String,
  pub address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OutgoingConnection {
  SMTP(SMTPOutgoingConnection),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
  pub order: i64,
  pub username: String,
  pub incoming: IncomingConnection,
  pub outgoing: OutgoingConnection,
}

impl Account {
  pub fn new(username: String, incoming: IncomingConnection, outgoing: OutgoingConnection) -> Self {
    Self {
      order: 0,
      username,
      incoming,
      outgoing,
    }
  }
}

type AccountInnerCollection = HashMap<u64, Account>;

pub struct AccountCollection(pub AccountInnerCollection);

impl AccountCollection {
  pub fn new() -> Self {
    Self(AccountInnerCollection::new())
  }

  pub fn load_from_disk(f: PathBuf) -> Result<Self, Error> {
    let data = read_to_string(f)?;
    let collec: AccountInnerCollection = serde_json::from_str(&data).unwrap();
    Ok(Self(collec))
  }

  pub fn save_to_disk(&self, f: PathBuf) -> PlutonCoreResult<()> {
    let data = serde_json::to_string(&self.0)?;
    write(f, data)?;
    Ok(())
  }
}
