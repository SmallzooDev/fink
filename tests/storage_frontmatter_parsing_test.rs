#[cfg(test)]
mod tests {
    use jkms::application::models::PromptType;
    
    #[test]
    fn should_extract_type_from_frontmatter() {
        let content = r#"---
name: "Test Prompt"
tags: ["test", "example"]
type: "instruction"
---

# Test Content"#;

        let prompt_type = jkms::storage::extract_type_from_content(content);
        assert_eq!(prompt_type, Some(PromptType::Instruction));
    }

    #[test]
    fn should_return_default_type_when_missing() {
        let content = r#"---
name: "Test Prompt"
tags: ["test"]
---

# Test Content"#;

        let prompt_type = jkms::storage::extract_type_from_content(content);
        assert_eq!(prompt_type, None);
    }

    #[test]
    fn should_handle_different_type_values() {
        let test_cases = vec![
            ("context", PromptType::Context),
            ("input_indicator", PromptType::InputIndicator),
            ("output_indicator", PromptType::OutputIndicator),
            ("etc", PromptType::Etc),
            ("whole", PromptType::Whole),
        ];

        for (type_str, expected) in test_cases {
            let content = format!(r#"---
name: "Test"
type: "{}"
---
Content"#, type_str);

            let prompt_type = jkms::storage::extract_type_from_content(&content);
            assert_eq!(prompt_type, Some(expected));
        }
    }

    #[test]
    fn should_handle_invalid_type_value() {
        let content = r#"---
name: "Test"
type: "invalid_type"
---
Content"#;

        let prompt_type = jkms::storage::extract_type_from_content(&content);
        assert_eq!(prompt_type, None);
    }
}