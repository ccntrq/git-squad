mod buddy;
mod cli;
mod config;
mod git;

use anyhow::{Context, Result};
use buddy::{Buddies, Buddy};
use cli::{Cli, Command, print_completions};
use config::ConfigService;
use std::io::{self, Write};

fn main() -> Result<()> {
    let cli = cli::parse();

    match cli.get_command() {
        Command::With { alias } => {
            let conf = config::FileConfig {
                buddies_file: cli.buddies_file.clone(),
            };
            let buddies = conf.load_buddies()?;

            if let Some(buddy) = buddies.get(&alias) {
                let mut active_buddies = git::get_active_buddies(&buddies)?;

                active_buddies
                    .add(buddy.clone())
                    .context(format!("Buddy '{}' is already active", alias))?;
                git::update_commit_template(&active_buddies)?;
                println!("Added buddy '{}' to the current session", alias);
                return Ok(());
            }

            anyhow::bail!("Buddy with alias '{}' does not exist", alias);
        }

        Command::Without { alias } => {
            let conf = config::FileConfig {
                buddies_file: cli.buddies_file.clone(),
            };
            let buddies = conf.load_buddies()?;

            let mut active_buddies = git::get_active_buddies(&buddies)?;

            active_buddies
                .forget(&alias)
                .context(format!("Buddy '{}' is not active", alias))?;

            git::update_commit_template(&active_buddies)?;
            println!("Removed buddy '{}' from the current session", alias);
        }

        Command::Alone => {
            git::update_commit_template(&Buddies::default())?;
            println!("Removed all buddies from the current session");
        }

        Command::Create { alias } => {
            let conf = config::FileConfig {
                buddies_file: cli.buddies_file.clone(),
            };
            let mut buddies = conf.load_buddies()?;

            print!("Enter name for buddy '{}': ", alias);
            io::stdout().flush()?;
            let mut name = String::new();
            io::stdin().read_line(&mut name)?;
            name = name.trim().to_string();

            print!("Enter email for buddy '{}': ", alias);
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
            println!("Created new buddy '{}'", alias);
        }

        Command::Forget { alias } => {
            let conf = config::FileConfig {
                buddies_file: cli.buddies_file.clone(),
            };
            let mut buddies = conf.load_buddies()?;

            let mut active_buddies = git::get_active_buddies(&buddies)?;

            let _ = active_buddies.forget(&alias);
            git::update_commit_template(&active_buddies)?;

            buddies.forget(&alias)?;
            conf.save_buddies(&buddies)?;

            println!("Completly forgot buddy '{}'", alias);
        }

        Command::Info => {
            command_active(&cli)?;
            command_list(&cli)?;
        }

        Command::List => command_list(&cli)?,

        Command::Active => command_active(&cli)?,

        Command::Completions { shell } => {
            print_completions(shell)
        },
    }

    Ok(())
}

fn command_list(cli: &Cli) -> Result<()> {
    let conf = config::FileConfig {
        buddies_file: cli.buddies_file.clone(),
    };
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

fn command_active(cli: &Cli) -> Result<()> {
    let conf = config::FileConfig {
        buddies_file: cli.buddies_file.clone(),
    };
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
