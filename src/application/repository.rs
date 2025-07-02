use anyhow::Result;
use crate::application::models::{PromptMetadata, SearchType};
use crate::storage::FileSystem;
use std::path::Path;

/// Repository pattern for data access
pub trait PromptRepository {
    fn list_all(&self) -> Result<Vec<PromptMetadata>>;
    fn find_by_name(&self, name: &str) -> Result<Option<PromptMetadata>>;
    fn get_content(&self, file_path: &str) -> Result<String>;
    fn search(&self, query: &str, search_type: SearchType) -> Result<Vec<PromptMetadata>>;
    fn create_prompt(&self, name: &str, content: &str) -> Result<()>;
    fn prompt_exists(&self, name: &str) -> bool;
    fn get_template_content(&self, template_name: &str) -> Result<String>;
    fn get_base_path(&self) -> &std::path::Path;
    fn delete_prompt(&self, file_path: &str) -> Result<()>;
}

/// Adapter to use FileSystem as a PromptRepository
pub struct FileSystemRepository {
    storage: FileSystem,
}

impl FileSystemRepository {
    pub fn new(storage: FileSystem) -> Self {
        Self { storage }
    }
}

impl PromptRepository for FileSystemRepository {
    fn list_all(&self) -> Result<Vec<PromptMetadata>> {
        self.storage.list_prompts()
    }

    fn find_by_name(&self, name: &str) -> Result<Option<PromptMetadata>> {
        let prompts = self.list_all()?;
        Ok(prompts.into_iter().find(|p| {
            p.name.to_lowercase() == name.to_lowercase()
                || p.file_path.trim_end_matches(".md") == name
        }))
    }

    fn get_content(&self, file_path: &str) -> Result<String> {
        let relative_path = Path::new("jkms").join(file_path);
        let content = self.storage.read_to_string(&relative_path)?;
        
        // Extract content after frontmatter
        if content.starts_with("---\n") {
            let parts: Vec<&str> = content.splitn(3, "---\n").collect();
            if parts.len() >= 3 {
                return Ok(parts[2].trim().to_string());
            }
        }
        
        Ok(content)
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

    fn create_prompt(&self, name: &str, content: &str) -> Result<()> {
        let prompts_dir = Path::new("jkms");
        self.storage.create_dir_all(prompts_dir)?;
        
        let file_name = format!("{}.md", name);
        let file_path = prompts_dir.join(&file_name);
        
        self.storage.write(&file_path, content)?;
        Ok(())
    }

    fn prompt_exists(&self, name: &str) -> bool {
        let file_name = format!("{}.md", name);
        let file_path = Path::new("jkms").join(&file_name);
        self.storage.exists(&file_path)
    }

    fn get_template_content(&self, template_name: &str) -> Result<String> {
        let template_path = Path::new("templates").join(format!("{}.md", template_name));
        self.storage.read_to_string(&template_path)
    }

    fn get_base_path(&self) -> &std::path::Path {
        self.storage.base_path()
    }

    fn delete_prompt(&self, file_path: &str) -> Result<()> {
        let full_path = Path::new("jkms").join(file_path);
        self.storage.delete(&full_path)?;
        Ok(())
    }
}