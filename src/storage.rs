use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::models::Contact;

const FILE_PATH: &str = "contacts.json";

pub fn load_contacts() -> Vec<Contact> {
    let mut file = match File::open(FILE_PATH) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}

pub fn save_contacts(contacts: &Vec<Contact>) {
    let json = serde_json::to_string_pretty(contacts).unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .unwrap();

    file.write_all(json.as_bytes()).unwrap();
}