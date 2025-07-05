use jkms::presentation::tui::components::CreateDialog;
use jkms::presentation::tui::components::{DialogField, CreateTemplate};

#[test]
fn test_create_dialog_initialization() {
    let dialog = CreateDialog::new();
    
    assert_eq!(dialog.get_filename(), "");
    assert_eq!(dialog.get_template(), CreateTemplate::FromClipboard);
    assert_eq!(dialog.current_field(), DialogField::Filename);
}

#[test]
fn test_create_dialog_input_filename() {
    let mut dialog = CreateDialog::new();
    
    dialog.add_char('t');
    dialog.add_char('e');
    dialog.add_char('s');
    dialog.add_char('t');
    
    assert_eq!(dialog.get_filename(), "test");
}

#[test]
fn test_create_dialog_backspace() {
    let mut dialog = CreateDialog::new();
    
    dialog.add_char('t');
    dialog.add_char('e');
    dialog.add_char('s');
    dialog.add_char('t');
    dialog.delete_char();
    
    assert_eq!(dialog.get_filename(), "tes");
}

#[test]
fn test_create_dialog_navigate_fields() {
    let mut dialog = CreateDialog::new();
    
    assert_eq!(dialog.current_field(), DialogField::Filename);
    
    dialog.next_field();
    assert_eq!(dialog.current_field(), DialogField::Template);
    
    dialog.next_field();
    assert_eq!(dialog.current_field(), DialogField::Filename); // Should wrap around
    
    dialog.previous_field();
    assert_eq!(dialog.current_field(), DialogField::Template);
}

#[test]
fn test_create_dialog_template_selection() {
    let mut dialog = CreateDialog::new();
    
    // Navigate to template field
    dialog.next_field();
    assert_eq!(dialog.current_field(), DialogField::Template);
    
    // Select different templates
    dialog.next_template();
    assert_eq!(dialog.get_template(), CreateTemplate::Default);
    
    dialog.next_template();
    assert_eq!(dialog.get_template(), CreateTemplate::Basic);
    
    dialog.next_template();
    assert_eq!(dialog.get_template(), CreateTemplate::FromClipboard); // Should wrap around
    
    dialog.previous_template();
    assert_eq!(dialog.get_template(), CreateTemplate::Basic);
}

#[test]
fn test_create_dialog_validation() {
    let mut dialog = CreateDialog::new();
    
    // Empty filename should not be valid
    assert!(!dialog.is_valid());
    
    // Add filename
    dialog.add_char('t');
    dialog.add_char('e');
    dialog.add_char('s');
    dialog.add_char('t');
    
    assert!(dialog.is_valid());
}

#[test]
fn test_create_dialog_normalize_filename() {
    let mut dialog = CreateDialog::new();
    
    // Test with spaces
    dialog.add_char('T');
    dialog.add_char('e');
    dialog.add_char('s');
    dialog.add_char('t');
    dialog.add_char(' ');
    dialog.add_char('F');
    dialog.add_char('i');
    dialog.add_char('l');
    dialog.add_char('e');
    
    assert_eq!(dialog.get_normalized_filename(), "test-file");
}