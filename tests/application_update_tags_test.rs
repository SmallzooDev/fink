use fink::application::traits::PromptApplication;
use fink::application::application::DefaultPromptApplication;
use tempfile::TempDir;
use std::fs;
use std::path::PathBuf;

fn setup_test_app() -> (TempDir, DefaultPromptApplication) {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path().to_path_buf();
    
    // Create the jkms directory
    let jkms_dir = base_path.join("prompts");
    fs::create_dir_all(&jkms_dir).unwrap();
    
    let app = DefaultPromptApplication::new(base_path).unwrap();
    (temp_dir, app)
}

fn create_test_prompt(base_path: &PathBuf, name: &str, content: &str) {
    let file_path = base_path.join("prompts").join(format!("{}.md", name));
    fs::write(file_path, content).unwrap();
}

#[test]
fn test_update_tags_with_existing_frontmatter() {
    let (_temp_dir, app) = setup_test_app();
    let base_path = app.get_base_path().to_path_buf();
    
    // Create a prompt with existing frontmatter
    let content = r#"---
name: "test-prompt"
description: "A test prompt"
tags: ["old-tag"]
---
# Test Prompt
Content here"#;
    
    create_test_prompt(&base_path, "test-prompt", content);
    
    // Update tags
    let new_tags = vec!["new-tag1".to_string(), "new-tag2".to_string()];
    app.update_prompt_tags("test-prompt", new_tags.clone()).unwrap();
    
    // Read the file and verify
    let file_path = base_path.join("prompts").join("test-prompt.md");
    let updated_content = fs::read_to_string(file_path).unwrap();
    
    assert!(updated_content.contains(r#"tags: ["new-tag1", "new-tag2"]"#));
    assert!(!updated_content.contains("old-tag"));
    assert!(updated_content.contains("# Test Prompt"));
}

#[test]
fn test_update_tags_without_existing_tags() {
    let (_temp_dir, app) = setup_test_app();
    let base_path = app.get_base_path().to_path_buf();
    
    // Create a prompt without tags
    let content = r#"---
name: "test-prompt"
description: "A test prompt"
---
# Test Prompt
Content here"#;
    
    create_test_prompt(&base_path, "test-prompt", content);
    
    // Update tags
    let new_tags = vec!["tag1".to_string(), "tag2".to_string()];
    app.update_prompt_tags("test-prompt", new_tags.clone()).unwrap();
    
    // Read the file and verify
    let file_path = base_path.join("prompts").join("test-prompt.md");
    let updated_content = fs::read_to_string(file_path).unwrap();
    
    assert!(updated_content.contains(r#"tags: ["tag1", "tag2"]"#));
    assert!(updated_content.contains("description: \"A test prompt\""));
}

#[test]
fn test_update_tags_without_frontmatter() {
    let (_temp_dir, app) = setup_test_app();
    let base_path = app.get_base_path().to_path_buf();
    
    // Create a prompt without frontmatter
    let content = r#"# Test Prompt
Just some content without frontmatter"#;
    
    create_test_prompt(&base_path, "test-prompt", content);
    
    // Update tags
    let new_tags = vec!["tag1".to_string()];
    app.update_prompt_tags("test-prompt", new_tags.clone()).unwrap();
    
    // Read the file and verify
    let file_path = base_path.join("prompts").join("test-prompt.md");
    let updated_content = fs::read_to_string(file_path).unwrap();
    
    assert!(updated_content.starts_with("---\n"));
    assert!(updated_content.contains(r#"name: "test-prompt""#));
    assert!(updated_content.contains(r#"tags: ["tag1"]"#));
    assert!(updated_content.contains("# Test Prompt"));
}

#[test]
fn test_update_tags_with_empty_tags() {
    let (_temp_dir, app) = setup_test_app();
    let base_path = app.get_base_path().to_path_buf();
    
    // Create a prompt with tags
    let content = r#"---
name: "test-prompt"
tags: ["tag1", "tag2"]
---
# Test Prompt"#;
    
    create_test_prompt(&base_path, "test-prompt", content);
    
    // Update with empty tags
    let new_tags = vec![];
    app.update_prompt_tags("test-prompt", new_tags).unwrap();
    
    // Read the file and verify
    let file_path = base_path.join("prompts").join("test-prompt.md");
    let updated_content = fs::read_to_string(file_path).unwrap();
    
    assert!(updated_content.contains("tags: []"));
    assert!(!updated_content.contains("tag1"));
}

#[test]
fn test_update_tags_preserves_other_fields() {
    let (_temp_dir, app) = setup_test_app();
    let base_path = app.get_base_path().to_path_buf();
    
    // Create a prompt with multiple fields
    let content = r#"---
name: "test-prompt"
description: "Important description"
author: "test-author"
tags: ["old"]
custom_field: "custom value"
---
# Test Prompt"#;
    
    create_test_prompt(&base_path, "test-prompt", content);
    
    // Update tags
    let new_tags = vec!["new".to_string()];
    app.update_prompt_tags("test-prompt", new_tags).unwrap();
    
    // Read the file and verify all fields are preserved
    let file_path = base_path.join("prompts").join("test-prompt.md");
    let updated_content = fs::read_to_string(file_path).unwrap();
    
    assert!(updated_content.contains("description: \"Important description\""));
    assert!(updated_content.contains("author: \"test-author\""));
    assert!(updated_content.contains("custom_field: \"custom value\""));
    assert!(updated_content.contains(r#"tags: ["new"]"#));
}

#[test]
fn test_update_tags_nonexistent_prompt() {
    let (_temp_dir, app) = setup_test_app();
    
    // Try to update tags for non-existent prompt
    let result = app.update_prompt_tags("nonexistent", vec!["tag".to_string()]);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}

#[test]
fn test_update_tags_with_special_characters() {
    let (_temp_dir, app) = setup_test_app();
    let base_path = app.get_base_path().to_path_buf();
    
    // Create a prompt
    let content = r#"---
name: "test-prompt"
---
# Test"#;
    
    create_test_prompt(&base_path, "test-prompt", content);
    
    // Update with tags containing special characters
    let new_tags = vec![
        "tag-with-dash".to_string(),
        "tag_with_underscore".to_string(),
        "tag with spaces".to_string(),
    ];
    app.update_prompt_tags("test-prompt", new_tags).unwrap();
    
    // Read and verify
    let file_path = base_path.join("prompts").join("test-prompt.md");
    let updated_content = fs::read_to_string(file_path).unwrap();
    
    assert!(updated_content.contains(r#"tags: ["tag-with-dash", "tag_with_underscore", "tag with spaces"]"#));
}