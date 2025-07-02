use jkms::presentation::tui::tui::TUIApp as OldTUIApp;
use jkms::presentation::tui::tui_refactored::TUIApp as NewTUIApp;
use jkms::presentation::tui::tui::AppMode;
use tempfile::TempDir;

fn create_test_environment() -> anyhow::Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let prompts_dir = temp_dir.path().join("jkms");
    std::fs::create_dir_all(&prompts_dir)?;
    
    let prompt1_content = r#"---
name: test-prompt-1
description: Test prompt 1
tags: [test, one]
---
# Test Prompt 1
This is test prompt 1"#;
    std::fs::write(prompts_dir.join("test-prompt-1.md"), prompt1_content)?;
    
    let prompt2_content = r#"---
name: test-prompt-2
description: Test prompt 2
tags: [test, two]
---
# Test Prompt 2
This is test prompt 2"#;
    std::fs::write(prompts_dir.join("test-prompt-2.md"), prompt2_content)?;
    
    Ok(temp_dir)
}

#[test]
fn both_implementations_should_have_same_initial_state() {
    // Arrange
    let temp_dir = create_test_environment().unwrap();
    let old_app = OldTUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    let new_app = NewTUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    
    // Assert
    assert_eq!(old_app.selected_index(), new_app.selected_index());
    assert_eq!(old_app.mode(), new_app.mode());
    assert_eq!(old_app.should_quit(), new_app.should_quit());
    assert_eq!(old_app.get_prompts().len(), new_app.get_prompts().len());
}

#[test]
fn both_implementations_should_navigate_identically() {
    // Arrange
    let temp_dir = create_test_environment().unwrap();
    let mut old_app = OldTUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    let mut new_app = NewTUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    
    // Act & Assert - test navigation
    old_app.next();
    new_app.next();
    assert_eq!(old_app.selected_index(), new_app.selected_index());
    assert_eq!(old_app.selected_index(), 1);
    
    old_app.previous();
    new_app.previous();
    assert_eq!(old_app.selected_index(), new_app.selected_index());
    assert_eq!(old_app.selected_index(), 0);
    
    // Test wrap around
    old_app.previous();
    new_app.previous();
    assert_eq!(old_app.selected_index(), new_app.selected_index());
    assert_eq!(old_app.selected_index(), 1); // Should wrap to last
}

#[test]
fn both_implementations_should_toggle_mode_identically() {
    // Arrange
    let temp_dir = create_test_environment().unwrap();
    let mut old_app = OldTUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    let mut new_app = NewTUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    
    // Act & Assert
    assert_eq!(old_app.mode(), new_app.mode());
    assert_eq!(old_app.mode(), &AppMode::QuickSelect);
    
    old_app.toggle_mode();
    new_app.toggle_mode();
    assert_eq!(old_app.mode(), new_app.mode());
    assert_eq!(old_app.mode(), &AppMode::Management);
}

#[test]
fn both_implementations_should_handle_confirmation_identically() {
    // Arrange
    let temp_dir = create_test_environment().unwrap();
    let mut old_app = OldTUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    let mut new_app = NewTUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    
    // Act - show confirmation
    old_app.show_delete_confirmation();
    new_app.show_delete_confirmation();
    
    // Assert
    assert_eq!(old_app.is_showing_confirmation(), new_app.is_showing_confirmation());
    assert!(old_app.is_showing_confirmation());
    
    // Both should have same confirmation message
    assert_eq!(old_app.get_confirmation_message(), new_app.get_confirmation_message());
    
    // Act - cancel confirmation
    old_app.cancel_confirmation();
    new_app.cancel_confirmation();
    
    // Assert
    assert_eq!(old_app.is_showing_confirmation(), new_app.is_showing_confirmation());
    assert!(!old_app.is_showing_confirmation());
}

#[test]
fn both_implementations_should_handle_quit_identically() {
    // Arrange
    let temp_dir = create_test_environment().unwrap();
    let mut old_app = OldTUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    let mut new_app = NewTUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    
    // Act
    old_app.quit();
    new_app.quit();
    
    // Assert
    assert_eq!(old_app.should_quit(), new_app.should_quit());
    assert!(old_app.should_quit());
}