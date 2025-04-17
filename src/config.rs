use std::{
  fs::{self, File},
  io::{Read, Write},
  path::PathBuf,
};

use anyhow::{Context, Result};
use dirs::home_dir;

use crate::buddy::Buddies;

pub trait ConfigService {
  fn load_buddies(&self) -> Result<Buddies>;
  fn save_buddies(&self, buddies: &Buddies) -> Result<()>;
}

pub struct FileConfig {
  pub buddies_file: Option<PathBuf>,
}

impl FileConfig {
  pub fn get_config_dir() -> Result<PathBuf> {
    let home = home_dir().context("Failed to determine home directory")?;
    let config_dir = home.join(".config").join("git-squad");

    if !config_dir.exists() {
      fs::create_dir_all(&config_dir)
        .context("Failed to create config directory")?;
    }

    Ok(config_dir)
  }

  pub fn get_buddies_file(&self) -> Result<PathBuf> {
    Ok(
      self
        .buddies_file
        .clone()
        .unwrap_or(FileConfig::get_config_dir()?.join("buddies.toml")),
    )
  }
}

impl ConfigService for FileConfig {
  fn load_buddies(&self) -> Result<Buddies> {
    let path = self.get_buddies_file()?;

    if !path.exists() {
      return Ok(Buddies::default());
    }

    let mut file = File::open(path).context("Failed to open config file")?;

    let mut contents = String::new();
    file
      .read_to_string(&mut contents)
      .context("Failed to read config file")?;

    if contents.trim().is_empty() {
      return Ok(Buddies::default());
    }

    toml::from_str(&contents).context("Failed to parse config")
  }

  fn save_buddies(&self, config: &Buddies) -> Result<()> {
    let path = self.get_buddies_file()?;

    let contents =
      toml::to_string(config).context("Failed to serialize config")?;

    let mut file =
      File::create(path).context("Failed to create config file")?;

    file
      .write_all(contents.as_bytes())
      .context("Failed to write to buddies file")?;

    Ok(())
  }
}

#[deprecated(
  since = "0.3.0",
  note = "please use toml config with `FileConfig` instead"
)]
pub struct DeprecatedFileConfig {
  pub buddies_file: Option<PathBuf>,
}

#[allow(deprecated)]
impl DeprecatedFileConfig {
  #[deprecated(
    since = "0.3.0",
    note = "please use toml config with `FileConfig::get_config_file` instead"
  )]
  pub fn get_buddies_file(&self) -> Result<PathBuf> {
    Ok(
      self
        .buddies_file
        .clone()
        .unwrap_or(FileConfig::get_config_dir()?.join("buddies.yaml")),
    )
  }

  pub fn migrate(&self, to: &FileConfig) -> Result<()> {
    let buddies = self.load_buddies()?;

    to.save_buddies(&buddies)
  }
}

#[allow(deprecated)]
impl ConfigService for DeprecatedFileConfig {
  fn load_buddies(&self) -> Result<Buddies> {
    let path = self.get_buddies_file()?;

    if !path.exists() {
      return Ok(Buddies::default());
    }

    let mut file = File::open(path).context("Failed to open buddies file")?;

    let mut contents = String::new();
    file
      .read_to_string(&mut contents)
      .context("Failed to read buddies file")?;

    if contents.trim().is_empty() {
      return Ok(Buddies::default());
    }

    serde_yaml::from_str(&contents).context("Failed to parse buddies file")
  }

  fn save_buddies(&self, buddies: &Buddies) -> Result<()> {
    let path = self.get_buddies_file()?;

    let contents =
      serde_yaml::to_string(buddies).context("Failed to serialize buddies")?;

    let mut file =
      File::create(path).context("Failed to create buddies file")?;

    file
      .write_all(contents.as_bytes())
      .context("Failed to write to buddies file")?;

    Ok(())
  }
}
