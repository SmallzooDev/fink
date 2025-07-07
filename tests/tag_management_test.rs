use fink::presentation::tui::tui::TUIApp;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_add_tag_to_prompt() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create a test prompt file
    let prompt_content = r#"---
name: "test-prompt"
tags: ["existing", "tags"]
---
# Test Prompt
This is a test prompt."#;
    
    fs::write(jkms_path.join("test-prompt.md"), prompt_content).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Select the prompt
    assert_eq!(app.get_prompts().len(), 1);
    
    // Add a new tag
    app.add_tag_to_selected("newtag").unwrap();
    
    // Verify the tag was added
    let prompts = app.get_prompts();
    assert_eq!(prompts[0].tags.len(), 3);
    assert!(prompts[0].tags.contains(&"existing".to_string()));
    assert!(prompts[0].tags.contains(&"tags".to_string()));
    assert!(prompts[0].tags.contains(&"newtag".to_string()));
}

#[test]
fn test_remove_tag_from_prompt() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create a test prompt file
    let prompt_content = r#"---
name: "test-prompt"
tags: ["tag1", "tag2", "tag3"]
---
# Test Prompt
This is a test prompt."#;
    
    fs::write(jkms_path.join("test-prompt.md"), prompt_content).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Remove a tag
    app.remove_tag_from_selected("tag2").unwrap();
    
    // Verify the tag was removed
    let prompts = app.get_prompts();
    assert_eq!(prompts[0].tags.len(), 2);
    assert!(prompts[0].tags.contains(&"tag1".to_string()));
    assert!(!prompts[0].tags.contains(&"tag2".to_string()));
    assert!(prompts[0].tags.contains(&"tag3".to_string()));
}

#[test]
fn test_add_duplicate_tag() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create a test prompt file
    let prompt_content = r#"---
name: "test-prompt"
tags: ["existing"]
---
# Test Prompt
This is a test prompt."#;
    
    fs::write(jkms_path.join("test-prompt.md"), prompt_content).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Try to add a duplicate tag
    let result = app.add_tag_to_selected("existing");
    
    // Should fail or be ignored (tags should remain unique)
    assert!(result.is_err() || app.get_prompts()[0].tags.len() == 1);
}

#[test]
fn test_remove_nonexistent_tag() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create a test prompt file
    let prompt_content = r#"---
name: "test-prompt"
tags: ["tag1"]
---
# Test Prompt
This is a test prompt."#;
    
    fs::write(jkms_path.join("test-prompt.md"), prompt_content).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Try to remove a non-existent tag
    let result = app.remove_tag_from_selected("nonexistent");
    
    // Should fail gracefully
    assert!(result.is_err() || app.get_prompts()[0].tags.len() == 1);
}

#[test]
fn test_tag_persistence() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create a test prompt file
    let prompt_content = r#"---
name: "test-prompt"
tags: ["original"]
---
# Test Prompt
This is a test prompt."#;
    
    let file_path = jkms_path.join("test-prompt.md");
    fs::write(&file_path, prompt_content).unwrap();
    
    // Create TUIApp and add a tag
    {
        let mut app = TUIApp::new(temp_path.clone()).unwrap();
        app.add_tag_to_selected("persistent").unwrap();
    }
    
    // Read the file to verify persistence
    let saved_content = fs::read_to_string(&file_path).unwrap();
    assert!(saved_content.contains("original"));
    assert!(saved_content.contains("persistent"));
    
    // Create a new app instance to verify tags are loaded correctly
    let app2 = TUIApp::new(temp_path).unwrap();
    let prompts = app2.get_prompts();
    assert_eq!(prompts[0].tags.len(), 2);
    assert!(prompts[0].tags.contains(&"original".to_string()));
    assert!(prompts[0].tags.contains(&"persistent".to_string()));
}

#[test] 
fn test_tag_management_dialog() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create a test prompt file
    let prompt_content = r#"---
name: "test-prompt"
tags: ["tag1", "tag2"]
---
# Test Prompt"#;
    
    fs::write(jkms_path.join("test-prompt.md"), prompt_content).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Open tag management dialog
    app.open_tag_management();
    
    // Verify dialog is open
    assert!(app.is_tag_management_active());
    
    // Get current tags for selected prompt
    let current_tags = app.get_selected_prompt_tags();
    assert_eq!(current_tags.len(), 2);
    assert!(current_tags.contains(&"tag1".to_string()));
    assert!(current_tags.contains(&"tag2".to_string()));
    
    // Close dialog
    app.close_tag_management();
    assert!(!app.is_tag_management_active());
}