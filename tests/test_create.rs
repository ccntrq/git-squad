use common::{read_buddies_file, setup_git_repo, spawn_test_command};

mod common;

#[test]
fn test_create() -> Result<(), Box<dyn std::error::Error>> {
  let test_bed = setup_git_repo()?;
  let mut proc = spawn_test_command(&test_bed, ["create", "peter"])?;

  proc.exp_string("Enter name")?;
  proc.send_line("Peter Pan")?;
  proc.exp_string("Enter email")?;
  proc.send_line("peter.pan@example.com")?;
  proc.exp_string("Created new buddy")?;
  proc.exp_eof()?;

  let buddies = read_buddies_file(&test_bed)?;

  insta::assert_snapshot!(buddies, @r#"
  [[buddies]]
  alias = "peter"
  name = "Peter Pan"
  email = "peter.pan@example.com"
  "#);

  Ok(())
}
