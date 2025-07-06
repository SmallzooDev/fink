use fink::application::repository::{FileSystemRepository, PromptRepository};
use fink::storage::FileSystem;
use tempfile::tempdir;

#[test]
fn should_get_prompt_content_for_clipboard() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("fink");
    std::fs::create_dir(&prompts_dir).unwrap();

    let prompt_content = r#"---
name: "Test Prompt"
description: "A test prompt"
tags: ["test"]
---
# Test Prompt

This is the content that should be copied to clipboard."#;

    std::fs::write(prompts_dir.join("test.md"), prompt_content).unwrap();

    // Act
    let storage = FileSystem::new(temp_dir.path().to_path_buf());
    let repository = FileSystemRepository::new(storage);
    let content = repository.get_content("test.md").unwrap();

    // Assert
    assert_eq!(
        content,
        "# Test Prompt\n\nThis is the content that should be copied to clipboard."
    );
}
