use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add todo  
    Add {
        /// todo content  
        content: String,
    },
    #[clap(about = "Remove a todo item.")]
    #[clap(visible_aliases = & ["ls"])]
    List,
    /// delete todo
    Remove {
        /// todo id
        id: u64,
    },
}
