use fink::presentation::tui::app::{TUIApp, AppMode};
use fink::presentation::tui::runner::EventHandler;
use fink::utils::config::Config;
use crossterm::event::{Event, KeyCode, KeyEvent};
use tempfile::TempDir;
use std::fs;

#[test]
fn test_first_launch_should_show_initialization_dialog() {
    // Create a fresh directory with no prompts and no .initialized flag
    let temp_dir = TempDir::new().unwrap();
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    
    // Create app - should detect first launch
    let app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    
    // Should show initialization dialog on first launch
    assert!(app.is_showing_init_dialog(), "Should show init dialog on first launch");
    assert_eq!(app.get_prompts().len(), 0, "Should have no prompts initially");
}

#[test]
fn test_accepting_initialization_should_create_default_prompts() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    
    let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    let event_handler = EventHandler::new();
    
    // Should show init dialog
    assert!(app.is_showing_init_dialog());
    
    // Press 'y' to accept initialization
    let yes_event = Event::Key(KeyEvent::from(KeyCode::Char('y')));
    let result = event_handler.handle_event(&mut app, yes_event);
    assert!(result.is_ok());
    
    // Dialog should be closed
    assert!(!app.is_showing_init_dialog(), "Init dialog should be closed");
    
    // Should have created default prompts
    app.reload_prompts().unwrap();
    assert!(app.get_prompts().len() > 0, "Should have default prompts after accepting");
    
    // Check that .initialized flag exists
    let init_flag = temp_dir.path().join("prompts").join(".initialized");
    assert!(init_flag.exists(), "Should create .initialized flag");
}

#[test]
fn test_declining_initialization_should_leave_empty() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    
    let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    let event_handler = EventHandler::new();
    
    // Should show init dialog
    assert!(app.is_showing_init_dialog());
    
    // Press 'n' to decline initialization
    let no_event = Event::Key(KeyEvent::from(KeyCode::Char('n')));
    let result = event_handler.handle_event(&mut app, no_event);
    assert!(result.is_ok());
    
    // Dialog should be closed
    assert!(!app.is_showing_init_dialog(), "Init dialog should be closed");
    
    // Should still have no prompts
    assert_eq!(app.get_prompts().len(), 0, "Should have no prompts after declining");
    
    // But should still create .initialized flag to not ask again
    let init_flag = temp_dir.path().join("prompts").join(".initialized");
    assert!(init_flag.exists(), "Should create .initialized flag even when declined");
}

#[test]
fn test_should_not_show_dialog_if_already_initialized() {
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    fs::create_dir_all(&prompts_dir).unwrap();
    
    // Create .initialized flag
    fs::write(prompts_dir.join(".initialized"), "").unwrap();
    
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    
    let app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    
    // Should NOT show init dialog
    assert!(!app.is_showing_init_dialog(), "Should not show dialog if already initialized");
}

#[test]
fn test_should_not_show_dialog_if_prompts_exist() {
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    fs::create_dir_all(&prompts_dir).unwrap();
    
    // Create a prompt file (no .initialized flag)
    fs::write(
        prompts_dir.join("existing.md"),
        "---\nname: \"existing\"\n---\nContent"
    ).unwrap();
    
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    
    let app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    
    // Should NOT show init dialog if prompts already exist
    assert!(!app.is_showing_init_dialog(), "Should not show dialog if prompts exist");
}