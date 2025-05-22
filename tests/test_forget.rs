use common::{create_test_buddies, make_test_command, read_buddies_file, setup_git_repo};
use insta_cmd::assert_cmd_snapshot;

mod common;

#[test]
fn test_forget() -> Result<(), Box<dyn std::error::Error>> {
  let test_bed = setup_git_repo()?;
  create_test_buddies(&test_bed)?;

  let mut cmd = make_test_command(&test_bed, ["forget", "pp"])?;
  assert_cmd_snapshot!(cmd, @r"
  success: true
  exit_code: 0
  ----- stdout -----
  Completly forgot buddy 'pp'

  ----- stderr -----
  ");


  let buddies = read_buddies_file(&test_bed)?;
  insta::assert_snapshot!(buddies, @r#"
  [[buddies]]
  alias = "wd"
  name = "Wendy Darling"
  email = "wendy.darling@example.com"
  "#);

  Ok(())
}

#[test]
fn test_forget_non_existing() -> Result<(), Box<dyn std::error::Error>> {
  let test_bed = setup_git_repo()?;

  let mut cmd = make_test_command(&test_bed, ["forget", "pp"])?;
  assert_cmd_snapshot!(cmd, @r"
  success: false
  exit_code: 1
  ----- stdout -----

  ----- stderr -----
  Error: Buddy with alias 'pp' doesn't exist
  ");

  Ok(())
}
