use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use jkms::cli::EventHandler;
use jkms::ui::app::App;
use tempfile::tempdir;

#[test]
fn should_handle_quit_event() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let mut app = App::new(temp_dir.path().to_path_buf()).unwrap();
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

    let mut app = App::new(temp_dir.path().to_path_buf()).unwrap();
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

    let mut app = App::new(temp_dir.path().to_path_buf()).unwrap();
    let handler = EventHandler::new();

    // Act
    let enter_event = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()));
    let result = handler.handle_event(&mut app, enter_event);

    // Assert
    assert!(result.is_ok());
    assert!(app.should_quit()); // App should quit after copying
}
