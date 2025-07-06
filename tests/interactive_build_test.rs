#[cfg(test)]
mod tests {
    use fink::presentation::tui::tui::TUIApp;
    use fink::presentation::tui::components::BuildStep;
    use fink::application::models::PromptType;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_interactive_build_panel_steps() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Create jkms directory
        let jkms_path = temp_path.join("fink");
        fs::create_dir_all(&jkms_path).unwrap();
        
        // Create prompts of different types
        let instruction_prompt = r#"---
name: "instruction-1"
tags: ["test"]
type: "instruction"
---
You are a helpful assistant."#;
        
        let context_prompt = r#"---
name: "context-1"
tags: ["test"]
type: "context"
---
Context information here."#;
        
        fs::write(jkms_path.join("instruction.md"), instruction_prompt).unwrap();
        fs::write(jkms_path.join("context.md"), context_prompt).unwrap();
        
        // Create TUIApp and enter build mode
        let mut app = TUIApp::new(temp_path).unwrap();
        app.reload_prompts().unwrap();
        app.enter_build_mode();
        
        // Verify interactive panel is created
        assert!(app.get_interactive_build_panel().is_some());
        
        if let Some(panel) = app.get_interactive_build_panel_mut() {
            // Should start with instruction selection
            assert_eq!(panel.current_step, BuildStep::SelectInstruction);
            
            // Get options (should include "None" and available instruction prompts)
            let options = panel.get_current_options();
            assert!(options.len() >= 2); // At least "None" and one instruction
            assert!(options[0].contains("None"));
            
            // Select first instruction prompt
            panel.next(); // Move from "None" to first prompt
            panel.select_current();
            
            // Should move to context selection
            assert_eq!(panel.current_step, BuildStep::SelectContext);
            
            // Select "None" for context
            panel.select_current(); // Select "None" (default selection)
            
            // Should continue through remaining steps
            assert_eq!(panel.current_step, BuildStep::SelectInputIndicator);
        }
    }

    #[test]
    fn test_interactive_build_with_comment() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Create jkms directory
        let jkms_path = temp_path.join("fink");
        fs::create_dir_all(&jkms_path).unwrap();
        
        // Create a simple prompt
        let prompt = r#"---
name: "test-prompt"
tags: []
type: "instruction"
---
Test content"#;
        
        fs::write(jkms_path.join("test.md"), prompt).unwrap();
        
        // Create TUIApp and enter build mode
        let mut app = TUIApp::new(temp_path).unwrap();
        app.reload_prompts().unwrap();
        app.enter_build_mode();
        
        if let Some(panel) = app.get_interactive_build_panel_mut() {
            // Skip through all prompt selections by selecting "None"
            while !matches!(panel.current_step, BuildStep::AddComment) {
                panel.select_current(); // Select "None" for each type
            }
            
            // Should now be at comment step
            assert_eq!(panel.current_step, BuildStep::AddComment);
            
            // Add a comment
            panel.add_comment_char('T');
            panel.add_comment_char('e');
            panel.add_comment_char('s');
            panel.add_comment_char('t');
            
            assert_eq!(panel.get_comment(), "Test");
            
            // Finish comment
            panel.finish_comment();
            
            // Should be complete
            assert_eq!(panel.current_step, BuildStep::Complete);
            assert!(panel.is_complete());
        }
    }
    
    #[test]
    fn test_get_selected_prompt_names() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Create jkms directory
        let jkms_path = temp_path.join("fink");
        fs::create_dir_all(&jkms_path).unwrap();
        
        // Create prompts
        let instruction = r#"---
name: "my-instruction"
tags: []
type: "instruction"
---
Instruction content"#;
        
        let context = r#"---
name: "my-context"
tags: []
type: "context"
---
Context content"#;
        
        fs::write(jkms_path.join("instruction.md"), instruction).unwrap();
        fs::write(jkms_path.join("context.md"), context).unwrap();
        
        // Create TUIApp and enter build mode
        let mut app = TUIApp::new(temp_path).unwrap();
        app.reload_prompts().unwrap();
        app.enter_build_mode();
        
        if let Some(panel) = app.get_interactive_build_panel_mut() {
            // Select instruction
            panel.next(); // Move to first instruction
            panel.select_current();
            
            // Select context
            panel.next(); // Move to first context
            panel.select_current();
            
            // Skip remaining types
            while !matches!(panel.current_step, BuildStep::AddComment) {
                panel.select_current();
            }
            
            // Get selected prompts
            let selected = panel.get_selected_prompt_names();
            assert_eq!(selected.len(), 2);
            assert_eq!(selected[0], (PromptType::Instruction, "my-instruction".to_string()));
            assert_eq!(selected[1], (PromptType::Context, "my-context".to_string()));
        }
    }
}