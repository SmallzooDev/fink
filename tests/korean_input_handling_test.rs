use fink::presentation::tui::app::TUIApp;
use fink::presentation::tui::runner::EventHandler;
use crossterm::event::{Event, KeyCode, KeyEvent};
use tempfile::tempdir;
use std::fs;

#[test]
fn test_korean_input_backspace_should_not_crash() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create prompts directory
    let prompts_path = temp_path.join("prompts");
    fs::create_dir_all(&prompts_path).unwrap();
    
    // Create a test prompt file
    let prompt_path = prompts_path.join("test-prompt.md");
    fs::write(&prompt_path, r#"---
name: "test-prompt"
tags: ["test"]
---
# Test Prompt
This is a test prompt."#).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    // Activate search mode
    app.activate_search();
    assert!(app.is_search_active());
    
    // Add Korean characters to search
    let korean_text = "한글";
    app.set_search_query(korean_text);
    assert_eq!(app.get_search_query(), korean_text);
    
    // Simulate backspace key event
    let event_handler = EventHandler::new();
    let backspace_event = Event::Key(KeyEvent::from(KeyCode::Backspace));
    
    // This should not panic
    event_handler.handle_event(&mut app, backspace_event).unwrap();
    
    // Verify that one character was removed properly
    assert_eq!(app.get_search_query(), "한");
}

#[test]
fn test_proper_unicode_backspace_handling() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create prompts directory
    let prompts_path = temp_path.join("prompts");
    fs::create_dir_all(&prompts_path).unwrap();
    
    // Create a test prompt file
    let prompt_path = prompts_path.join("test-prompt.md");
    fs::write(&prompt_path, r#"---
name: "test-prompt"
tags: ["test"]
---
# Test Prompt
This is a test prompt."#).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    app.activate_search();
    
    // Test with various Unicode strings
    let test_cases = vec![
        ("한글", "한"),           // Korean
        ("こんにちは", "こんにち"), // Japanese
        ("🎉🎊", "🎉"),          // Emojis
        ("café", "caf"),         // Accented characters
        ("test한글", "test한"),   // Mixed
    ];
    
    let event_handler = EventHandler::new();
    let backspace_event = Event::Key(KeyEvent::from(KeyCode::Backspace));
    
    for (input, expected_after_backspace) in test_cases {
        app.set_search_query(input);
        
        // Test backspace handling
        event_handler.handle_event(&mut app, backspace_event.clone()).unwrap();
        assert_eq!(app.get_search_query(), expected_after_backspace);
    }
}

#[test]
fn test_mixed_text_multiple_backspaces() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create prompts directory
    let prompts_path = temp_path.join("prompts");
    fs::create_dir_all(&prompts_path).unwrap();
    
    // Create a test prompt file
    let prompt_path = prompts_path.join("test-prompt.md");
    fs::write(&prompt_path, r#"---
name: "test-prompt"
tags: ["test"]
---
# Test Prompt
This is a test prompt."#).unwrap();
    
    // Create TUIApp
    let mut app = TUIApp::new(temp_path).unwrap();
    
    app.activate_search();
    
    // Add mixed Korean and English text
    let mixed_text = "test한글abc";
    app.set_search_query(mixed_text);
    
    // Expected states after each backspace
    let expected_states = vec![
        "test한글ab",
        "test한글a",
        "test한글",
        "test한",
        "test",
        "tes",
        "te",
        "t",
        "",
    ];
    
    let event_handler = EventHandler::new();
    let backspace_event = Event::Key(KeyEvent::from(KeyCode::Backspace));
    
    // Test multiple backspaces
    for expected in expected_states {
        event_handler.handle_event(&mut app, backspace_event.clone()).unwrap();
        assert_eq!(app.get_search_query(), expected);
    }
}