use std::fs;
use tempfile::tempdir;

#[test]
fn should_list_prompts_from_directory() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    fs::create_dir(&prompts_dir).unwrap();

    // Create test prompt files
    let prompt1_content = r#"---
name: "Test Prompt 1"
description: "A test prompt"
tags: ["test"]
---
# Test Prompt 1
This is a test prompt."#;

    fs::write(prompts_dir.join("test1.md"), prompt1_content).unwrap();
    fs::write(prompts_dir.join("test2.md"), "# Test Prompt 2").unwrap();

    // Act
    let storage = fink::storage::FileSystem::new(temp_dir.path().to_path_buf());
    let prompts = storage.list_prompts().unwrap();

    // Assert
    assert_eq!(prompts.len(), 2);
    assert!(prompts.iter().any(|p| p.name == "Test Prompt 1"));
}
