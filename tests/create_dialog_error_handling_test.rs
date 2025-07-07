use fink::presentation::tui::tui::TUIApp;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_create_dialog_error_handling() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create an existing file
    let existing_content = r#"---
name: "existing-prompt"
tags: []
---
# Existing"#;
    fs::write(jkms_path.join("existing-prompt.md"), existing_content).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path.clone()).unwrap();
    
    // Open create dialog
    app.create_new_prompt().unwrap();
    
    // Try to create a prompt with existing name
    if let Some(dialog) = app.get_create_dialog_mut() {
        dialog.add_char('e');
        dialog.add_char('x');
        dialog.add_char('i');
        dialog.add_char('s');
        dialog.add_char('t');
        dialog.add_char('i');
        dialog.add_char('n');
        dialog.add_char('g');
        dialog.add_char(' ');
        dialog.add_char('p');
        dialog.add_char('r');
        dialog.add_char('o');
        dialog.add_char('m');
        dialog.add_char('p');
        dialog.add_char('t');
    }
    
    // Try to confirm creation - should fail
    let result = app.confirm_create();
    assert!(result.is_err());
    
    // Error should contain "already exists"
    assert!(result.unwrap_err().to_string().contains("already exists"));
}

#[test]
fn test_error_message_display() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Set an error message
    app.set_error("Test error message".to_string());
    
    // Verify error state
    assert!(app.has_error());
    assert_eq!(app.get_error_message(), Some("Test error message"));
    
    // Clear error
    app.clear_error();
    assert!(!app.has_error());
    assert_eq!(app.get_error_message(), None);
}