use common::{
  create_test_buddies, make_test_command, read_commit_template, setup_git_repo
};
use insta::assert_snapshot;
use insta_cmd::assert_cmd_snapshot;

mod common;


#[test]
fn test_alone() -> Result<(), Box<dyn std::error::Error>> {
  let test_bed = setup_git_repo()?;
  create_test_buddies(&test_bed)?;

  let mut cmd = make_test_command(&test_bed, ["with", "pp"])?;
  let proc = cmd.spawn()?;
  proc.wait_with_output()?;

  let mut cmd = make_test_command(&test_bed, ["with", "wd"])?;
  let proc = cmd.spawn()?;
  proc.wait_with_output()?;

  let mut cmd = make_test_command(&test_bed, ["alone"])?;

  assert_cmd_snapshot!(cmd, @r"
  success: true
  exit_code: 0
  ----- stdout -----
  Removed all buddies from the current session

  ----- stderr -----
  ");

  let commit_template = read_commit_template(&test_bed)?;
  assert_snapshot!(commit_template, @"Test commit template");

  Ok(())
}
