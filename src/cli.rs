use std::{io, path::PathBuf};

use clap::{CommandFactory, Parser, Subcommand, builder::StyledStr};
use clap_complete::{
  CompleteEnv, Shell,
  engine::{ArgValueCompleter, CompletionCandidate},
  env::Shells,
  generate,
};

use crate::config::{ConfigService, FileConfig};

#[derive(Debug, Parser)]
#[command(name = "git-squad")]
#[command(about = "Manage co-authors in git commit messages with ease", long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  command: Option<Command>,

  /// Use a custom buddy file instead of ~/.config/git-squad/buddies.toml
  #[arg(long = "buddies-file", global = true)]
  pub buddies_file: Option<PathBuf>,
}

impl Cli {
  pub fn new() -> Self {
    CompleteEnv::with_factory(Cli::command).complete();
    Self::parse()
  }

  pub fn get_command(&self) -> Command {
    self.command.clone().unwrap_or(Command::Info)
  }
}

pub fn print_completions(shell: Shell) -> anyhow::Result<()> {
  print_completions_internal(shell, &mut Cli::command())
}

fn print_completions_internal(
  shell: Shell,
  cmd: &mut clap::Command,
) -> anyhow::Result<()> {
  generate(shell, cmd, cmd.get_name().to_string(), &mut io::stdout());

  println!();

  let name = cmd.get_name();
  if let Some(completer) =
    Shells::builtins().completer(shell.to_string().as_str())
  {
    completer.write_registration(
      "COMPLETE",
      name,
      name,
      name,
      &mut io::stdout(),
    )?;
  }

  Ok(())
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
  /// Add buddies to the current session
  With {
    /// The aliases of the buddies to add
    #[arg( required = true,
               num_args = 1..,
               add = ArgValueCompleter::new(alias_completer))]
    // TODO: I would rather  use NonEmpty<String> here but clap makes
    // this really cumbersome
    aliases: Vec<String>,
  },

  /// Remove buddies from the current session
  Without {
    /// The aliases of the buddies to remove
    #[arg( required = true,
               num_args = 1..,
               add = ArgValueCompleter::new(alias_completer))]
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
    #[arg(add = ArgValueCompleter::new(alias_completer))]
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

  /// Migrate buddies from old yaml format to new toml format
  MigrateBuddies {
    /// Optional location of the old buddies file to use instead of
    /// ~/.config/git-squad/buddies.yaml
    #[arg(long = "old-buddies-file")]
    old_buddies_file: Option<PathBuf>,
  },
}

fn alias_completer(current: &std::ffi::OsStr) -> Vec<CompletionCandidate> {
  // TODO: support completions with custom buddies_file locations
  let conf = FileConfig { buddies_file: None };

  if let Ok(buddies) = conf.load_buddies() {
    let current = current.to_str().unwrap_or_default();
    return buddies
      .buddies
      .iter()
      .filter(|s| {
        s.alias.starts_with(current)
          || s.name.starts_with(current)
          || s.email.starts_with(current)
      })
      .map(|b| {
        let help = Some(StyledStr::from(b.format_buddy()));
        CompletionCandidate::new(b.alias.clone()).help(help)
      })
      .collect();
  }

  vec![]
}
