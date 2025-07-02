use jkms::presentation::ui::app::{App, AppMode};
use tempfile::tempdir;

#[test]
fn should_create_app_with_initial_state() {
    // Arrange
    let temp_dir = tempdir().unwrap();

    // Act
    let app = App::new(temp_dir.path().to_path_buf()).unwrap();

    // Assert
    assert_eq!(app.mode(), &AppMode::QuickSelect);
    assert!(!app.should_quit());
}

#[test]
fn should_handle_quit_command() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let mut app = App::new(temp_dir.path().to_path_buf()).unwrap();

    // Act
    app.quit();

    // Assert
    assert!(app.should_quit());
}

#[test]
fn should_handle_navigation() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("jkms");
    std::fs::create_dir(&prompts_dir).unwrap();

    std::fs::write(prompts_dir.join("test1.md"), "# Test 1").unwrap();
    std::fs::write(prompts_dir.join("test2.md"), "# Test 2").unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf()).unwrap();

    // Act
    app.next();

    // Assert
    assert_eq!(app.selected_index(), 1);
}

#[test]
fn should_get_selected_content() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("jkms");
    std::fs::create_dir(&prompts_dir).unwrap();

    let content = r#"---
name: "Test Prompt"
---
# Test Content
This is test content."#;

    std::fs::write(prompts_dir.join("test.md"), content).unwrap();

    let app = App::new(temp_dir.path().to_path_buf()).unwrap();

    // Act
    let selected_content = app.get_selected_content();

    // Assert
    assert!(selected_content.is_some());
    assert_eq!(
        selected_content.unwrap(),
        "# Test Content\nThis is test content."
    );
}

#[test]
fn should_copy_selected_prompt_to_clipboard() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("jkms");
    std::fs::create_dir(&prompts_dir).unwrap();

    let content = r#"---
name: "Test Prompt"
---
# Test Content
This is test content."#;

    std::fs::write(prompts_dir.join("test.md"), content).unwrap();

    let mut app = App::new(temp_dir.path().to_path_buf()).unwrap();

    // Act
    let result = app.copy_selected_to_clipboard();

    // Assert
    assert!(result.is_ok());
}
