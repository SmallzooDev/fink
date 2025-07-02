use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use jkms::presentation::tui::runner::EventHandler;
use jkms::presentation::tui::tui::{TUIApp, AppMode};
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
    let prompts_dir = temp_dir.path().join("jkms");
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
    let prompts_dir = temp_dir.path().join("jkms");
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
