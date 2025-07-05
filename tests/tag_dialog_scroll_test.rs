use jkms::presentation::tui::components::TagManagementDialog;
use jkms::presentation::tui::components::TagFilterDialog;

#[test]
fn test_tag_management_dialog_scroll() {
    // Create dialog with many tags
    let tags: Vec<String> = (0..20).map(|i| format!("tag{}", i)).collect();
    let mut dialog = TagManagementDialog::new(tags.clone());
    
    // Start removing mode to enable selection
    dialog.start_removing_tag();
    
    // Test scrolling down
    for _ in 0..5 {
        dialog.move_selection_down();
    }
    
    // Should be at index 5
    assert_eq!(dialog.get_selected_tag_for_removal(), Some("tag5".to_string()));
    
    // Test scrolling up
    for _ in 0..3 {
        dialog.move_selection_up();
    }
    
    // Should be at index 2
    assert_eq!(dialog.get_selected_tag_for_removal(), Some("tag2".to_string()));
    
    // Test wrap around at bottom
    dialog.move_selection_up();
    dialog.move_selection_up();
    dialog.move_selection_up(); // Should wrap to bottom
    
    assert_eq!(dialog.get_selected_tag_for_removal(), Some("tag19".to_string()));
    
    // Test wrap around at top
    dialog.move_selection_down(); // Should wrap to top
    
    assert_eq!(dialog.get_selected_tag_for_removal(), Some("tag0".to_string()));
}

#[test]
fn test_tag_filter_dialog_scroll() {
    // Create dialog with many tags
    let tags: Vec<String> = (0..15).map(|i| format!("filter{}", i)).collect();
    let mut dialog = TagFilterDialog::new(tags.clone(), None);
    
    // Initial selection should be at 0
    assert_eq!(dialog.get_selected_tag(), Some(&"filter0".to_string()));
    
    // Test scrolling down
    for _ in 0..5 {
        dialog.move_down();
    }
    assert_eq!(dialog.get_selected_tag(), Some(&"filter5".to_string()));
    
    // Test scrolling to end and wrap around
    for _ in 0..10 {
        dialog.move_down();
    }
    assert_eq!(dialog.get_selected_tag(), Some(&"filter0".to_string()));
    
    // Test scrolling up and wrap around
    dialog.move_up();
    assert_eq!(dialog.get_selected_tag(), Some(&"filter14".to_string()));
}

#[test]
fn test_empty_tag_list_navigation() {
    // Test with empty tag list
    let mut dialog = TagManagementDialog::new(vec![]);
    dialog.start_removing_tag();
    
    // Should not crash when trying to navigate
    dialog.move_selection_down();
    dialog.move_selection_up();
    
    assert_eq!(dialog.get_selected_tag_for_removal(), None);
}

#[test]
fn test_single_tag_navigation() {
    // Test with single tag
    let mut dialog = TagManagementDialog::new(vec!["single".to_string()]);
    dialog.start_removing_tag();
    
    // Should stay on the same item
    dialog.move_selection_down();
    assert_eq!(dialog.get_selected_tag_for_removal(), Some("single".to_string()));
    
    dialog.move_selection_up();
    assert_eq!(dialog.get_selected_tag_for_removal(), Some("single".to_string()));
}