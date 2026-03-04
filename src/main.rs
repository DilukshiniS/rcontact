mod cli;
mod models;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use models::Contact;
use storage::{load_contacts, save_contacts};
use prettytable::{Table, row};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name, phone, email } => {
            let mut contacts = load_contacts();

            let new_id = contacts.iter().map(|c| c.id).max().unwrap_or(0) + 1;

            let contact = Contact {
                id: new_id,
                name,
                phone,
                email,
            };

            contacts.push(contact);
            save_contacts(&contacts);

            println!("✅ Contact added successfully!");
        }

        Commands::List => {
            let contacts = load_contacts();

            if contacts.is_empty() {
                println!("No contacts found.");
                return;
            }

            let mut table = Table::new();
            table.add_row(row!["ID", "Name", "Phone", "Email"]);

            for contact in contacts {
                table.add_row(row![
                    contact.id,
                    contact.name,
                    contact.phone,
                    contact.email.unwrap_or("N/A".to_string())
                ]);
            }

            table.printstd();
        }

        Commands::Search { name } => {
            let contacts = load_contacts();
            let search_term = name.to_lowercase();

            let results: Vec<Contact> = contacts
                .into_iter()
                .filter(|c| c.name.to_lowercase().contains(&search_term))
                .collect();

            if results.is_empty() {
                println!("No matching contacts found.");
                return;
            }

            let mut table = Table::new();
            table.add_row(row!["ID", "Name", "Phone", "Email"]);

            for contact in results {
                table.add_row(row![
                    contact.id,
                    contact.name,
                    contact.phone,
                    contact.email.unwrap_or("N/A".to_string())
                ]);
            }

            table.printstd();
        }

        Commands::Delete { id } => {
            let mut contacts = load_contacts();

            let initial_len = contacts.len();
            contacts.retain(|c| c.id != id);

            if contacts.len() == initial_len {
                println!("❌ No contact found with ID {}", id);
            } else {
                save_contacts(&contacts);
                println!("✅ Contact deleted successfully!");
            }
        }
    }
}
