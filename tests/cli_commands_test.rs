use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn should_list_prompts_with_list_command() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("jkms");
    std::fs::create_dir(&prompts_dir).unwrap();

    let prompt1 = r#"---
name: "Code Review"
tags: ["code", "review"]
---
# Code Review Template"#;

    let prompt2 = r#"---
name: "Bug Report"
tags: ["bug", "issue"]
---
# Bug Report Template"#;

    std::fs::write(prompts_dir.join("code-review.md"), prompt1).unwrap();
    std::fs::write(prompts_dir.join("bug-report.md"), prompt2).unwrap();

    // Act & Assert
    let mut cmd = Command::cargo_bin("jkms").unwrap();
    cmd.arg("list")
        .arg("--path")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Code Review"))
        .stdout(predicate::str::contains("Bug Report"))
        .stdout(predicate::str::contains("code, review"))
        .stdout(predicate::str::contains("bug, issue"));
}

#[test]
fn should_handle_empty_directory_with_list_command() {
    // Arrange
    let temp_dir = tempdir().unwrap();

    // Act & Assert
    let mut cmd = Command::cargo_bin("jkms").unwrap();
    cmd.arg("list")
        .arg("--path")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("No prompts found"));
}

#[test]
fn should_get_prompt_content_by_name() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("jkms");
    std::fs::create_dir(&prompts_dir).unwrap();

    let prompt = r#"---
name: "Code Review"
tags: ["code", "review"]
---
# Code Review Template

This is the code review template content."#;

    std::fs::write(prompts_dir.join("code-review.md"), prompt).unwrap();

    // Act & Assert
    let mut cmd = Command::cargo_bin("jkms").unwrap();
    cmd.arg("get")
        .arg("code-review")
        .arg("--path")
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("# Code Review Template"))
        .stdout(predicate::str::contains(
            "This is the code review template content.",
        ));
}

#[test]
fn should_handle_nonexistent_prompt() {
    // Arrange
    let temp_dir = tempdir().unwrap();

    // Act & Assert
    let mut cmd = Command::cargo_bin("jkms").unwrap();
    cmd.arg("get")
        .arg("nonexistent")
        .arg("--path")
        .arg(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Prompt not found"));
}
