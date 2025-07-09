#[cfg(test)]
mod state_restoration_tests {
    use fink::presentation::tui::app::{TUIApp, AppMode};
    use fink::utils::config::Config;
    use fink::utils::state::AppState;
    use tempfile::TempDir;
    use std::fs;

    fn create_test_app_with_prompts() -> (TempDir, Config) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let state_path = temp_dir.path().join("state.json");
        
        // Set test environment variables with unique paths for each test
        unsafe {
            std::env::set_var("FINK_TEST_CONFIG_PATH", config_path.to_str().unwrap());
            std::env::set_var("FINK_TEST_STATE_PATH", state_path.to_str().unwrap());
        }
        
        // Create a config with test storage path
        let storage_path = temp_dir.path().join(".fink");
        let prompts_dir = storage_path.join("prompts");
        fs::create_dir_all(&prompts_dir).unwrap();
        
        // Create some test prompts
        let prompt1 = r#"---
name: test-prompt-1
description: First test prompt
tags: []
prompt_type: whole
---
This is the first test prompt."#;
        
        let prompt2 = r#"---
name: test-prompt-2
description: Second test prompt
tags: []
prompt_type: whole
---
This is the second test prompt."#;
        
        let prompt3 = r#"---
name: test-prompt-3
description: Third test prompt
tags: []
prompt_type: whole
---
This is the third test prompt."#;
        
        fs::write(prompts_dir.join("test-prompt-1.md"), prompt1).unwrap();
        fs::write(prompts_dir.join("test-prompt-2.md"), prompt2).unwrap();
        fs::write(prompts_dir.join("test-prompt-3.md"), prompt3).unwrap();
        
        // Create minimal config file
        let config_content = format!(
            r#"editor = "vim"
storage_path = "{}"
clipboard_prefix = ""
clipboard_postfix = ""
"#,
            storage_path.to_str().unwrap()
        );
        fs::write(&config_path, config_content).unwrap();
        
        let config = Config::load_from_file(&config_path).unwrap();
        
        (temp_dir, config)
    }
    
    fn cleanup_test_env() {
        unsafe {
            std::env::remove_var("FINK_TEST_CONFIG_PATH");
            std::env::remove_var("FINK_TEST_STATE_PATH");
        }
    }

    #[test]
    fn test_state_saved_on_navigation() {
        let (_temp_dir, config) = create_test_app_with_prompts();
        let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
        
        // Navigate to the next prompt
        app.next();
        
        // Get the selected prompt name after navigation
        let selected_name = app.get_selected_prompt_name().expect("Should have a selection");
        println!("Selected after navigation: {}", selected_name);
        
        // Verify state was saved
        let state_path = AppState::state_file_path();
        println!("State path: {:?}", state_path);
        println!("State path exists: {}", state_path.exists());
        
        if state_path.exists() {
            // Load state and verify
            let state = AppState::load().unwrap();
            let saved_prompt = state.last_selected_prompt();
            println!("Saved prompt in state: {:?}", saved_prompt);
            assert_eq!(saved_prompt, Some(selected_name.as_str()));
        } else {
            panic!("State file was not created");
        }
        
        cleanup_test_env();
    }

    #[test]
    fn test_state_saved_on_quit() {
        let (_temp_dir, config) = create_test_app_with_prompts();
        let mut app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
        
        // Navigate to the third prompt
        app.next();
        app.next();
        
        // Get the selected prompt name before quitting
        let selected_name = app.get_selected_prompt_name().expect("Should have a selection");
        
        // Quit the app
        app.quit();
        
        // Load state and verify
        let state = AppState::load().unwrap();
        assert_eq!(state.last_selected_prompt(), Some(selected_name.as_str()));
        
        cleanup_test_env();
    }

    #[test]
    fn test_state_restored_on_startup() {
        let (_temp_dir, config) = create_test_app_with_prompts();
        
        // Create and save a state
        let mut state = AppState::new();
        state.set_last_selected_prompt(Some("test-prompt-2".to_string()));
        state.save().unwrap();
        
        // Create a new app - it should restore the cursor position
        let app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
        
        // Verify the selection was restored
        let selected_name = app.get_selected_prompt_name().expect("Should have a selection");
        assert_eq!(selected_name, "test-prompt-2");
        
        cleanup_test_env();
    }

    #[test]
    fn test_state_handles_missing_prompt() {
        let (_temp_dir, config) = create_test_app_with_prompts();
        
        // Create and save a state with a non-existent prompt
        let mut state = AppState::new();
        state.set_last_selected_prompt(Some("non-existent-prompt".to_string()));
        state.save().unwrap();
        
        // Create a new app - it should handle the missing prompt gracefully
        let app = TUIApp::new_with_mode_and_config(&config, AppMode::QuickSelect).unwrap();
        
        // Should have a valid selection (the first prompt in the sorted list)
        let selected_name = app.get_selected_prompt_name();
        assert!(selected_name.is_some(), "Should have a default selection when saved prompt doesn't exist");
        
        // The selected prompt should be one of our test prompts
        let selected = selected_name.unwrap();
        assert!(
            selected.starts_with("test-prompt-"),
            "Selected prompt '{}' should be one of the test prompts",
            selected
        );
        
        cleanup_test_env();
    }
}