use anyhow::Result;
use std::path::{Path, PathBuf};
use crate::application::models::PromptMetadata;
use crate::utils::constants::PROMPTS_DIR;

pub struct FileSystem {
    base_path: PathBuf,
}

impl FileSystem {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    pub fn create_dir_all(&self, relative_path: &Path) -> Result<()> {
        let full_path = self.base_path.join(relative_path);
        std::fs::create_dir_all(full_path)?;
        Ok(())
    }

    pub fn exists(&self, relative_path: &Path) -> bool {
        self.base_path.join(relative_path).exists()
    }

    pub fn write(&self, relative_path: &Path, content: &str) -> Result<()> {
        let full_path = self.base_path.join(relative_path);
        std::fs::write(full_path, content)?;
        Ok(())
    }

    pub fn read_to_string(&self, relative_path: &Path) -> Result<String> {
        let full_path = self.base_path.join(relative_path);
        let content = std::fs::read_to_string(full_path)?;
        Ok(content)
    }

    pub fn join(&self, relative_path: &Path) -> PathBuf {
        self.base_path.join(relative_path)
    }

    pub fn list_prompts(&self) -> Result<Vec<PromptMetadata>> {
        let prompts_dir = self.base_path.join(PROMPTS_DIR);
        let mut prompts = Vec::new();
        

        if prompts_dir.exists() {
            for entry in std::fs::read_dir(&prompts_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    let content = std::fs::read_to_string(&path)?;
                    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                    
                    // Extract metadata from content
                    let name = extract_name_from_content(&content)
                        .or_else(|| path.file_stem().and_then(|s| s.to_str()).map(|s| s.to_string()))
                        .unwrap_or_else(|| file_name.to_string());
                    
                    let tags = extract_tags_from_content(&content);
                    let type_option = extract_type_from_content(&content);
                    
                    // If no type or invalid type, update the file with default type
                    let prompt_type = if type_option.is_none() {
                        // Update file to add type: "whole"
                        if let Ok(updated_content) = crate::utils::frontmatter::FrontmatterUpdater::ensure_type(&content, &name, type_option) {
                            // Write the updated content back to file
                            // Calculate relative path from base_path
                            let relative_path = path.strip_prefix(&self.base_path)
                                .unwrap_or(&path);
                            if let Err(e) = self.write(relative_path, &updated_content) {
                                eprintln!("Warning: Failed to update type in {}: {}", file_name, e);
                            }
                        }
                        crate::application::models::PromptType::default()
                    } else {
                        type_option.unwrap()
                    };
                    
                    prompts.push(PromptMetadata {
                        name,
                        file_path: file_name.to_string(),
                        tags,
                        prompt_type,
                    });
                }
            }
        }

        Ok(prompts)
    }

    pub fn delete(&self, relative_path: &Path) -> Result<()> {
        let full_path = self.base_path.join(relative_path);
        std::fs::remove_file(full_path)?;
        Ok(())
    }
}

fn extract_name_from_content(content: &str) -> Option<String> {
    // Very simple front-matter parsing for now
    if content.starts_with("---\n") {
        let parts: Vec<&str> = content.splitn(3, "---\n").collect();
        if parts.len() >= 2 {
            for line in parts[1].lines() {
                if line.starts_with("name: ") {
                    let name = line.trim_start_matches("name: ").trim().trim_matches('"');
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

fn extract_tags_from_content(content: &str) -> Vec<String> {
    if let Some(start) = content.find("---") {
        if let Some(end) = content[start + 3..].find("---") {
            let frontmatter = &content[start + 3..start + 3 + end];
            
            // Look for tags line
            for line in frontmatter.lines() {
                if line.trim().starts_with("tags:") {
                    let tags_part = line.trim_start_matches("tags:").trim();
                    
                    // Parse array format [tag1, tag2]
                    if tags_part.starts_with('[') && tags_part.ends_with(']') {
                        let tags_str = &tags_part[1..tags_part.len() - 1];
                        return tags_str
                            .split(',')
                            .map(|s| s.trim().trim_matches('"').to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                }
            }
        }
    }
    Vec::new()
}

pub fn extract_type_from_content(content: &str) -> Option<crate::application::models::PromptType> {
    if let Some(start) = content.find("---") {
        if let Some(end) = content[start + 3..].find("---") {
            let frontmatter = &content[start + 3..start + 3 + end];
            
            // Look for type line
            for line in frontmatter.lines() {
                if line.trim().starts_with("type:") {
                    let type_part = line.trim_start_matches("type:").trim().trim_matches('"');
                    
                    // Map string to PromptType enum
                    return match type_part.to_lowercase().as_str() {
                        "instruction" => Some(crate::application::models::PromptType::Instruction),
                        "context" => Some(crate::application::models::PromptType::Context),
                        "input_indicator" => Some(crate::application::models::PromptType::InputIndicator),
                        "output_indicator" => Some(crate::application::models::PromptType::OutputIndicator),
                        "etc" => Some(crate::application::models::PromptType::Etc),
                        "whole" => Some(crate::application::models::PromptType::Whole),
                        _ => None, // Invalid type value
                    };
                }
            }
        }
    }
    None // No type field found
}
