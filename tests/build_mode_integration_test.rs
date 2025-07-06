#[cfg(test)]
mod tests {
    use jkms::presentation::tui::tui::{TUIApp, AppMode};
    use jkms::presentation::tui::runner::EventHandler;
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn should_enter_build_mode_when_b_key_pressed() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Create jkms directory
        let jkms_path = temp_path.join("jkms");
        fs::create_dir_all(&jkms_path).unwrap();
        
        // Create some test prompts with different types
        let instruction_prompt = r#"---
name: "instruction-prompt"
tags: []
type: "instruction"
---
# Instruction prompt"#;
        
        let context_prompt = r#"---
name: "context-prompt"
tags: []
type: "context"
---
# Context prompt"#;
        
        fs::write(jkms_path.join("instruction.md"), instruction_prompt).unwrap();
        fs::write(jkms_path.join("context.md"), context_prompt).unwrap();
        
        // Create TUIApp and event handler
        let mut app = TUIApp::new(temp_path.clone()).unwrap();
        app.reload_prompts().unwrap();
        
        let event_handler = EventHandler::new();
        
        // Initially should be in QuickSelect mode
        assert_eq!(*app.mode(), AppMode::QuickSelect);
        
        // Simulate pressing 'b' key
        let b_key_event = Event::Key(KeyEvent {
            code: KeyCode::Char('b'),
            modifiers: KeyModifiers::empty(),
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::empty(),
        });
        
        event_handler.handle_event(&mut app, b_key_event).unwrap();
        
        // Should now be in Build mode
        assert_eq!(*app.mode(), AppMode::Build);
        
        // Build panel should be initialized
        assert!(app.get_build_panel().is_some());
    }
    
    #[test]
    fn should_exit_build_mode_when_escape_pressed() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Create jkms directory
        let jkms_path = temp_path.join("jkms");
        fs::create_dir_all(&jkms_path).unwrap();
        
        // Create TUIApp and enter build mode
        let mut app = TUIApp::new(temp_path.clone()).unwrap();
        app.enter_build_mode();
        
        let event_handler = EventHandler::new();
        
        // Should be in Build mode
        assert_eq!(*app.mode(), AppMode::Build);
        
        // Simulate pressing Escape key
        let esc_key_event = Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::empty(),
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::empty(),
        });
        
        event_handler.handle_event(&mut app, esc_key_event).unwrap();
        
        // Should return to QuickSelect mode
        assert_eq!(*app.mode(), AppMode::QuickSelect);
        
        // Build panel should be cleared
        assert!(app.get_build_panel().is_none());
    }
    
    #[test]
    fn should_navigate_in_build_panel_with_arrow_keys() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Create jkms directory
        let jkms_path = temp_path.join("jkms");
        fs::create_dir_all(&jkms_path).unwrap();
        
        // Create multiple prompts
        for i in 1..=3 {
            let prompt = format!(r#"---
name: "prompt-{}"
tags: []
type: "instruction"
---
# Prompt {}"#, i, i);
            fs::write(jkms_path.join(format!("prompt{}.md", i)), prompt).unwrap();
        }
        
        // Create TUIApp and enter build mode
        let mut app = TUIApp::new(temp_path.clone()).unwrap();
        app.reload_prompts().unwrap();
        app.enter_build_mode();
        
        let event_handler = EventHandler::new();
        
        // Test navigation with down arrow
        let down_key_event = Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::empty(),
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::empty(),
        });
        
        event_handler.handle_event(&mut app, down_key_event).unwrap();
        
        // Test navigation with up arrow
        let up_key_event = Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::empty(),
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::empty(),
        });
        
        event_handler.handle_event(&mut app, up_key_event).unwrap();
        
        // Should still be in build mode
        assert_eq!(*app.mode(), AppMode::Build);
    }
}