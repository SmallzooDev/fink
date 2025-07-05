use jkms::presentation::tui::components::{CreateDialog, DialogField, CreateTemplate};

#[test]
fn test_h_l_keys_in_filename_field() {
    let mut dialog = CreateDialog::new();
    
    // In filename field, h and l should be typed as characters
    dialog.add_char('h');
    dialog.add_char('e');
    dialog.add_char('l');
    dialog.add_char('l');
    dialog.add_char('o');
    
    assert_eq!(dialog.get_filename(), "hello");
    assert_eq!(dialog.current_field(), DialogField::Filename);
}

#[test]
fn test_h_l_navigation_in_template_field() {
    let mut dialog = CreateDialog::new();
    
    // Add some filename first
    dialog.add_char('t');
    dialog.add_char('e');
    dialog.add_char('s');
    dialog.add_char('t');
    
    // Switch to template field
    dialog.next_field();
    assert_eq!(dialog.current_field(), DialogField::Template);
    
    // Initial template should be FromClipboard
    assert_eq!(dialog.get_template(), CreateTemplate::FromClipboard);
    
    // Test navigation with next_template (simulating 'l' key)
    dialog.next_template();
    assert_eq!(dialog.get_template(), CreateTemplate::Default);
    
    // Test navigation with previous_template (simulating 'h' key)
    dialog.previous_template();
    assert_eq!(dialog.get_template(), CreateTemplate::FromClipboard);
    
    // Filename should remain unchanged
    assert_eq!(dialog.get_filename(), "test");
}

#[test]
fn test_filename_with_h_and_l() {
    let mut dialog = CreateDialog::new();
    
    // Type a filename that contains h and l
    dialog.add_char('h');
    dialog.add_char('e');
    dialog.add_char('l');
    dialog.add_char('p');
    dialog.add_char('f');
    dialog.add_char('u');
    dialog.add_char('l');
    dialog.add_char('-');
    dialog.add_char('t');
    dialog.add_char('o');
    dialog.add_char('o');
    dialog.add_char('l');
    
    assert_eq!(dialog.get_filename(), "helpful-tool");
    assert_eq!(dialog.get_normalized_filename(), "helpful-tool");
}