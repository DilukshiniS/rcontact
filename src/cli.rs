use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rcontact")]
#[command(about = "A simple Contact Book CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(long)]
        name: String,

        #[arg(long)]
        phone: String,

        #[arg(long)]
        email: Option<String>,
    },
    List,
    Search {
        #[arg(long)]
        name: String,
    },
    Delete {
        #[arg(long)]
        id: u32,
    },
}