#[cfg(test)]
mod tests {
    use jkms::presentation::tui::tui::TUIApp;
    use jkms::application::models::PromptType;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn should_combine_prompts_in_correct_order() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Create jkms directory
        let jkms_path = temp_path.join("jkms");
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
        let mut app = TUIApp::new(temp_path.clone()).unwrap();
        app.reload_prompts().unwrap();
        app.enter_build_mode();
        
        // Select all prompts
        if let Some(build_panel) = app.get_build_panel_mut() {
            build_panel.toggle_selection(); // Select first (instruction)
            build_panel.next();
            build_panel.toggle_selection(); // Select second (context)
            build_panel.next();
            build_panel.toggle_selection(); // Select third (output)
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
        let jkms_path = temp_path.join("jkms");
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
        let mut app = TUIApp::new(temp_path.clone()).unwrap();
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
        let jkms_path = temp_path.join("jkms");
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
        let mut app = TUIApp::new(temp_path.clone()).unwrap();
        app.reload_prompts().unwrap();
        app.enter_build_mode();
        
        // Verify that whole type is filtered out
        let build_prompts = app.get_build_prompts();
        assert_eq!(build_prompts.len(), 1);
        assert_eq!(build_prompts[0].name, "instruction-prompt");
        assert_ne!(build_prompts[0].prompt_type, PromptType::Whole);
    }
}