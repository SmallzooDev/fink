use anyhow::Result;
use crate::application::models::{PromptMetadata, SearchType};
use crate::storage::FileSystem;

/// Repository pattern for data access
pub trait PromptRepository {
    fn list_all(&self) -> Result<Vec<PromptMetadata>>;
    fn find_by_name(&self, name: &str) -> Result<Option<PromptMetadata>>;
    fn get_content(&self, file_path: &str) -> Result<String>;
    fn search(&self, query: &str, search_type: SearchType) -> Result<Vec<PromptMetadata>>;
}

/// Adapter to use FileSystem as a PromptRepository
pub struct FileSystemRepository {
    storage: FileSystem,
}

impl FileSystemRepository {
    pub fn new(storage: FileSystem) -> Self {
        Self { storage }
    }
    
    fn extract_metadata(&self, prompt: &crate::storage::Prompt) -> PromptMetadata {
        let tags = self.extract_tags(&prompt.file_path);
        
        PromptMetadata {
            name: prompt.name.clone(),
            file_path: prompt.file_path.clone(),
            tags,
        }
    }
    
    fn extract_tags(&self, file_path: &str) -> Vec<String> {
        let full_path = self.storage.base_path().join("jkms").join(file_path);
        
        if let Ok(content) = std::fs::read_to_string(&full_path) {
            if content.starts_with("---\n") {
                let parts: Vec<&str> = content.splitn(3, "---\n").collect();
                if parts.len() >= 2 {
                    for line in parts[1].lines() {
                        if line.starts_with("tags:") {
                            let tags_str = line.trim_start_matches("tags:").trim();
                            if tags_str.starts_with('[') && tags_str.ends_with(']') {
                                let tags_str = &tags_str[1..tags_str.len() - 1];
                                return tags_str
                                    .split(',')
                                    .map(|s| s.trim().trim_matches('"').to_string())
                                    .collect();
                            }
                        }
                    }
                }
            }
        }
        Vec::new()
    }
}

impl PromptRepository for FileSystemRepository {
    fn list_all(&self) -> Result<Vec<PromptMetadata>> {
        let prompts = self.storage.list_prompts()?;
        Ok(prompts.into_iter().map(|p| self.extract_metadata(&p)).collect())
    }

    fn find_by_name(&self, name: &str) -> Result<Option<PromptMetadata>> {
        let prompts = self.list_all()?;
        Ok(prompts.into_iter().find(|p| {
            p.name.to_lowercase() == name.to_lowercase()
                || p.file_path.trim_end_matches(".md") == name
        }))
    }

    fn get_content(&self, file_path: &str) -> Result<String> {
        let full_path = self.storage.base_path().join("jkms").join(file_path);
        std::fs::read_to_string(full_path).map_err(Into::into)
    }

    fn search(&self, query: &str, search_type: SearchType) -> Result<Vec<PromptMetadata>> {
        let all_prompts = self.list_all()?;
        let query_lower = query.to_lowercase();
        
        Ok(all_prompts.into_iter().filter(|p| {
            match search_type {
                SearchType::Name => p.name.to_lowercase().contains(&query_lower),
                SearchType::Tags => p.tags.iter().any(|t| t.to_lowercase().contains(&query_lower)),
                SearchType::Content => {
                    if let Ok(content) = self.get_content(&p.file_path) {
                        content.to_lowercase().contains(&query_lower)
                    } else {
                        false
                    }
                }
                SearchType::All => {
                    p.name.to_lowercase().contains(&query_lower)
                        || p.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
                        || if let Ok(content) = self.get_content(&p.file_path) {
                            content.to_lowercase().contains(&query_lower)
                        } else {
                            false
                        }
                }
            }
        }).collect())
    }
}