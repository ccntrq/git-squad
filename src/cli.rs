use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "git-squad")]
#[command(about = "Manage pair or mob programming co-authors in Git commit messages", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Use a custom buddy file instead of ~/.config/git-squad/buddies.yaml
    #[arg(long = "buddies-file", global = true)]
    pub buddies_file: Option<PathBuf>,
}

impl Cli {
    pub fn command(&self) -> Command {
        self.command.clone().unwrap_or(Command::Info)
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    /// Add a buddy to the current session
    With {
        /// The alias of the buddy to add
        alias: String,
    },

    /// Remove a buddy from the current session
    Without {
        /// The alias of the buddy to remove
        alias: String,
    },

    /// Remove all buddies from the current session
    Alone,

    /// Create a new buddy
    Create {
        /// The alias for the new buddy
        alias: String,
    },

    /// Delete a buddy from the list of available buddies
    Forget {
        /// The alias for the buddy to delete
        alias: String,
    },

    /// List both active and available buddies
    Info,

    /// List all available buddies
    List,

    /// List active buddies in the current session
    Active,
}

pub fn parse() -> Cli {
    Cli::parse()
}
