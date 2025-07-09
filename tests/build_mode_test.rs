#[cfg(test)]
mod tests {
    use fink::presentation::tui::app::AppMode;
    use fink::presentation::tui::app::TUIApp;
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
    fn should_have_build_mode_in_app_mode_enum() {
        // This test will verify that AppMode::Build exists
        let mode = AppMode::Build;
        assert!(matches!(mode, AppMode::Build));
    }

    #[test]
    fn should_enter_build_mode_on_b_key() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Create jkms directory
        let jkms_path = temp_path.join("prompts");
        fs::create_dir_all(&jkms_path).unwrap();
        
        // Create TUIApp
        let mut app = TUIApp::new(temp_path.clone()).unwrap();
        
        // Initially should be in QuickSelect mode
        assert_eq!(*app.mode(), AppMode::QuickSelect);
        
        // Enter build mode
        app.enter_build_mode();
        
        // Should now be in Build mode
        assert_eq!(*app.mode(), AppMode::Build);
    }

    #[test]
    fn should_exit_build_mode_on_escape() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Create jkms directory
        let jkms_path = temp_path.join("prompts");
        fs::create_dir_all(&jkms_path).unwrap();
        
        // Create TUIApp
        let mut app = TUIApp::new(temp_path.clone()).unwrap();
        
        // Enter build mode
        app.enter_build_mode();
        assert_eq!(*app.mode(), AppMode::Build);
        
        // Exit build mode
        app.exit_build_mode();
        
        // Should return to QuickSelect mode
        assert_eq!(*app.mode(), AppMode::QuickSelect);
    }

    #[test]
    fn should_get_prompts_filtered_by_type_in_build_mode() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        
        // Create jkms directory
        let jkms_path = temp_path.join("prompts");
        fs::create_dir_all(&jkms_path).unwrap();
        
        // Create prompts with different types
        let instruction_prompt = r#"---
name: "instruction-prompt"
tags: []
type: "instruction"
---
# Instruction prompt"#;
        
        let whole_prompt = r#"---
name: "whole-prompt"
tags: []
type: "whole"
---
# Whole prompt"#;
        
        fs::write(jkms_path.join("instruction.md"), instruction_prompt).unwrap();
        fs::write(jkms_path.join("whole.md"), whole_prompt).unwrap();
        
        // Create TUIApp and reload prompts
        let mut app = create_test_app(&temp_path);
        app.reload_prompts().unwrap();
        
        // Enter build mode
        app.enter_build_mode();
        
        // Get build prompts (should exclude 'whole' type)
        let build_prompts = app.get_build_prompts();
        
        assert_eq!(build_prompts.len(), 1);
        assert_eq!(build_prompts[0].name, "instruction-prompt");
    }
}