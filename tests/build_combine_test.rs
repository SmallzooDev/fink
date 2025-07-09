#[cfg(test)]
mod tests {
    use fink::presentation::tui::tui::TUIApp;
    use fink::application::models::PromptType;
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
    fn should_combine_prompts_in_correct_order() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Create jkms directory
        let jkms_path = temp_path.join("prompts");
        fs::create_dir_all(&jkms_path).unwrap();
        
        // Create prompts of different types
        let instruction_prompt = r#"---
name: "instruction-prompt"
tags: []
type: "instruction"
---
You are a helpful assistant."#;
        
        let context_prompt = r#"---
name: "context-prompt"
tags: []
type: "context"
---
Context: This is a test scenario."#;
        
        let output_prompt = r#"---
name: "output-prompt"
tags: []
type: "output_indicator"
---
Output format: JSON"#;
        
        fs::write(jkms_path.join("instruction.md"), instruction_prompt).unwrap();
        fs::write(jkms_path.join("context.md"), context_prompt).unwrap();
        fs::write(jkms_path.join("output.md"), output_prompt).unwrap();
        
        // Create TUIApp and enter build mode
        let mut app = create_test_app(&temp_path);
        app.reload_prompts().unwrap();
        app.enter_build_mode();
        
        // Select prompts using interactive build panel
        if let Some(panel) = app.get_interactive_build_panel_mut() {
            // Select instruction prompt
            panel.select_current(); // This will select the instruction prompt and move to next step
            
            // Select context prompt
            panel.select_current(); // This will select the context prompt and move to next step
            
            // Skip input indicator
            panel.select_current(); // Select "None" for input indicator
            
            // Select output prompt
            panel.next(); // Move to the output prompt option
            panel.select_current(); // Select the output prompt
            
            // Skip etc
            panel.select_current(); // Select "None" for etc
            
            // Skip comment
            panel.finish_comment(); // Skip adding comment
        }
        
        // Combine and copy
        let result = app.combine_and_copy_selected_prompts();
        assert!(result.is_ok());
        
        // Verify we exited build mode
        assert!(!app.is_build_mode());
    }
    
    #[test]
    fn should_error_when_no_prompts_selected() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Create jkms directory
        let jkms_path = temp_path.join("prompts");
        fs::create_dir_all(&jkms_path).unwrap();
        
        // Create a prompt
        let prompt = r#"---
name: "test-prompt"
tags: []
type: "instruction"
---
Test content"#;
        
        fs::write(jkms_path.join("test.md"), prompt).unwrap();
        
        // Create TUIApp and enter build mode
        let mut app = create_test_app(&temp_path);
        app.reload_prompts().unwrap();
        app.enter_build_mode();
        
        // Don't select any prompts and try to combine
        let result = app.combine_and_copy_selected_prompts();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No prompts selected");
    }
    
    #[test]
    fn should_filter_out_whole_type_prompts() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Create jkms directory
        let jkms_path = temp_path.join("prompts");
        fs::create_dir_all(&jkms_path).unwrap();
        
        // Create prompts with different types including whole
        let instruction_prompt = r#"---
name: "instruction-prompt"
tags: []
type: "instruction"
---
Instruction content"#;
        
        let whole_prompt = r#"---
name: "whole-prompt"
tags: []
type: "whole"
---
Whole prompt content"#;
        
        fs::write(jkms_path.join("instruction.md"), instruction_prompt).unwrap();
        fs::write(jkms_path.join("whole.md"), whole_prompt).unwrap();
        
        // Create TUIApp and enter build mode
        let mut app = create_test_app(&temp_path);
        app.reload_prompts().unwrap();
        app.enter_build_mode();
        
        // Verify that whole type is filtered out
        let build_prompts = app.get_build_prompts();
        assert_eq!(build_prompts.len(), 1);
        assert_eq!(build_prompts[0].name, "instruction-prompt");
        assert_ne!(build_prompts[0].prompt_type, PromptType::Whole);
    }
}