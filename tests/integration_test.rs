use jkms::cli::run_app;
use tempfile::tempdir;

#[test]
fn should_create_app_runner() {
    // Arrange
    let temp_dir = tempdir().unwrap();
    let prompts_dir = temp_dir.path().join("jkms");
    std::fs::create_dir(&prompts_dir).unwrap();

    std::fs::write(prompts_dir.join("test.md"), "# Test").unwrap();

    // Act
    let runner = run_app(temp_dir.path().to_path_buf());

    // Assert
    assert!(runner.is_ok());
}

#[test]
fn should_handle_empty_directory() {
    // Arrange
    let temp_dir = tempdir().unwrap();

    // Act
    let runner = run_app(temp_dir.path().to_path_buf());

    // Assert
    assert!(runner.is_ok());
}
