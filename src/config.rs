use anyhow::{Context, Result};
use dirs::home_dir;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::buddy::Buddies;

pub trait ConfigService {
    fn load_buddies(&self) -> Result<Buddies>;
    fn save_buddies(&self, buddies: &Buddies) -> Result<()>;
}

pub struct FileConfig {
    pub buddies_file: Option<PathBuf>,
}

impl FileConfig {
    pub fn get_config_dir(&self) -> Result<PathBuf> {
        let home = home_dir().context("Failed to determine home directory")?;
        let config_dir = home.join(".config").join("git-squad");

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
        }

        Ok(config_dir)
    }

    pub fn get_buddies_file(&self) -> Result<PathBuf> {
        Ok(self
            .buddies_file
            .clone()
            .unwrap_or(self.get_config_dir()?.join("buddies.yaml")))
    }
}

impl ConfigService for FileConfig {
    fn load_buddies(&self) -> Result<Buddies> {
        let path = self.get_buddies_file()?;

        if !path.exists() {
            return Ok(Buddies::default());
        }

        let mut file = File::open(path).context("Failed to open buddies file")?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("Failed to read buddies file")?;

        if contents.trim().is_empty() {
            return Ok(Buddies::default());
        }

        serde_yaml::from_str(&contents).context("Failed to parse buddies file")
    }

    fn save_buddies(&self, buddies: &Buddies) -> Result<()> {
        let path = self.get_buddies_file()?;

        let contents = serde_yaml::to_string(buddies).context("Failed to serialize buddies")?;

        let mut file = File::create(path).context("Failed to create buddies file")?;

        file.write_all(contents.as_bytes())
            .context("Failed to write to buddies file")?;

        Ok(())
    }
}
