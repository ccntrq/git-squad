use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use tempfile::{NamedTempFile, TempDir, tempdir};

// Helper function to set up a test environment with a git repository
fn setup_git_repo() -> Result<(TempDir, String, NamedTempFile), Box<dyn std::error::Error>> {
    // Create a temporary directory for git repo
    let repo_dir = tempdir()?;
    let repo_path = repo_dir.path();

    // Create a temporary buddies file
    let buddies_file = NamedTempFile::new()?;

    // Initialize git repository
    Command::new("git")
        .args(["init"])
        .current_dir(repo_path)
        .output()?;

    // Configure git user for the test
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_path)
        .output()?;

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo_path)
        .output()?;

    // Create and set a commit template
    let template_path = repo_path.join("commit-template.txt");
    fs::write(&template_path, "Test commit template\n")?;

    Command::new("git")
        .args(["config", "commit.template", template_path.to_str().unwrap()])
        .current_dir(repo_path)
        .output()?;

    let path = repo_path.to_string_lossy().to_string();

    Ok((repo_dir, path, buddies_file))
}

// Helper function to read commit template content
fn read_commit_template(repo_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(["config", "--get", "commit.template"])
        .current_dir(repo_path)
        .output()?;

    let template_path = String::from_utf8(output.stdout)?.trim().to_string();
    let mut file = File::open(template_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[test]
fn test_create_and_list_buddy() -> Result<(), Box<dyn std::error::Error>> {
    let (_repo_dir, repo_path, buddies_file) = setup_git_repo()?;
    let buddies_path = buddies_file.path().to_string_lossy().to_string();

    // Create a new buddy with simulated input
    let mut cmd = Command::cargo_bin("git-squad")?;
    cmd.args(["--buddies-file", &buddies_path, "create", "peter"])
        .current_dir(&repo_path);

    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(b"Peter Pan\npeter@neverland.com\n")?;
        // stdin is closed when it goes out of scope
    }
    let output = child.wait_with_output()?;
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Created new buddy 'peter'"));

    // List buddies and check that Peter appears
    let assert = Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "list"])
        .current_dir(&repo_path)
        .assert();

    assert.success().stdout(predicate::str::contains(
        "peter (Peter Pan <peter@neverland.com>)",
    ));

    Ok(())
}

#[test]
fn test_with_and_active() -> Result<(), Box<dyn std::error::Error>> {
    let (_repo_dir, repo_path, buddies_file) = setup_git_repo()?;
    let buddies_path = buddies_file.path().to_string_lossy().to_string();

    // Create a buddy
    let mut cmd = Command::cargo_bin("git-squad")?;
    cmd.args(["--buddies-file", &buddies_path, "create", "wendy"])
        .current_dir(&repo_path);

    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(b"Wendy Darling\nwendy@darling.com\n")?;
    }
    let output = child.wait_with_output()?;
    assert!(output.status.success());

    // Add buddy to active session
    let assert = Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "with", "wendy"])
        .current_dir(&repo_path)
        .assert();

    assert.success().stdout(predicate::str::contains(
        "Added buddy 'wendy' to the current session",
    ));

    // Check active buddies
    let assert = Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "active"])
        .current_dir(&repo_path)
        .assert();

    assert.success().stdout(predicate::str::contains(
        "wendy (Wendy Darling <wendy@darling.com>)",
    ));

    // Verify commit template contains co-author
    let template_content = read_commit_template(&repo_path)?;
    assert!(template_content.contains("Co-authored-by: Wendy Darling <wendy@darling.com>"));
    assert!(template_content.contains("# BEGIN GIT-SQUAD"));
    assert!(template_content.contains("# END GIT-SQUAD"));

    Ok(())
}

