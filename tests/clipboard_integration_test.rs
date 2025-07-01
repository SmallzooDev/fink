use jkms::external::ClipboardManager;

#[test]
fn should_copy_text_to_clipboard() {
    // Arrange
    let text = "Hello, clipboard!";
    let mut manager = ClipboardManager::new();

    // Act
    let result = manager.copy(text);

    // Assert
    assert!(result.is_ok());
}

#[test]
fn should_handle_empty_text() {
    // Arrange
    let text = "";
    let mut manager = ClipboardManager::new();

    // Act
    let result = manager.copy(text);

    // Assert
    assert!(result.is_ok());
}
