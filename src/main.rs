use clap::Parser;
use cli::{Cli, Commands};
use database::Database;
mod action;
mod cli;
mod database;
mod utils;

fn main() {
    let mut db = Database::open();
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Add { content } => action::add(&mut db, Some(content)),
        Commands::Remove { id } => action::remove(&mut db, Some(id)),
        Commands::List => action::list(&db),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
