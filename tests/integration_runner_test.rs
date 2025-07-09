use fink::presentation::tui::app::TUIApp;
use fink::utils::config::Config;
use tempfile::tempdir;

#[test]
fn should_create_tui_app() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    std::fs::create_dir(&prompts_dir).unwrap();

    std::fs::write(prompts_dir.join("test.md"), "# Test").unwrap();

    // Act
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    let app = TUIApp::new_with_config(&config);

    // Assert
    assert!(app.is_ok());
}

#[test]
fn should_handle_empty_directory() {
    // Arrange
    let temp_dir = tempdir().unwrap();

    // Act
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    let app = TUIApp::new_with_config(&config);

    // Assert
    assert!(app.is_ok());
}
