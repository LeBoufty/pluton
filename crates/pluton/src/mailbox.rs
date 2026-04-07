use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
pub struct SMTPMailbox {
    username: String,
    address: String,
    relay: String,
}

pub type MailboxCollection = HashMap<String, SMTPMailbox>;

pub fn load_mailbox_data(f: &str) -> MailboxCollection {
    let data = read_to_string(f);
    if data.is_err() {
        eprintln!("{:?}", data.err().unwrap());
    }
    let collec: HashMap<String, SMTPMailbox>;
}
