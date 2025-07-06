use crate::utils::error::{Result, FinkError, StorageError};
use crate::application::models::PromptType;

const FRONTMATTER_DELIMITER: &str = "---\n";

pub struct FrontmatterUpdater;

impl FrontmatterUpdater {
    /// Updates tags in the content, preserving all other frontmatter fields
    pub fn update_tags(content: &str, name: &str, tags: &[String]) -> Result<String> {
        if content.starts_with(FRONTMATTER_DELIMITER) {
            Self::update_existing_frontmatter(content, tags)
        } else {
            Ok(Self::add_new_frontmatter(content, name, tags))
        }
    }
    
    /// Ensures the content has a type field, adding it if missing or invalid
    pub fn ensure_type(content: &str, name: &str, current_type: Option<PromptType>) -> Result<String> {
        if content.starts_with(FRONTMATTER_DELIMITER) {
            Self::ensure_type_in_existing_frontmatter(content, current_type)
        } else {
            Ok(Self::add_new_frontmatter_with_type(content, name, &[], PromptType::default()))
        }
    }
    
    fn update_existing_frontmatter(content: &str, tags: &[String]) -> Result<String> {
        let parts: Vec<&str> = content.splitn(3, FRONTMATTER_DELIMITER).collect();
        
        if parts.len() < 3 {
            return Err(FinkError::Storage(StorageError::ParseError(
                "Invalid frontmatter format".to_string()
            )));
        }
        
        let updated_frontmatter = Self::update_tags_in_frontmatter(parts[1], tags);
        Ok(format!("{}{}{}{}", 
            FRONTMATTER_DELIMITER, 
            updated_frontmatter, 
            FRONTMATTER_DELIMITER, 
            parts[2]
        ))
    }
    
    fn update_tags_in_frontmatter(frontmatter: &str, tags: &[String]) -> String {
        let mut lines: Vec<String> = Vec::new();
        let mut tags_updated = false;
        
        for line in frontmatter.lines() {
            if line.starts_with("tags:") {
                lines.push(TagFormatter::format_tags_line(tags));
                tags_updated = true;
            } else {
                lines.push(line.to_string());
            }
        }
        
        // Add tags if they didn't exist
        if !tags_updated {
            lines.push(TagFormatter::format_tags_line(tags));
        }
        
        lines.join("\n") + "\n"
    }
    
    fn add_new_frontmatter(content: &str, name: &str, tags: &[String]) -> String {
        format!(
            "{}name: \"{}\"\n{}\n{}{}",
            FRONTMATTER_DELIMITER,
            name,
            TagFormatter::format_tags_line(tags),
            FRONTMATTER_DELIMITER,
            content
        )
    }
    
    fn add_new_frontmatter_with_type(content: &str, name: &str, tags: &[String], prompt_type: PromptType) -> String {
        format!(
            "{}name: \"{}\"\n{}\ntype: \"{}\"\n{}{}",
            FRONTMATTER_DELIMITER,
            name,
            TagFormatter::format_tags_line(tags),
            Self::prompt_type_to_string(prompt_type),
            FRONTMATTER_DELIMITER,
            content
        )
    }
    
    fn ensure_type_in_existing_frontmatter(content: &str, current_type: Option<PromptType>) -> Result<String> {
        let parts: Vec<&str> = content.splitn(3, FRONTMATTER_DELIMITER).collect();
        
        if parts.len() < 3 {
            return Err(FinkError::Storage(StorageError::ParseError(
                "Invalid frontmatter format".to_string()
            )));
        }
        
        let mut lines: Vec<String> = Vec::new();
        let mut type_found = false;
        
        for line in parts[1].lines() {
            if line.trim().starts_with("type:") {
                type_found = true;
                // Only update if current_type is None (invalid) or line has invalid value
                if current_type.is_none() {
                    lines.push(format!("type: \"{}\"", Self::prompt_type_to_string(PromptType::default())));
                } else {
                    lines.push(line.to_string());
                }
            } else {
                lines.push(line.to_string());
            }
        }
        
        // Add type field if it wasn't found
        if !type_found {
            lines.push(format!("type: \"{}\"", Self::prompt_type_to_string(PromptType::default())));
        }
        
        Ok(format!("{}{}\n{}{}", 
            FRONTMATTER_DELIMITER, 
            lines.join("\n"), 
            FRONTMATTER_DELIMITER, 
            parts[2]
        ))
    }
    
    fn prompt_type_to_string(prompt_type: PromptType) -> &'static str {
        match prompt_type {
            PromptType::Instruction => "instruction",
            PromptType::Context => "context",
            PromptType::InputIndicator => "input_indicator",
            PromptType::OutputIndicator => "output_indicator",
            PromptType::Etc => "etc",
            PromptType::Whole => "whole",
        }
    }
}

pub struct TagFormatter;

impl TagFormatter {
    /// Formats tags for YAML frontmatter
    pub fn format_tags_line(tags: &[String]) -> String {
        if tags.is_empty() {
            "tags: []".to_string()
        } else {
            format!(
                "tags: [{}]",
                tags.iter()
                    .map(|tag| format!("\"{}\"", tag))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_empty_tags() {
        let result = TagFormatter::format_tags_line(&[]);
        assert_eq!(result, "tags: []");
    }
    
    #[test]
    fn test_format_single_tag() {
        let tags = vec!["test".to_string()];
        let result = TagFormatter::format_tags_line(&tags);
        assert_eq!(result, r#"tags: ["test"]"#);
    }
    
    #[test]
    fn test_format_multiple_tags() {
        let tags = vec!["tag1".to_string(), "tag2".to_string()];
        let result = TagFormatter::format_tags_line(&tags);
        assert_eq!(result, r#"tags: ["tag1", "tag2"]"#);
    }
    
    #[test]
    fn test_update_tags_with_existing_frontmatter() {
        let content = r#"---
name: "test"
tags: ["old"]
---
Body"#;
        let tags = vec!["new".to_string()];
        let result = FrontmatterUpdater::update_tags(content, "test", &tags).unwrap();
        
        assert!(result.contains(r#"tags: ["new"]"#));
        assert!(!result.contains("old"));
        assert!(result.contains("Body"));
    }
    
    #[test]
    fn test_add_frontmatter_when_missing() {
        let content = "Just body content";
        let tags = vec!["tag1".to_string()];
        let result = FrontmatterUpdater::update_tags(content, "test", &tags).unwrap();
        
        assert!(result.starts_with("---\n"));
        assert!(result.contains(r#"name: "test""#));
        assert!(result.contains(r#"tags: ["tag1"]"#));
        assert!(result.contains("Just body content"));
    }
}