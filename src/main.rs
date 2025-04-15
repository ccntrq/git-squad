mod buddy;
mod cli;
mod config;
mod git;

use anyhow::Result;
use buddy::{Buddies, Buddy};
use cli::{Cli, Command, print_completions};
#[allow(deprecated)]
use config::{ConfigService, DeprecatedFileConfig, FileConfig};
use std::{
    ffi::OsStr,
    io::{self, Write},
};

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    let cli = Cli::new();

    if let Some(buddies_file) = &cli.buddies_file {
        let ext = buddies_file.extension();
        if ext == Some(OsStr::new("yaml")) || ext == Some(OsStr::new("yml")) {
            anyhow::bail!(
                "Yaml config is deprecated! Please migrate your buddies \
                file using:
  $ git squad migrate-buddies {} --buddies-file=/path/to/buddies.toml",
                buddies_file.display()
            )
        }
    } else {
        #[allow(deprecated)]
        let from = config::DeprecatedFileConfig { buddies_file: None };
        let to = config::FileConfig { buddies_file: None };

        #[allow(deprecated)]
        if from.get_buddies_file()?.exists() && !to.get_buddies_file()?.exists() {
            println!("Yaml config deprecated - running migration");
            migrate_config(&from, &to)?;
        }
    }

    let conf = config::FileConfig {
        buddies_file: cli.buddies_file.clone(),
    };

    match cli.get_command() {
        Command::With { aliases } => {
            let buddies = conf.load_buddies()?;

            let mut active_buddies = git::get_active_buddies(&buddies)?;
            for alias in &aliases {
                match buddies.get(alias) {
                    Some(buddy) => match active_buddies.add(buddy.clone()) {
                        Ok(()) => println!("Added buddy '{alias}' to the current session"),
                        Err(_) => eprintln!("Buddy '{alias}' is already active"),
                    },
                    None => eprintln!("Buddy with alias '{alias}' does not exist"),
                }
            }
            git::update_commit_template(&active_buddies)?;
        }

        Command::Without { aliases } => {
            let buddies = conf.load_buddies()?;

            let mut active_buddies = git::get_active_buddies(&buddies)?;

            for alias in &aliases {
                match active_buddies.forget(alias) {
                    Ok(()) => println!("Removed buddy '{alias}' from the current session"),
                    Err(_) => eprintln!("Buddy '{alias}' is not active"),
                }
            }
            git::update_commit_template(&active_buddies)?;
        }

        Command::Alone => {
            git::update_commit_template(&Buddies::default())?;
            println!("Removed all buddies from the current session");
        }

        Command::Create { alias } => {
            let mut buddies = conf.load_buddies()?;

            if buddies.has(&alias) {
                anyhow::bail!("Buddy with alias '{}' already exists", alias)
            }

            print!("Enter name for buddy '{alias}': ");
            io::stdout().flush()?;
            let mut name = String::new();
            io::stdin().read_line(&mut name)?;
            name = name.trim().to_string();

            print!("Enter email for buddy '{alias}': ");
            io::stdout().flush()?;
            let mut email = String::new();
            io::stdin().read_line(&mut email)?;
            email = email.trim().to_string();

            buddies.add(Buddy {
                alias: alias.clone(),
                name,
                email,
            })?;
            conf.save_buddies(&buddies)?;
            println!("Created new buddy '{alias}'");
        }

        Command::Forget { alias } => {
            let mut buddies = conf.load_buddies()?;

            let mut active_buddies = git::get_active_buddies(&buddies)?;

            let _ = active_buddies.forget(&alias);
            git::update_commit_template(&active_buddies)?;

            buddies.forget(&alias)?;
            conf.save_buddies(&buddies)?;

            println!("Completly forgot buddy '{alias}'");
        }

        Command::Info => {
            command_active(&conf)?;
            command_list(&conf)?;
        }

        Command::List => command_list(&conf)?,

        Command::Active => command_active(&conf)?,

        Command::Completions { shell } => print_completions(shell)?,

        Command::MigrateBuddies { old_buddies_file } => {
            #[allow(deprecated)]
            let from = config::DeprecatedFileConfig {
                buddies_file: old_buddies_file.clone(),
            };

            migrate_config(&from, &conf)?;
        }
    }

    Ok(())
}

#[allow(deprecated)]
fn migrate_config(from: &DeprecatedFileConfig, to: &FileConfig) -> Result<()> {
    let old = from.get_buddies_file()?;
    println!("Migrating {:#?} to {:#?}", old, to.get_buddies_file()?);

    from.migrate(to)?;

    println!("Migration successful");

    Ok(())
}

fn command_list(conf: &impl ConfigService) -> Result<()> {
    let buddies = conf.load_buddies()?;

    if buddies.buddies.is_empty() {
        println!("No buddies found.");
        return Ok(());
    }

    println!("Available buddies:");
    for buddy in &buddies.buddies {
        println!("- {} ({} <{}>)", buddy.alias, buddy.name, buddy.email);
    }

    Ok(())
}

fn command_active(conf: &impl ConfigService) -> Result<()> {
    let buddies = conf.load_buddies()?;
    let active_buddies = git::get_active_buddies(&buddies)?;

    if active_buddies.buddies.is_empty() {
        println!("No active buddies in the current session.");
        return Ok(());
    }

    println!("Active buddies in the current session:");
    for buddy in &active_buddies.buddies {
        println!("- {} ({} <{}>)", buddy.alias, buddy.name, buddy.email);
    }

    Ok(())
}
