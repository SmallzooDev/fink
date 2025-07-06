use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use fink::presentation::tui::runner::EventHandler;
use fink::presentation::tui::tui::{TUIApp, AppMode};
use tempfile::tempdir;

#[test]
fn should_handle_quit_event() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let mut app = TUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    let handler = EventHandler::new();

    // Act
    let quit_event = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()));
    let result = handler.handle_event(&mut app, quit_event);

    // Assert
    assert!(result.is_ok());
    assert!(app.should_quit());
}

#[test]
fn should_handle_navigation_down() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("fink");
    std::fs::create_dir(&prompts_dir).unwrap();

    std::fs::write(prompts_dir.join("test1.md"), "# Test 1").unwrap();
    std::fs::write(prompts_dir.join("test2.md"), "# Test 2").unwrap();

    let mut app = TUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    let handler = EventHandler::new();

    // Act
    let down_event = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()));
    let result = handler.handle_event(&mut app, down_event);

    // Assert
    assert!(result.is_ok());
    assert_eq!(app.selected_index(), 1);
}

#[test]
fn should_handle_enter_to_copy() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("fink");
    std::fs::create_dir(&prompts_dir).unwrap();

    let content = r#"---
name: "Test Prompt"
---
# Test Content"#;

    std::fs::write(prompts_dir.join("test.md"), content).unwrap();

    let mut app = TUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    let handler = EventHandler::new();

    // Act
    let enter_event = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()));
    let result = handler.handle_event(&mut app, enter_event);

    // Assert
    assert!(result.is_ok());
    assert!(app.should_quit()); // App should quit after copying
}

#[test]
fn should_toggle_to_management_mode() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let mut app = TUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    let handler = EventHandler::new();
    
    // Verify starts in QuickSelect mode
    assert_eq!(*app.mode(), AppMode::QuickSelect);
    
    // Act
    let m_event = Event::Key(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::empty()));
    let result = handler.handle_event(&mut app, m_event);
    
    // Assert
    assert!(result.is_ok());
    assert_eq!(*app.mode(), AppMode::Management);
    assert!(!app.should_quit()); // Should not quit, just change mode
}

#[test]
fn should_toggle_back_to_quick_select_mode() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let mut app = TUIApp::new_with_mode(temp_dir.path().to_path_buf(), AppMode::Management).unwrap();
    let handler = EventHandler::new();
    
    // Verify starts in Management mode
    assert_eq!(*app.mode(), AppMode::Management);
    
    // Act
    let m_event = Event::Key(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::empty()));
    let result = handler.handle_event(&mut app, m_event);
    
    // Assert
    assert!(result.is_ok());
    assert_eq!(*app.mode(), AppMode::QuickSelect);
}

#[test]
fn should_handle_edit_in_management_mode() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("fink");
    std::fs::create_dir(&prompts_dir).unwrap();
    
    let content = r#"---
name: "Test Prompt"
---
# Test Content"#;
    
    std::fs::write(prompts_dir.join("test.md"), content).unwrap();
    
    let mut app = TUIApp::new_with_mode(temp_dir.path().to_path_buf(), AppMode::Management).unwrap();
    let handler = EventHandler::new();
    
    // Act
    let e_event = Event::Key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::empty()));
    let result = handler.handle_event(&mut app, e_event);
    
    // Assert
    assert!(result.is_ok());
    // For now, we'll just check it doesn't crash
    // Later we'll add actual edit functionality
}

#[test]
fn should_handle_delete_in_management_mode() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("fink");
    std::fs::create_dir(&prompts_dir).unwrap();
    
    let content = r#"---
name: "Test Prompt"
---
# Test Content"#;
    
    std::fs::write(prompts_dir.join("test.md"), content).unwrap();
    
    let mut app = TUIApp::new_with_mode(temp_dir.path().to_path_buf(), AppMode::Management).unwrap();
    let handler = EventHandler::new();
    
    // Act
    let d_event = Event::Key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::empty()));
    let result = handler.handle_event(&mut app, d_event);
    
    // Assert
    assert!(result.is_ok());
    // For now, we'll just check it doesn't crash
    // Later we'll add actual delete functionality
}

