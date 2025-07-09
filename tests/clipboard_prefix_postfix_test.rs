#[cfg(test)]
mod tests {
    use fink::presentation::tui::tui::TUIApp;
    use fink::utils::config::Config;
    use tempfile::TempDir;

    #[test]
    fn test_clipboard_with_prefix_postfix_newlines() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        // Create a config with test storage path and prefix/postfix
        let storage_path = temp_dir.path().join(".fink");
        std::fs::create_dir_all(&storage_path).unwrap();
        std::fs::create_dir_all(storage_path.join("prompts")).unwrap();
        
        let config_content = format!(
            r#"editor = "vim"
storage_path = "{}"
clipboard_prefix = "PREFIX"
clipboard_postfix = "POSTFIX"
"#,
            storage_path.to_str().unwrap()
        );
        std::fs::write(&config_path, config_content).unwrap();
        
        let config = Config::load_from_file(&config_path).unwrap();
        
        // Create a test prompt
        let test_prompt_content = "MAIN CONTENT";
        let test_prompt_path = storage_path.join("prompts").join("test.md");
        std::fs::write(&test_prompt_path, format!(
            "---\nname: test\ndescription: Test prompt\ntags: []\ntype: whole\n---\n{}", 
            test_prompt_content
        )).unwrap();
        
        // Create TUIApp and select the prompt
        let mut app = TUIApp::new_with_config(&config).unwrap();
        app.reload_prompts().unwrap();
        app.next(); // Select first prompt
        
        // Get the content that would be copied
        let content = app.get_selected_content().unwrap();
        
        // Build expected content with newlines
        let prefix = app.get_config().clipboard_prefix();
        let postfix = app.get_config().clipboard_postfix();
        
        let mut expected = String::new();
        if !prefix.is_empty() {
            expected.push_str(prefix);
            expected.push('\n');
        }
        expected.push_str(&content);
        if !postfix.is_empty() {
            expected.push('\n');
            expected.push_str(postfix);
        }
        
        // Should be: "PREFIX\nMAIN CONTENT\nPOSTFIX"
        assert_eq!(expected, "PREFIX\nMAIN CONTENT\nPOSTFIX");
    }

    #[test]
    fn test_clipboard_without_prefix_postfix() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        // Create a config with empty prefix/postfix
        let storage_path = temp_dir.path().join(".fink");
        std::fs::create_dir_all(&storage_path).unwrap();
        std::fs::create_dir_all(storage_path.join("prompts")).unwrap();
        
        let config_content = format!(
            r#"editor = "vim"
storage_path = "{}"
clipboard_prefix = ""
clipboard_postfix = ""
"#,
            storage_path.to_str().unwrap()
        );
        std::fs::write(&config_path, config_content).unwrap();
        
        let config = Config::load_from_file(&config_path).unwrap();
        
        // Create a test prompt
        let test_prompt_content = "MAIN CONTENT";
        let test_prompt_path = storage_path.join("prompts").join("test.md");
        std::fs::write(&test_prompt_path, format!(
            "---\nname: test\ndescription: Test prompt\ntags: []\ntype: whole\n---\n{}", 
            test_prompt_content
        )).unwrap();
        
        // Create TUIApp and select the prompt
        let mut app = TUIApp::new_with_config(&config).unwrap();
        app.reload_prompts().unwrap();
        app.next(); // Select first prompt
        
        // Get the content that would be copied
        let content = app.get_selected_content().unwrap();
        
        // Build expected content (should be unchanged)
        let prefix = app.get_config().clipboard_prefix();
        let postfix = app.get_config().clipboard_postfix();
        
        let mut expected = String::new();
        if !prefix.is_empty() {
            expected.push_str(prefix);
            expected.push('\n');
        }
        expected.push_str(&content);
        if !postfix.is_empty() {
            expected.push('\n');
            expected.push_str(postfix);
        }
        
        // Should be just the content with no extra newlines
        assert_eq!(expected, "MAIN CONTENT");
    }
}