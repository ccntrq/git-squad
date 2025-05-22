use common::{
  create_test_buddies, make_test_command, setup_git_repo,
};
use insta_cmd::assert_cmd_snapshot;

mod common;

#[test]
fn test_active_empty() -> Result<(), Box<dyn std::error::Error>> {
  let test_bed = setup_git_repo()?;
  let mut cmd = make_test_command(&test_bed, ["active"])?;

  assert_cmd_snapshot!(cmd, @r"
  success: true
  exit_code: 0
  ----- stdout -----
  No active buddies in the current session.

  ----- stderr -----
  ");

  Ok(())
}

#[test]
fn test_active() -> Result<(), Box<dyn std::error::Error>> {
  let test_bed = setup_git_repo()?;
  create_test_buddies(&test_bed)?;

  let mut cmd = make_test_command(&test_bed, ["with", "pp"])?;
  let proc = cmd.spawn()?;
  proc.wait_with_output()?;

  let mut cmd = make_test_command(&test_bed, ["with", "wd"])?;
  let proc = cmd.spawn()?;
  proc.wait_with_output()?;

  let mut cmd = make_test_command(&test_bed, ["active"])?;

  assert_cmd_snapshot!(cmd, @r"
  success: true
  exit_code: 0
  ----- stdout -----
  Active buddies in the current session:
  - pp (Peter Pan <peter.pan@example.com>)
  - wd (Wendy Darling <wendy.darling@example.com>)

  ----- stderr -----
  ");

  Ok(())
}
