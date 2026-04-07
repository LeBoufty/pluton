use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    io::Error,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SMTPMailbox {
    pub username: String,
    pub address: String,
    pub relay: String,
}

pub type MailboxCollection = HashMap<String, SMTPMailbox>;

pub fn load_mailbox_data(f: PathBuf) -> Result<MailboxCollection, Error> {
    let data = read_to_string(f);
    if data.is_err() {
        return Err(data.err().unwrap());
    }
    let content = data.as_deref().unwrap();
    let collec: HashMap<String, SMTPMailbox> = serde_json::from_str(content).unwrap();
    Ok(collec)
}

pub fn save_mailbox_data(
    collection: MailboxCollection,
    f: PathBuf,
) -> Result<(), serde_json::Error> {
    let data = serde_json::to_string(&collection);
    if data.is_err() {
        return Err(data.err().unwrap());
    }
    _ = write(f, data.unwrap());
    Ok(())
}
