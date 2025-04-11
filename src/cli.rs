use std::{io, path::PathBuf};

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Generator, Shell, generate};

#[derive(Debug, Parser)]
#[command(name = "git-squad")]
#[command(about = "Manage co-authors in git commit messages with ease", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Use a custom buddy file instead of ~/.config/git-squad/buddies.yaml
    #[arg(long = "buddies-file", global = true)]
    pub buddies_file: Option<PathBuf>,
}

impl Cli {
    pub fn get_command(&self) -> Command {
        self.command.clone().unwrap_or(Command::Info)
    }
}

pub fn print_completions<G: Generator>(generator: G) {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    generate(generator, &mut cmd, name, &mut io::stdout());
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    /// Add buddies to the current session
    With {
        /// The aliases of the buddies to add
        #[arg( required = true, num_args = 1..,)]
        // TODO: I would rather  use NonEmpty<String> here but clap makes
        // this really cumbersome
        aliases: Vec<String>,
    },

    /// Remove buddies from the current session
    Without {
        /// The aliases of the buddies to remove
        #[arg( required = true, num_args = 1..,)]
        // TODO: I would rather  use NonEmpty<String> here but clap makes
        // this really cumbersome
        aliases: Vec<String>,
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

    /// Generate completions for your shell
    Completions { shell: Shell },
}

pub fn parse() -> Cli {
    Cli::parse()
}
