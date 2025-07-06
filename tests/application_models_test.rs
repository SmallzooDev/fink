#[cfg(test)]
mod tests {
    use fink::application::models::{PromptMetadata, PromptType};

    #[test]
    fn should_create_prompt_metadata_with_type() {
        let metadata = PromptMetadata {
            name: "test-prompt".to_string(),
            file_path: "/path/to/test.md".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            prompt_type: PromptType::Instruction,
        };

        assert_eq!(metadata.name, "test-prompt");
        assert_eq!(metadata.prompt_type, PromptType::Instruction);
    }

    #[test]
    fn should_have_default_type_as_whole() {
        let metadata = PromptMetadata {
            name: "test-prompt".to_string(),
            file_path: "/path/to/test.md".to_string(),
            tags: vec![],
            prompt_type: PromptType::default(),
        };

        assert_eq!(metadata.prompt_type, PromptType::Whole);
    }
}