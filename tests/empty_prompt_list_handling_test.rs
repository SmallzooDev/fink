use fink::presentation::tui::app::{TUIApp, AppMode};
use fink::utils::config::Config;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_copy_selected_with_empty_prompt_list_should_handle_gracefully() {
    // Create a temporary directory with no prompts
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    fs::create_dir_all(&prompts_dir).unwrap();
    
    // Create empty .initialized flag to prevent default prompts
    fs::write(prompts_dir.join(".initialized"), "").unwrap();
    
    // Create a config that uses our temp directory
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    
    let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    
    // Verify list is empty
    assert_eq!(app.get_prompts().len(), 0, "Prompt list should be empty");
    
    // Attempt to copy - this should not panic but return an error
    let result = app.copy_selected_to_clipboard();
    assert!(result.is_err(), "Should return error when no prompt selected");
    
    // The error message should be meaningful
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("No prompt selected") || error_msg.contains("empty"));
}

#[test]
fn test_enter_key_on_empty_list_should_not_crash_app() {
    // Create a temporary directory with no prompts
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    fs::create_dir_all(&prompts_dir).unwrap();
    
    // Create empty .initialized flag
    fs::write(prompts_dir.join(".initialized"), "").unwrap();
    
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    
    let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    
    // Try to navigate when list is empty - should not panic
    app.next(); // Should handle empty list gracefully
    app.previous(); // Should handle empty list gracefully
    
    // Check selected index
    assert!(app.get_selected_content().is_none(), "No content should be selected in empty list");
}

#[test]
fn test_ui_should_show_helpful_message_when_empty() {
    // Create a temporary directory with no prompts
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    fs::create_dir_all(&prompts_dir).unwrap();
    fs::write(prompts_dir.join(".initialized"), "").unwrap();
    
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    
    let app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    
    // In a real implementation, we'd check that the UI shows a helpful message
    // For now, we just verify the app can be created with empty prompts
    assert_eq!(app.get_prompts().len(), 0);
}