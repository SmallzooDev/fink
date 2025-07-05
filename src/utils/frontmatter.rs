use crate::utils::error::{Result, JkmsError, StorageError};

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
    
    fn update_existing_frontmatter(content: &str, tags: &[String]) -> Result<String> {
        let parts: Vec<&str> = content.splitn(3, FRONTMATTER_DELIMITER).collect();
        
        if parts.len() < 3 {
            return Err(JkmsError::Storage(StorageError::ParseError(
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