use fink::storage::FileSystem;
use fink::application::repository::{FileSystemRepository, PromptRepository};
use fink::utils::default_prompts::initialize_default_prompts;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_prompts_should_be_stored_in_prompts_directory_not_fink() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    // The prompts should be stored in ~/.fink/prompts/, not ~/.fink/fink/
    let prompts_dir = base_path.join("prompts");
    fs::create_dir_all(&prompts_dir).unwrap();
    
    // Create a test prompt
    let prompt_content = r#"---
name: "test-prompt"
description: "Test prompt"
tags: ["test"]
type: "whole"
---
Test content"#;
    
    fs::write(prompts_dir.join("test-prompt.md"), prompt_content).unwrap();
    
    // FileSystem should find prompts in the prompts directory
    let fs = FileSystem::new(base_path.to_path_buf());
    let prompts = fs.list_prompts().unwrap();
    
    assert_eq!(prompts.len(), 1, "Should find the prompt in prompts directory");
    assert_eq!(prompts[0].name, "test-prompt");
}

#[test]
fn test_initialization_should_use_prompts_directory() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    let prompts_dir = base_path.join("prompts");
    
    // Initialize default prompts
    initialize_default_prompts(&prompts_dir).unwrap();
    
    // Check that prompts were created in the correct directory
    assert!(prompts_dir.exists(), "prompts directory should exist");
    assert!(prompts_dir.join(".initialized").exists(), "initialization flag should be in prompts directory");
    
    // Verify prompts were created
    let entries: Vec<_> = fs::read_dir(&prompts_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .collect();
    
    assert!(!entries.is_empty(), "Should have created prompt files");
}

#[test]
fn test_repository_should_use_prompts_directory() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    let fs = FileSystem::new(base_path.to_path_buf());
    let repo = FileSystemRepository::new(fs);
    
    // Create a prompt
    let content = r#"---
name: "test"
description: "Test prompt"
tags: ["test"]
type: "whole"
---
Test content"#;
    
    repo.create_prompt("test", content).unwrap();
    
    // Check it was created in the prompts directory
    let prompt_file = base_path.join("prompts").join("test.md");
    assert!(prompt_file.exists(), "Prompt should be created in prompts directory");
}

#[test]
fn test_migration_from_old_fink_directory() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();
    
    // Create old structure with prompts in fink directory
    let old_dir = base_path.join("fink");
    fs::create_dir_all(&old_dir).unwrap();
    
    let old_prompt = r#"---
name: "old-prompt"
description: "Old prompt"
tags: ["old"]
---
Old content"#;
    
    fs::write(old_dir.join("old-prompt.md"), old_prompt).unwrap();
    fs::write(old_dir.join(".initialized"), "").unwrap();
    
    // TODO: When migration is implemented, it should move files from fink/ to prompts/
    // For now, this test documents the expected behavior
    
    // After migration:
    // let prompts_dir = base_path.join("prompts");
    // assert!(prompts_dir.join("old-prompt.md").exists(), "Old prompts should be migrated");
    // assert!(prompts_dir.join(".initialized").exists(), "Init flag should be migrated");
    // assert!(!old_dir.exists(), "Old directory should be removed after migration");
}