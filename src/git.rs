use std::{
  fmt::Display,
  fs::File,
  io::{Read, Write},
  path::PathBuf,
  process::Command,
};

use anyhow::{Context, Result};
use itertools::Itertools;
use regex::Regex;

use crate::buddy::Buddies;

// Markers for the git-squad section in the commit template
const BEGIN_MARKER: &str = "# BEGIN GIT-SQUAD";
const END_MARKER: &str = "# END GIT-SQUAD";

pub fn get_commit_template_path() -> Result<PathBuf> {
  let output = Command::new("git")
    .args(["config", "--get", "commit.template"])
    .output()
    .context("Failed to execute git command")?;

  if !output.status.success() {
    anyhow::bail!(
      "No template file set. Configure one using `git config --set \
       commit.template /path/to/template/file"
    )
  }

  let path_str = String::from_utf8(output.stdout)
    .context("Failed to parse git output")?
    .trim()
    .to_string();

  Ok(PathBuf::from(path_str))
}

#[derive(Debug)]
pub struct Author {
  pub name: String,
  pub email: String,
}

impl Display for Author {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} <{}>", self.name, self.email)
  }
}

fn parse_author(input: &str) -> Option<Author> {
  if let (Some(start), Some(end)) = (input.find('<'), input.find('>')) {
    let name = input[1..start].trim().to_string();
    let email = input[start + 1..end].trim().to_string();
    Some(Author { name, email })
  } else {
    None
  }
}

pub fn get_authors() -> Result<Vec<Author>> {
  let output = Command::new("git")
    .args(["log", "--all", "--format='%aN <%aE>'"])
    .output()
    .context("Failed to execute git command")?;

  if !output.status.success() {
    anyhow::bail!("FAIL")
  }

  let authors = String::from_utf8(output.stdout)?
    .lines()
    .unique()
    .filter_map(parse_author)
    .collect();

  Ok(authors)
}

pub fn get_active_buddies(buddies: &Buddies) -> Result<Buddies> {
  let template_path = get_commit_template_path()?;

  if !template_path.exists() {
    return Ok(Buddies::default());
  }

  let mut file = File::open(&template_path)
    .context("Failed to open commit template file")?;

  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .context("Failed to read commit template file")?;

  let co_author_regex = Regex::new(r"Co-authored-by: .* <(.+)>").unwrap();
  let mut active_buddies = Vec::new();

  let mut in_squad_section = false;
  for line in contents.lines() {
    if line.trim() == BEGIN_MARKER {
      in_squad_section = true;
      continue;
    }
    if line.trim() == END_MARKER {
      in_squad_section = false;
      continue;
    }

    if in_squad_section {
      if let Some(captures) = co_author_regex.captures(line) {
        if let Some(email_match) = captures.get(1) {
          let email = email_match.as_str();
          if let Some(buddy) = buddies.get_buddy_by_email(email) {
            active_buddies.push(buddy.clone());
          }
        }
      }
    }
  }

  Ok(Buddies::new(active_buddies))
}

pub fn update_commit_template(active_buddies: &Buddies) -> Result<()> {
  let template_path = get_commit_template_path()?;

  if !template_path.exists() {
    let template_dir = template_path.parent().unwrap();
    if !template_dir.exists() {
      anyhow::bail!("Template dir '{}' doens't exist", template_dir.display());
    }
    File::create(&template_path)
      .context("Failed to create commit template file")?;
  }

  let mut file = File::open(&template_path)
    .context("Failed to open commit template file")?;

  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .context("Failed to read commit template file")?;

  // Extract the content excluding our section
  let mut new_content = String::new();
  let mut skipping = false;

  // Parse the file line by line, skipping our section
  for line in contents.lines() {
    if line.trim() == BEGIN_MARKER {
      skipping = true;
      continue;
    }

    if line.trim() == END_MARKER {
      skipping = false;
      continue;
    }

    if !skipping {
      new_content.push_str(line);
      new_content.push('\n');
    }
  }

  // Trim trailing whitespace
  new_content = new_content.trim_end().to_string();

  // Add our section with co-authors if needed
  if !active_buddies.buddies.is_empty() {
    new_content.push_str("\n\n");
    new_content.push_str(BEGIN_MARKER);
    new_content.push('\n');

    for buddy in &active_buddies.buddies {
      new_content.push_str(&buddy.format_co_author());
      new_content.push('\n');
    }

    new_content.push_str(END_MARKER);
    new_content.push('\n');
  }

  // Write back to file
  let mut file = File::create(&template_path)
    .context("Failed to open commit template file for writing")?;

  file
    .write_all(new_content.as_bytes())
    .context("Failed to write to commit template file")?;

  Ok(())
}