#[test]
fn should_handle_new_in_management_mode() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let mut app = TUIApp::new_with_mode(temp_dir.path().to_path_buf(), AppMode::Management).unwrap();
    let handler = EventHandler::new();
    
    // Act
    let n_event = Event::Key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::empty()));
    let result = handler.handle_event(&mut app, n_event);
    
    // Assert
    assert!(result.is_ok());
    // For now, we'll just check it doesn't crash
    // Later we'll add actual new prompt functionality
}

#[test]
fn should_not_handle_management_keys_in_quick_select_mode() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let mut app = TUIApp::new(temp_dir.path().to_path_buf()).unwrap();
    let handler = EventHandler::new();
    
    let initial_state = app.selected_index();
    
    // Act - try management mode keys in quick select mode
    let e_event = Event::Key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::empty()));
    let _ = handler.handle_event(&mut app, e_event);
    
    let d_event = Event::Key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::empty()));
    let _ = handler.handle_event(&mut app, d_event);
    
    let n_event = Event::Key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::empty()));
    let _ = handler.handle_event(&mut app, n_event);
    
    // Assert - nothing should have changed
    assert_eq!(app.selected_index(), initial_state);
    assert!(!app.should_quit());
}

#[test]
fn should_show_confirmation_dialog_on_delete() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("fink");
    std::fs::create_dir(&prompts_dir).unwrap();
    
    let content = r#"---
name: "Test Prompt"
---
# Test Content"#;
    
    std::fs::write(prompts_dir.join("test.md"), content).unwrap();
    
    let mut app = TUIApp::new_with_mode(temp_dir.path().to_path_buf(), AppMode::Management).unwrap();
    let handler = EventHandler::new();
    
    // Act - press 'd' to initiate delete
    let d_event = Event::Key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::empty()));
    let result = handler.handle_event(&mut app, d_event);
    
    // Assert
    assert!(result.is_ok());
    assert!(app.is_showing_confirmation());
    assert_eq!(app.get_confirmation_message().unwrap(), "Are you sure you want to delete 'Test Prompt'?");
    
    // File should still exist
    assert!(prompts_dir.join("test.md").exists());
}

#[test]
fn should_cancel_delete_on_escape_in_confirmation() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("fink");
    std::fs::create_dir(&prompts_dir).unwrap();
    
    let content = r#"---
name: "Test Prompt"
---
# Test Content"#;
    
    std::fs::write(prompts_dir.join("test.md"), content).unwrap();
    
    let mut app = TUIApp::new_with_mode(temp_dir.path().to_path_buf(), AppMode::Management).unwrap();
    let handler = EventHandler::new();
    
    // First show the confirmation dialog
    let d_event = Event::Key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::empty()));
    handler.handle_event(&mut app, d_event).unwrap();
    
    // Act - press ESC to cancel
    let esc_event = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()));
    let result = handler.handle_event(&mut app, esc_event);
    
    // Assert
    assert!(result.is_ok());
    assert!(!app.is_showing_confirmation());
    assert!(!app.should_quit()); // Should not quit the app
    
    // File should still exist
    assert!(prompts_dir.join("test.md").exists());
}

#[test]
fn should_confirm_delete_on_y_key() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("fink");
    std::fs::create_dir(&prompts_dir).unwrap();
    
    let content = r#"---
name: "Test Prompt"
---
# Test Content"#;
    
    std::fs::write(prompts_dir.join("test.md"), content).unwrap();
    
    let mut app = TUIApp::new_with_mode(temp_dir.path().to_path_buf(), AppMode::Management).unwrap();
    let handler = EventHandler::new();
    
    // First show the confirmation dialog
    let d_event = Event::Key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::empty()));
    handler.handle_event(&mut app, d_event).unwrap();
    
    // Act - press 'y' to confirm
    let y_event = Event::Key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::empty()));
    let result = handler.handle_event(&mut app, y_event);
    
    // Assert
    assert!(result.is_ok());
    assert!(!app.is_showing_confirmation());
    
    // File should be deleted
    assert!(!prompts_dir.join("test.md").exists());
}
