use fink::presentation::tui::tui::TUIApp;
use fink::presentation::tui::components::{CreateDialog, CreateTemplate};
use tempfile::tempdir;
use std::fs;

#[test]
fn test_create_dialog_integration() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path.clone()).unwrap();
    
    // Open create dialog
    app.create_new_prompt().unwrap();
    
    // Verify dialog is open
    assert!(app.is_create_dialog_active());
    assert!(app.get_create_dialog().is_some());
}

#[test]
fn test_create_prompt_with_dialog() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path.clone()).unwrap();
    
    // Open create dialog
    app.create_new_prompt().unwrap();
    
    // Simulate user input
    if let Some(dialog) = app.get_create_dialog_mut() {
        dialog.add_char('t');
        dialog.add_char('e');
        dialog.add_char('s');
        dialog.add_char('t');
        dialog.add_char('-');
        dialog.add_char('p');
        dialog.add_char('r');
        dialog.add_char('o');
        dialog.add_char('m');
        dialog.add_char('p');
        dialog.add_char('t');
    }
    
    // Confirm creation
    app.confirm_create().unwrap();
    
    // Verify dialog is closed
    assert!(!app.is_create_dialog_active());
    
    // Verify prompt was created
    let created_file = jkms_path.join("test-prompt.md");
    assert!(created_file.exists());
    
    // Verify prompt content
    let content = fs::read_to_string(&created_file).unwrap();
    assert!(content.contains("name: \"test-prompt\""));
}

#[test]
fn test_create_prompt_with_template() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path.clone()).unwrap();
    
    // Open create dialog
    app.create_new_prompt().unwrap();
    
    // Simulate user input and template selection
    if let Some(dialog) = app.get_create_dialog_mut() {
        dialog.add_char('w');
        dialog.add_char('i');
        dialog.add_char('t');
        dialog.add_char('h');
        dialog.add_char('-');
        dialog.add_char('t');
        dialog.add_char('e');
        dialog.add_char('m');
        dialog.add_char('p');
        dialog.add_char('l');
        dialog.add_char('a');
        dialog.add_char('t');
        dialog.add_char('e');
        
        // Switch to template field and select basic template
        dialog.next_field(); // Go to Type
        dialog.next_field(); // Go to Template
        dialog.next_template(); // FromClipboard -> Default
        dialog.next_template(); // Default -> Basic
    }
    
    // Confirm creation
    app.confirm_create().unwrap();
    
    // Verify prompt was created with template
    let created_file = jkms_path.join("with-template.md");
    assert!(created_file.exists());
    
    let content = fs::read_to_string(&created_file).unwrap();
    assert!(content.contains("name: \"with-template\""));
    // Should have template content
    assert!(content.contains("# Instruction"));
}

#[test]
fn test_create_dialog_with_type_selection() {
    let mut dialog = CreateDialog::new();
    
    // Set filename
    dialog.add_char('t');
    dialog.add_char('e');
    dialog.add_char('s');
    dialog.add_char('t');
    
    // Navigate to type field and select Context type
    dialog.next_field(); // Move to Type
    
    dialog.next_type(); // Instruction
    dialog.next_type(); // Context
    
    // Verify the state
    assert_eq!(dialog.get_normalized_filename(), "test");
    assert_eq!(dialog.get_template(), CreateTemplate::FromClipboard);
    assert_eq!(dialog.get_prompt_type(), fink::application::models::PromptType::Context);
    assert!(dialog.is_valid());
}

#[test]
fn test_create_prompt_with_specific_type() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path.clone()).unwrap();
    
    // Open create dialog
    app.create_new_prompt().unwrap();
    
    // Configure the dialog
    if let Some(dialog) = app.get_create_dialog_mut() {
        dialog.add_char('t');
        dialog.add_char('y');
        dialog.add_char('p');
        dialog.add_char('e');
        dialog.add_char('-');
        dialog.add_char('t');
        dialog.add_char('e');
        dialog.add_char('s');
        dialog.add_char('t');
        
        // Navigate to type field and select OutputIndicator
        dialog.next_field(); // Move to Type
        
        dialog.next_type(); // Instruction
        dialog.next_type(); // Context
        dialog.next_type(); // InputIndicator
        dialog.next_type(); // OutputIndicator
    }
    
    // Confirm creation
    app.confirm_create().unwrap();
    
    // Verify prompt was created with correct type
    let created_file = jkms_path.join("type-test.md");
    assert!(created_file.exists());
    
    let content = fs::read_to_string(&created_file).unwrap();
    assert!(content.contains("name: \"type-test\""));
    assert!(content.contains("type: \"output_indicator\""));
    assert!(!content.contains("type: \"whole\""));
}

#[test]
fn test_cancel_create_dialog() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("prompts");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path.clone()).unwrap();
    
    // Open create dialog
    app.create_new_prompt().unwrap();
    
    // Add some text
    if let Some(dialog) = app.get_create_dialog_mut() {
        dialog.add_char('t');
        dialog.add_char('e');
        dialog.add_char('s');
        dialog.add_char('t');
    }
    
    // Cancel dialog
    app.close_create_dialog();
    
    // Verify dialog is closed
    assert!(!app.is_create_dialog_active());
    
    // Verify no prompt was created
    let test_file = jkms_path.join("test.md");
    assert!(!test_file.exists());
}