#[test]
fn test_without_buddy() -> Result<(), Box<dyn std::error::Error>> {
    let (_repo_dir, repo_path, buddies_file) = setup_git_repo()?;
    let buddies_path = buddies_file.path().to_string_lossy().to_string();

    // Create a buddy
    let mut cmd = Command::cargo_bin("git-squad")?;
    cmd.args(["--buddies-file", &buddies_path, "create", "hook"])
        .current_dir(&repo_path);

    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(b"Captain Hook\nhook@jollyroger.com\n")?;
    }
    let output = child.wait_with_output()?;
    assert!(output.status.success());

    // Add buddy to active session
    let assert = Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "with", "hook"])
        .current_dir(&repo_path)
        .assert();

    assert.success();

    // Verify buddy is active
    let template_content = read_commit_template(&repo_path)?;
    assert!(template_content.contains("Co-authored-by: Captain Hook <hook@jollyroger.com>"));

    // Remove buddy from session
    let assert = Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "without", "hook"])
        .current_dir(&repo_path)
        .assert();

    assert.success().stdout(predicate::str::contains(
        "Removed buddy 'hook' from the current session",
    ));

    // Verify buddy is no longer in template
    let template_content = read_commit_template(&repo_path)?;
    assert!(!template_content.contains("Co-authored-by: Captain Hook <hook@jollyroger.com>"));

    Ok(())
}

#[test]
fn test_alone_command() -> Result<(), Box<dyn std::error::Error>> {
    let (_repo_dir, repo_path, buddies_file) = setup_git_repo()?;
    let buddies_path = buddies_file.path().to_string_lossy().to_string();

    // Create two buddies
    let mut cmd = Command::cargo_bin("git-squad")?;
    cmd.args(["--buddies-file", &buddies_path, "create", "peter"])
        .current_dir(&repo_path);

    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(b"Peter Pan\npeter@neverland.com\n")?;
    }
    child.wait_with_output()?;

    let mut cmd = Command::cargo_bin("git-squad")?;
    cmd.args(["--buddies-file", &buddies_path, "create", "wendy"])
        .current_dir(&repo_path);

    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(b"Wendy Darling\nwendy@darling.com\n")?;
    }
    child.wait_with_output()?;

    // Add both buddies to active session
    Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "with", "peter"])
        .current_dir(&repo_path)
        .assert()
        .success();

    Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "with", "wendy"])
        .current_dir(&repo_path)
        .assert()
        .success();

    // Verify both buddies are active
    let template_content = read_commit_template(&repo_path)?;
    assert!(template_content.contains("Co-authored-by: Peter Pan <peter@neverland.com>"));
    assert!(template_content.contains("Co-authored-by: Wendy Darling <wendy@darling.com>"));

    // Run alone command
    Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "alone"])
        .current_dir(&repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Removed all buddies from the current session",
        ));

    // Verify no buddies are active
    let template_content = read_commit_template(&repo_path)?;
    assert!(!template_content.contains("Co-authored-by: Peter Pan <peter@neverland.com>"));
    assert!(!template_content.contains("Co-authored-by: Wendy Darling <wendy@darling.com>"));
    assert!(!template_content.contains("# BEGIN GIT-SQUAD"));

    Ok(())
}

#[test]
fn test_forget_buddy() -> Result<(), Box<dyn std::error::Error>> {
    let (_repo_dir, repo_path, buddies_file) = setup_git_repo()?;
    let buddies_path = buddies_file.path().to_string_lossy().to_string();

    // Create a buddy
    let mut cmd = Command::cargo_bin("git-squad")?;
    cmd.args(["--buddies-file", &buddies_path, "create", "hook"])
        .current_dir(&repo_path);

    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(b"Captain Hook\nhook@jollyroger.com\n")?;
    }
    child.wait_with_output()?;

    // Add buddy to active session
    Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "with", "hook"])
        .current_dir(&repo_path)
        .assert()
        .success();

    // Forget buddy
    Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "forget", "hook"])
        .current_dir(&repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Completly forgot buddy 'hook'"));

    // Verify buddy is gone from list
    Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "list"])
        .current_dir(&repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("No buddies found."));

    // Verify buddy is gone from template
    let template_content = read_commit_template(&repo_path)?;
    assert!(!template_content.contains("Co-authored-by: Captain Hook <hook@jollyroger.com>"));

    Ok(())
}

