# rcontact - Rust Contact Book CLI

## 📌 Project Description
A Rust command-line application to manage contacts using JSON storage.

## 🚀 How to Run

Build the project:
cargo build

Run commands:
cargo run -- add --name "John Doe" --phone "0771234567" --email "john@gmail.com"
cargo run -- list
cargo run -- search --name "John"
cargo run -- delete --id 1

## 📦 Features

- Add contact (auto incremental ID)
- List contacts in table format
- Case-insensitive partial search
- Delete contact by ID
- Persistent JSON storage

## 📁 Data Storage

All contacts are stored in:
contacts.json