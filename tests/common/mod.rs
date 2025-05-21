#![allow(dead_code)]
use std::{
  ffi::OsStr,
  fs::{self, File},
  io::Read,
  path::Path,
  process::Command,
};

use assert_cmd::prelude::*;
use rexpect::session::{PtySession, spawn_command};
use tempfile::{NamedTempFile, TempDir, tempdir};

pub struct TestBed {
  test_dir: TempDir,
  buddies_file: NamedTempFile,
}

pub fn setup_git_repo() -> Result<TestBed, Box<dyn std::error::Error>> {
  let repo_dir = tempdir()?;
  let repo_path = repo_dir.path();

  let buddies_file = NamedTempFile::new()?;

  Command::new("git")
    .args(["init"])
    .current_dir(repo_path)
    .output()?;

  Command::new("git")
    .args(["config", "user.name", "Test User"])
    .current_dir(repo_path)
    .output()?;

  Command::new("git")
    .args(["config", "user.email", "test@example.com"])
    .current_dir(repo_path)
    .output()?;

  let template_path = repo_path.join("commit-template.txt");
  fs::write(&template_path, "Test commit template\n")?;

  Command::new("git")
    .args(["config", "commit.template", template_path.to_str().unwrap()])
    .current_dir(repo_path)
    .output()?;

  Ok(TestBed {
    test_dir: repo_dir,
    buddies_file,
  })
}

pub fn read_commit_template(
  test_bed: &TestBed,
) -> Result<String, Box<dyn std::error::Error>> {
  let test_dir_path = test_bed.test_dir.path().to_string_lossy().to_string();
  let output = Command::new("git")
    .args(["config", "--get", "commit.template"])
    .current_dir(test_dir_path)
    .output()?;

  let template_path = String::from_utf8(output.stdout)?.trim().to_string();
  slurp(template_path)
}

pub fn read_buddies_file(
  test_bed: &TestBed,
) -> Result<String, Box<dyn std::error::Error>> {
  let buddies_path = test_bed.buddies_file.path().to_string_lossy().to_string();
  slurp(buddies_path)
}

pub fn spawn_test_command<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(
  test_bed: &TestBed,
  args: I,
) -> Result<PtySession, Box<dyn std::error::Error>> {
  let cmd = make_test_command(test_bed, args)?;
  Ok(spawn_command(cmd, Some(1000))?)
}

pub fn make_test_command<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(
  test_bed: &TestBed,
  args: I,
) -> Result<Command, Box<dyn std::error::Error>> {
  let test_dir_path = test_bed.test_dir.path().to_string_lossy().to_string();
  let buddies_path = test_bed.buddies_file.path().to_string_lossy().to_string();

  let mut cmd = Command::cargo_bin("git-squad")?;

  cmd
    .args(["--buddies-file", &buddies_path])
    .args(args)
    .current_dir(&test_dir_path);

  Ok(cmd)
}

pub fn create_buddy(
  test_bed: &TestBed,
  alias: &str,
  name: &str,
  mail: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let mut proc = spawn_test_command(test_bed, ["create", alias])?;

  proc.exp_string("Enter name")?;
  proc.send_line(name)?;
  proc.exp_string("Enter email")?;
  proc.send_line(mail)?;
  proc.exp_string("Created new buddy")?;
  proc.exp_eof()?;

  Ok(())
}

pub fn create_test_buddies(
  test_bed: &TestBed,
) -> Result<(), Box<dyn std::error::Error>> {
  create_buddy(test_bed, "pp", "Peter Pan", "peter.pan@example.com")?;
  create_buddy(
    test_bed,
    "wd",
    "Wendy Darling",
    "wendy.darling@example.com",
  )?;
  Ok(())
}

fn slurp<P>(path: P) -> Result<String, Box<dyn std::error::Error>>
where
  P: AsRef<Path>,
{
  let mut file = File::open(path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}
