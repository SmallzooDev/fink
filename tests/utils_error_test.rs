use fink::utils::error::{FinkError, PromptError, StorageError, ExternalError, ValidationError};
use std::io;

#[test]
fn prompt_error_should_display_correctly() {
    let error = FinkError::Prompt(PromptError::NotFound("test-prompt".to_string()));
    assert_eq!(error.to_string(), "Prompt not found: test-prompt");
    
    let error = FinkError::Prompt(PromptError::AlreadyExists("existing".to_string()));
    assert_eq!(error.to_string(), "Prompt already exists: existing");
    
    let error = FinkError::Prompt(PromptError::InvalidFormat("bad format".to_string()));
    assert_eq!(error.to_string(), "Invalid prompt format: bad format");
}

#[test]
fn storage_error_should_wrap_io_errors() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let error = FinkError::Storage(StorageError::Io(io_error));
    assert!(error.to_string().contains("IO error"));
    
    let error = FinkError::Storage(StorageError::ParseError("invalid yaml".to_string()));
    assert_eq!(error.to_string(), "Parse error: invalid yaml");
    
    let error = FinkError::Storage(StorageError::InvalidPath("bad/path".to_string()));
    assert_eq!(error.to_string(), "Invalid path: bad/path");
}

#[test]
fn external_error_should_handle_clipboard_and_editor() {
    let error = FinkError::External(ExternalError::ClipboardError("failed to copy".to_string()));
    assert_eq!(error.to_string(), "Clipboard error: failed to copy");
    
    let error = FinkError::External(ExternalError::EditorError("vim crashed".to_string()));
    assert_eq!(error.to_string(), "Editor error: vim crashed");
}

#[test]
fn validation_error_should_provide_helpful_messages() {
    let error = FinkError::Validation(ValidationError::InvalidInput("name", "cannot contain spaces".to_string()));
    assert_eq!(error.to_string(), "Invalid input for 'name': cannot contain spaces");
    
    let error = FinkError::Validation(ValidationError::MissingRequired("template".to_string()));
    assert_eq!(error.to_string(), "Missing required field: template");
}

#[test]
fn error_should_implement_std_error() {
    // Ensure our error type implements std::error::Error
    fn assert_error<E: std::error::Error>(_: &E) {}
    
    let error = FinkError::Prompt(PromptError::NotFound("test".to_string()));
    assert_error(&error);
}

#[test]
fn error_should_support_conversion_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let jkms_error: FinkError = io_error.into();
    
    match jkms_error {
        FinkError::Storage(StorageError::Io(_)) => {} // Expected
        _ => panic!("Expected StorageError::Io variant"),
    }
}

#[test]
fn error_should_provide_user_friendly_messages() {
    let error = FinkError::Prompt(PromptError::NotFound("my-prompt".to_string()));
    let user_message = error.user_message();
    
    assert!(user_message.contains("Could not find"));
    assert!(user_message.contains("my-prompt"));
    assert!(user_message.contains("Try")); // Should include suggestions
}

#[test]
fn error_should_indicate_if_recoverable() {
    let not_found = FinkError::Prompt(PromptError::NotFound("test".to_string()));
    assert!(not_found.is_recoverable()); // User can fix by creating the prompt
    
    let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "no access");
    let storage_error = FinkError::Storage(StorageError::Io(io_error));
    assert!(!storage_error.is_recoverable()); // System error, harder to fix
}