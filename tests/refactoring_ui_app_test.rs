use jkms::presentation::tui::tui_refactored::TUIApp;
use jkms::presentation::tui::state::{UIEvent, Direction};
use jkms::presentation::tui::tui::AppMode;
use tempfile::TempDir;

fn create_test_app(dir: &TempDir) -> anyhow::Result<TUIApp> {
    // Create some test prompts
    let prompts_dir = dir.path().join("jkms");
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
    
    TUIApp::new(dir.path().to_path_buf())
}

#[test]
fn tui_app_should_use_ui_state_for_navigation() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let mut app = create_test_app(&temp_dir).unwrap();
    
    // Act & Assert
    assert_eq!(app.selected_index(), 0);
    
    app.handle_ui_event(UIEvent::Navigate(Direction::Next));
    assert_eq!(app.selected_index(), 1);
    
    app.handle_ui_event(UIEvent::Navigate(Direction::Previous));
    assert_eq!(app.selected_index(), 0);
}

#[test]
fn tui_app_should_use_ui_state_for_mode_toggle() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let mut app = create_test_app(&temp_dir).unwrap();
    assert_eq!(app.mode(), &AppMode::QuickSelect);
    
    // Act
    app.handle_ui_event(UIEvent::ToggleMode);
    
    // Assert
    assert_eq!(app.mode(), &AppMode::Management);
}

#[test]
fn tui_app_should_delegate_confirmation_to_ui_state() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let mut app = create_test_app(&temp_dir).unwrap();
    
    // Act
    app.show_delete_confirmation();
    
    // Assert
    assert!(app.is_showing_confirmation());
    assert!(app.get_confirmation_message().is_some());
    
    // Cancel confirmation
    app.handle_ui_event(UIEvent::CancelAction);
    assert!(!app.is_showing_confirmation());
}

#[test]
fn tui_app_should_process_commands_from_ui_state() {
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let mut app = create_test_app(&temp_dir).unwrap();
    
    // Setup delete confirmation
    app.show_delete_confirmation();
    
    // Act - confirm deletion
    app.handle_ui_event(UIEvent::ConfirmAction);
    app.process_pending_commands().unwrap();
    
    // Assert - prompt should be deleted
    let prompts = app.get_prompts();
    assert_eq!(prompts.len(), 1); // One prompt deleted
}

#[test]
fn tui_app_should_maintain_business_logic_separation() {
    // This test verifies that TUIApp still maintains its business logic
    // while delegating UI state management to UIState
    
    // Arrange
    let temp_dir = TempDir::new().unwrap();
    let mut app = create_test_app(&temp_dir).unwrap();
    
    // Business logic operations should still work
    let selected_content = app.get_selected_content();
    assert!(selected_content.is_some());
    
    // Copy to clipboard (business logic)
    let result = app.copy_selected_to_clipboard();
    assert!(result.is_ok());
}