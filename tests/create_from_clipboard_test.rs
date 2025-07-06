use fink::presentation::tui::components::{CreateDialog, CreateTemplate};
use fink::application::application::DefaultPromptApplication;
use fink::application::traits::PromptApplication;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_create_from_clipboard_template() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create jkms directory
    let jkms_path = temp_path.join("fink");
    fs::create_dir_all(&jkms_path).unwrap();
    
    // Create application
    let app = DefaultPromptApplication::new(temp_path.clone()).unwrap();
    
    // Set clipboard content
    let clipboard_content = "This is content from the clipboard that should become a prompt";
    app.copy_to_clipboard(clipboard_content).unwrap();
    
    // Create prompt from clipboard with content
    app.create_prompt_with_content("clipboard-test", Some("clipboard"), Some(clipboard_content.to_string())).unwrap();
    
    // Verify the prompt was created
    let (metadata, content) = app.get_prompt("clipboard-test").unwrap();
    
    assert_eq!(metadata.name, "clipboard-test");
    assert!(content.contains(clipboard_content));
}

#[test]
fn test_create_dialog_clipboard_template_selection() {
    let mut dialog = CreateDialog::new();
    
    // Add filename
    dialog.add_char('t');
    dialog.add_char('e');
    dialog.add_char('s');
    dialog.add_char('t');
    
    // Navigate to template field
    dialog.next_field();
    
    // FromClipboard is now the default, so we don't need to change it
    assert_eq!(dialog.get_template(), CreateTemplate::FromClipboard);
    assert_eq!(dialog.get_filename(), "test");
}