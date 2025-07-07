use fink::presentation::tui::runner::EventHandler;
use fink::presentation::tui::tui::{TUIApp, AppMode};
use fink::utils::config::Config;
use crossterm::event::{Event, KeyCode, KeyEvent};
use tempfile::TempDir;
use std::fs;

#[test]
fn test_error_from_empty_list_should_not_persist_after_relaunch() {
    // Create a temporary directory with no prompts
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    fs::create_dir_all(&prompts_dir).unwrap();
    fs::write(prompts_dir.join(".initialized"), "").unwrap();
    
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    
    // First launch - simulate pressing Enter on empty list
    {
        let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
        let event_handler = EventHandler::new();
        
        // Simulate pressing Enter
        let enter_event = Event::Key(KeyEvent::from(KeyCode::Enter));
        let result = event_handler.handle_event(&mut app, enter_event);
        
        // This should NOT error anymore - error is handled internally
        assert!(result.is_ok(), "Should handle error internally and not propagate");
        
        // App should have error message set
        assert!(app.has_error(), "App should have error message set");
    }
    
    // Second launch - app should start cleanly
    {
        let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
        
        // App should be in a clean state
        assert!(!app.should_quit(), "App should not be in quit state on second launch");
        assert_eq!(app.get_prompts().len(), 0, "Should still have empty prompt list");
        
        // Should be able to handle events normally
        let event_handler = EventHandler::new();
        let esc_event = Event::Key(KeyEvent::from(KeyCode::Esc));
        let result = event_handler.handle_event(&mut app, esc_event);
        assert!(result.is_ok(), "Should handle Esc key normally");
        assert!(app.should_quit(), "App should quit after Esc");
    }
}

#[test]
fn test_empty_list_enter_should_show_error_not_crash() {
    let temp_dir = TempDir::new().unwrap();
    let prompts_dir = temp_dir.path().join("prompts");
    fs::create_dir_all(&prompts_dir).unwrap();
    fs::write(prompts_dir.join(".initialized"), "").unwrap();
    
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    
    let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    let event_handler = EventHandler::new();
    
    // Press Enter on empty list
    let enter_event = Event::Key(KeyEvent::from(KeyCode::Enter));
    let result = event_handler.handle_event(&mut app, enter_event);
    
    // Should NOT return error - handled internally
    assert!(result.is_ok(), "Error should be handled internally");
    
    // App should still be functional and have error message
    assert!(!app.should_quit(), "App should not quit on error");
    assert!(app.has_error(), "App should have error message");
    
    // First key press should clear the error
    let clear_event = Event::Key(KeyEvent::from(KeyCode::Char('q')));
    let clear_result = event_handler.handle_event(&mut app, clear_event);
    assert!(clear_result.is_ok());
    assert!(!app.has_error(), "Error should be cleared");
    assert!(!app.should_quit(), "First 'q' clears error, doesn't quit");
    
    // Second 'q' should quit
    let quit_event = Event::Key(KeyEvent::from(KeyCode::Char('q')));
    let quit_result = event_handler.handle_event(&mut app, quit_event);
    assert!(quit_result.is_ok());
    assert!(app.should_quit(), "Second 'q' should quit");
}