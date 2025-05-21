use common::{
  create_test_buddies, make_test_command, setup_git_repo,
};
use insta_cmd::assert_cmd_snapshot;

mod common;

#[test]
fn test_list_empty() -> Result<(), Box<dyn std::error::Error>> {
  let test_bed = setup_git_repo()?;
  let mut cmd = make_test_command(&test_bed, ["list"])?;

  assert_cmd_snapshot!(cmd, @r"
  success: true
  exit_code: 0
  ----- stdout -----
  No buddies found.

  ----- stderr -----
  ");

  Ok(())
}

#[test]
fn test_list() -> Result<(), Box<dyn std::error::Error>> {
  let test_bed = setup_git_repo()?;
  create_test_buddies(&test_bed)?;

  let mut cmd = make_test_command(&test_bed, ["list"])?;

  assert_cmd_snapshot!(cmd, @r"
  success: true
  exit_code: 0
  ----- stdout -----
  Available buddies:
  - pp (Peter Pan <peter.pan@example.com>)
  - wd (Wendy Darling <wendy.darling@example.com>)

  ----- stderr -----
  ");

  Ok(())
}
