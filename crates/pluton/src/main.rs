use pluton::{accounts::Accounts, mailbox::SMTPMailbox};
use securestore::{KeySource, SecretsManager};
use std::fs::exists;

const KEY_FILE: &str = "data/secrets.key";
const PW_FILE: &str = "data/secrets.json";

fn main() {
    if !exists(KEY_FILE).unwrap() {
        println!("No key found. Generating.");
        let pvtkey = KeySource::Csprng;
        let manager_result = SecretsManager::new(pvtkey);
        if manager_result.is_err() {
            eprintln!(
                "Couldn't generate new secrets manager : {}",
                manager_result.err().unwrap().to_string()
            );
            return;
        }
        let manager = manager_result.unwrap();
        _ = manager.export_key(KEY_FILE);
        _ = manager.save_as(PW_FILE);
    }
    let manager = SecretsManager::load(PW_FILE, KEY_FILE).unwrap();
    let mut accounts = Accounts::new(manager);
    _ = accounts.add(
        SMTPMailbox {
            username: String::from("Whoa"),
            address: String::from("whoa@mafreidyne.motorcycles"),
            relay: String::from("smtp.mafreidyne.motorcycles"),
        },
        String::from("hey bro what's goooooooood"),
    );
    _ = accounts.save();
}
