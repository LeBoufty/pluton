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
    let mut manager = SecretsManager::load(PW_FILE, KEY_FILE).unwrap();
    _ = manager.save();
}
