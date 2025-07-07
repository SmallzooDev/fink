use fink::presentation::tui::runner::EventHandler;
use fink::presentation::tui::tui::{TUIApp, AppMode};
use fink::utils::config::Config;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tempfile::TempDir;

#[test]
fn test_ctrl_c_should_exit_from_any_mode() {
    let config = Config::default();
    let event_handler = EventHandler::new();
    
    // Test Ctrl+C in QuickSelect mode
    {
        let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
        assert!(!app.should_quit());
        
        let ctrl_c_event = Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
        
        let result = event_handler.handle_event(&mut app, ctrl_c_event);
        assert!(result.is_ok(), "Ctrl+C should be handled");
        assert!(app.should_quit(), "App should quit on Ctrl+C");
    }
    
    // Test Ctrl+C in Management mode
    {
        let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::Management).unwrap();
        assert!(!app.should_quit());
        
        let ctrl_c_event = Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
        
        let result = event_handler.handle_event(&mut app, ctrl_c_event);
        assert!(result.is_ok(), "Ctrl+C should be handled");
        assert!(app.should_quit(), "App should quit on Ctrl+C");
    }
    
    // Test Ctrl+C in Build mode
    {
        let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
        app.enter_build_mode();
        assert!(app.is_build_mode());
        assert!(!app.should_quit());
        
        let ctrl_c_event = Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
        
        let result = event_handler.handle_event(&mut app, ctrl_c_event);
        assert!(result.is_ok(), "Ctrl+C should be handled");
        assert!(app.should_quit(), "App should quit on Ctrl+C even in build mode");
    }
}

#[test]
fn test_ctrl_c_should_exit_even_with_dialogs_open() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = Config::default();
    config.set_storage_path(temp_dir.path().to_path_buf());
    
    let event_handler = EventHandler::new();
    
    // Test with create dialog open
    {
        let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::Management).unwrap();
        app.create_new_prompt().ok(); // This opens the create dialog
        assert!(app.is_create_dialog_active());
        
        let ctrl_c_event = Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
        
        let result = event_handler.handle_event(&mut app, ctrl_c_event);
        assert!(result.is_ok(), "Ctrl+C should be handled");
        assert!(app.should_quit(), "App should quit on Ctrl+C even with dialog open");
    }
    
    // Test with confirmation dialog open
    {
        // Create a prompt first so we can delete it
        let prompts_dir = temp_dir.path().join("prompts");
        std::fs::create_dir_all(&prompts_dir).unwrap();
        std::fs::write(
            prompts_dir.join("test-prompt.md"),
            "---\nname: \"test-prompt\"\ndescription: \"Test\"\ntags: [\"test\"]\ntype: \"whole\"\n---\nTest content"
        ).unwrap();
        
        let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::Management).unwrap();
        app.reload_prompts().unwrap(); // Load the test prompt
        
        app.show_delete_confirmation();
        assert!(app.is_showing_confirmation());
        
        let ctrl_c_event = Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        });
        
        let result = event_handler.handle_event(&mut app, ctrl_c_event);
        assert!(result.is_ok(), "Ctrl+C should be handled");
        assert!(app.should_quit(), "App should quit on Ctrl+C even with confirmation dialog");
    }
}

#[test]
fn test_ctrl_c_should_work_during_search() {
    let config = Config::default();
    let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
    let event_handler = EventHandler::new();
    
    // Activate search mode
    app.activate_search();
    assert!(app.is_search_active());
    
    let ctrl_c_event = Event::Key(KeyEvent {
        code: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
        kind: crossterm::event::KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    });
    
    let result = event_handler.handle_event(&mut app, ctrl_c_event);
    assert!(result.is_ok(), "Ctrl+C should be handled");
    assert!(app.should_quit(), "App should quit on Ctrl+C even during search");
}