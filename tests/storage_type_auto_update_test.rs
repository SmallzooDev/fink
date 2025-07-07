#[cfg(test)]
mod tests {
    use fink::application::models::PromptType;
    use fink::utils::frontmatter::FrontmatterUpdater;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn should_add_type_field_when_missing() {
        let content = r#"---
name: "Test Prompt"
tags: ["test"]
---

# Test Content"#;

        let result = FrontmatterUpdater::ensure_type(content, "Test Prompt", None).unwrap();
        
        assert!(result.contains("type: \"whole\""));
        assert!(result.contains("name: \"Test Prompt\""));
        assert!(result.contains("tags: [\"test\"]"));
        assert!(result.contains("# Test Content"));
    }

    #[test]
    fn should_update_invalid_type_to_whole() {
        let content = r#"---
name: "Test Prompt"
type: "invalid"
---

# Test Content"#;

        let result = FrontmatterUpdater::ensure_type(content, "Test Prompt", None).unwrap();
        
        assert!(result.contains("type: \"whole\""));
        assert!(!result.contains("type: \"invalid\""));
    }

    #[test]
    fn should_preserve_valid_type() {
        let content = r#"---
name: "Test Prompt"
type: "instruction"
---

# Test Content"#;

        let result = FrontmatterUpdater::ensure_type(content, "Test Prompt", Some(PromptType::Instruction)).unwrap();
        
        assert!(result.contains("type: \"instruction\""));
        assert!(!result.contains("type: \"whole\""));
    }

    #[test]
    fn should_auto_update_files_on_list() {
        use fink::storage::FileSystem;

        let temp_dir = TempDir::new().unwrap();
        let jkms_dir = temp_dir.path().join("prompts");
        fs::create_dir_all(&jkms_dir).unwrap();
        
        let prompt_path = jkms_dir.join("test-prompt.md");
        
        // Create a prompt file without type
        let content = r#"---
name: "Test Prompt"
tags: ["test"]
---

# Test Content"#;
        fs::write(&prompt_path, content).unwrap();

        // Create FileSystem and list prompts
        let fs = FileSystem::new(temp_dir.path().to_path_buf());
        let prompts = fs.list_prompts().unwrap();

        // Verify prompt has default type
        assert_eq!(prompts.len(), 1);
        assert_eq!(prompts[0].prompt_type, PromptType::Whole);

        // Verify file was updated
        let updated_content = fs::read_to_string(&prompt_path).unwrap();
        assert!(updated_content.contains("type: \"whole\""));
    }
}