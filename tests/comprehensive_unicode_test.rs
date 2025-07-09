use fink::presentation::tui::tui::TUIApp;
use fink::presentation::tui::runner::EventHandler;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tempfile::tempdir;
use std::fs;

fn create_test_app(temp_path: &std::path::Path) -> TUIApp {
    // Create config with the test path
    let config_content = format!(
        r#"editor = "vim"
storage_path = "{}"
clipboard_prefix = ""
clipboard_postfix = ""
"#,
        temp_path.to_str().unwrap()
    );
    let config_path = temp_path.join("config.toml");
    fs::write(&config_path, config_content).unwrap();
    let config = fink::utils::config::Config::load_from_file(&config_path).unwrap();
    
    // Create TUIApp with the config
    TUIApp::new_with_config(&config).unwrap()
}

#[test]
fn test_unicode_copy_paste_scenarios() {
    // Setup test environment
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create prompts directory
    let prompts_path = temp_path.join("prompts");
    fs::create_dir_all(&prompts_path).unwrap();
    
    // Create test prompts with Unicode names
    let unicode_prompts = vec![
        ("한글-프롬프트.md", "Korean prompt"),
        ("日本語プロンプト.md", "Japanese prompt"),
        ("emoji-🎉-prompt.md", "Emoji prompt"),
        ("café-prompt.md", "Accented prompt"),
    ];
    
    for (filename, content) in unicode_prompts {
        let prompt_path = prompts_path.join(filename);
        let frontmatter = format!(r#"---
name: "{}"
tags: ["unicode", "test"]
---
# {}
This is a test prompt."#, filename.trim_end_matches(".md"), content);
        fs::write(&prompt_path, frontmatter).unwrap();
    }
    
    // Create TUIApp
    let mut app = create_test_app(&temp_path);
    
    // Test that Unicode prompts are loaded correctly
    let prompts = app.get_filtered_prompts();
    assert!(prompts.len() >= 4, "Should load all Unicode prompts");
    
    // Test searching for Unicode content
    app.activate_search();
    app.set_search_query("한글");
    let filtered = app.get_filtered_prompts();
    assert_eq!(filtered.len(), 1, "Should find Korean prompt");
    
    // Test navigation with Unicode prompts
    app.set_search_query("");
    app.next();
    app.previous();
    // Should not crash
}

#[test]
fn test_unicode_in_different_app_modes() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create prompts directory
    let prompts_path = temp_path.join("prompts");
    fs::create_dir_all(&prompts_path).unwrap();
    
    // Create a test prompt
    let prompt_path = prompts_path.join("test.md");
    fs::write(&prompt_path, r#"---
name: "test"
tags: ["test"]
---
Test"#).unwrap();
    
    let mut app = create_test_app(&temp_path);
    let event_handler = EventHandler::new();
    
    // Test in QuickSelect mode
    app.activate_search();
    let unicode_text = "검색어"; // Korean for "search term"
    for ch in unicode_text.chars() {
        let event = Event::Key(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::empty()));
        event_handler.handle_event(&mut app, event).unwrap();
    }
    assert_eq!(app.get_search_query(), unicode_text);
    
    // Clear search
    app.deactivate_search();
    
    // Switch to Management mode
    let switch_event = Event::Key(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::empty()));
    event_handler.handle_event(&mut app, switch_event).unwrap();
    
    // Test search in Management mode
    app.activate_search();
    for ch in unicode_text.chars() {
        let event = Event::Key(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::empty()));
        event_handler.handle_event(&mut app, event).unwrap();
    }
    assert_eq!(app.get_search_query(), unicode_text);
}

#[test]
fn test_edge_cases_with_complex_unicode() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create prompts directory
    let prompts_path = temp_path.join("prompts");
    fs::create_dir_all(&prompts_path).unwrap();
    
    // Create a test prompt
    let prompt_path = prompts_path.join("test.md");
    fs::write(&prompt_path, r#"---
name: "test"
tags: ["test"]
---
Test"#).unwrap();
    
    let mut app = create_test_app(&temp_path);
    let event_handler = EventHandler::new();
    
    // Test complex Unicode sequences
    let test_cases = vec![
        "👨‍👩‍👧‍👦", // Family emoji (multi-codepoint)
        "🏳️‍🌈", // Rainbow flag (combining characters)
        "é", // Combining accent
        "한국어와English", // Mixed scripts
        "Ａ", // Full-width character
        "𝐇𝐞𝐥𝐥𝐨", // Mathematical alphanumeric symbols
    ];
    
    for test_text in test_cases {
        app.activate_search();
        app.set_search_query("");
        
        // Type the text
        for ch in test_text.chars() {
            let event = Event::Key(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::empty()));
            event_handler.handle_event(&mut app, event).unwrap();
        }
        
        // Verify it was added correctly
        assert_eq!(app.get_search_query(), test_text);
        
        // Test backspace with complex Unicode
        let char_count = test_text.chars().count();
        for _ in 0..char_count {
            let backspace_event = Event::Key(KeyEvent::from(KeyCode::Backspace));
            event_handler.handle_event(&mut app, backspace_event).unwrap();
        }
        
        // Should be empty after deleting all characters
        assert_eq!(app.get_search_query(), "");
        
        app.deactivate_search();
    }
}

#[test]
fn test_unicode_tag_filtering() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create prompts directory
    let prompts_path = temp_path.join("prompts");
    fs::create_dir_all(&prompts_path).unwrap();
    
    // Create prompts with Unicode tags
    let prompts_with_unicode_tags = vec![
        ("prompt1.md", vec!["한국어", "태그"]),
        ("prompt2.md", vec!["日本語", "タグ"]),
        ("prompt3.md", vec!["emoji", "🎉"]),
    ];
    
    for (filename, tags) in prompts_with_unicode_tags {
        let prompt_path = prompts_path.join(filename);
        let tags_str = tags.iter().map(|t| format!("\"{}\"", t)).collect::<Vec<_>>().join(", ");
        let frontmatter = format!(r#"---
name: "{}"
tags: [{}]
---
Test"#, filename.trim_end_matches(".md"), tags_str);
        fs::write(&prompt_path, frontmatter).unwrap();
    }
    
    let app = create_test_app(&temp_path);
    
    // Test that Unicode tags are loaded
    let prompts = app.get_filtered_prompts();
    assert_eq!(prompts.len(), 3, "Should load all prompts with Unicode tags");
    
    // Verify tags are preserved correctly
    for prompt in prompts {
        assert!(!prompt.tags.is_empty(), "Should have tags");
    }
}