#[test]
fn test_info_command() -> Result<(), Box<dyn std::error::Error>> {
    let (_repo_dir, repo_path, buddies_file) = setup_git_repo()?;
    let buddies_path = buddies_file.path().to_string_lossy().to_string();

    // Create a buddy
    let mut cmd = Command::cargo_bin("git-squad")?;
    cmd.args(["--buddies-file", &buddies_path, "create", "peter"])
        .current_dir(&repo_path);

    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(b"Peter Pan\npeter@neverland.com\n")?;
    }
    child.wait_with_output()?;

    // Add buddy to active session
    Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "with", "peter"])
        .current_dir(&repo_path)
        .assert()
        .success();

    // Run info command
    Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "info"])
        .current_dir(&repo_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Active buddies in the current session:",
        ))
        .stdout(predicate::str::contains(
            "peter (Peter Pan <peter@neverland.com>)",
        ))
        .stdout(predicate::str::contains("Available buddies:"))
        .stdout(predicate::str::contains(
            "peter (Peter Pan <peter@neverland.com>)",
        ));

    Ok(())
}

#[test]
fn test_with_nonexistent_buddy() -> Result<(), Box<dyn std::error::Error>> {
    let (_repo_dir, repo_path, buddies_file) = setup_git_repo()?;
    let buddies_path = buddies_file.path().to_string_lossy().to_string();

    // Try to add non-existent buddy
    Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "with", "crocodile"])
        .current_dir(&repo_path)
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Buddy with alias 'crocodile' does not exist",
        ));

    Ok(())
}

#[test]
fn test_without_nonexistent_buddy() -> Result<(), Box<dyn std::error::Error>> {
    let (_repo_dir, repo_path, buddies_file) = setup_git_repo()?;
    let buddies_path = buddies_file.path().to_string_lossy().to_string();

    // Try to remove non-existent buddy
    Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "without", "crocodile"])
        .current_dir(&repo_path)
        .assert()
        .success()
        .stderr(predicate::str::contains("Buddy 'crocodile' is not active"));

    Ok(())
}


#[test]
fn test_with_no_buddy() -> Result<(), Box<dyn std::error::Error>> {
    let (_repo_dir, repo_path, buddies_file) = setup_git_repo()?;
    let buddies_path = buddies_file.path().to_string_lossy().to_string();

    // Try to remove non-existent buddy
    Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "with"])
        .current_dir(&repo_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: the following required arguments were not provided:\n  <ALIASES>",
        ));

    Ok(())
}

#[test]
fn test_without_no_buddy() -> Result<(), Box<dyn std::error::Error>> {
    let (_repo_dir, repo_path, buddies_file) = setup_git_repo()?;
    let buddies_path = buddies_file.path().to_string_lossy().to_string();

    // Try to remove non-existent buddy
    Command::cargo_bin("git-squad")?
        .args(["--buddies-file", &buddies_path, "without"])
        .current_dir(&repo_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: the following required arguments were not provided:\n  <ALIASES>",
        ));

    Ok(())
}

#[test]
fn test_create_duplicate_buddy() -> Result<(), Box<dyn std::error::Error>> {
    let (_repo_dir, repo_path, buddies_file) = setup_git_repo()?;
    let buddies_path = buddies_file.path().to_string_lossy().to_string();

    // Create a buddy
    let mut cmd = Command::cargo_bin("git-squad")?;
    cmd.args(["--buddies-file", &buddies_path, "create", "peter"])
        .current_dir(&repo_path);

    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(b"Peter Pan\npeter@neverland.com\n")?;
    }
    child.wait_with_output()?;

    // Try to create a buddy with the same alias
    let mut cmd = Command::cargo_bin("git-squad")?;
    cmd.args(["--buddies-file", &buddies_path, "create", "peter"])
        .current_dir(&repo_path);

    let child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;
    let error_output = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    assert!(
        error_output.contains("Buddy with alias 'peter' already exists"),
        "Expected error message about duplicate buddy, got: {error_output}"
    );

    Ok(())
}
