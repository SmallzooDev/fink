use anyhow::Result;
use std::path::PathBuf;
use std::cell::RefCell;
use crate::application::models::{PromptMetadata, PromptFilter, SearchType};
use crate::application::repository::{PromptRepository, FileSystemRepository};
use crate::application::traits::PromptApplication;
use crate::storage::FileSystem;
use crate::core::PromptManager;
use crate::external::ClipboardManager;

pub struct DefaultPromptApplication {
    repository: Box<dyn PromptRepository>,
    prompt_manager: PromptManager,
    clipboard: RefCell<ClipboardManager>,
}

impl DefaultPromptApplication {
    pub fn new(base_path: PathBuf) -> Result<Self> {
        let storage = FileSystem::new(base_path.clone());
        let repository = Box::new(FileSystemRepository::new(storage));
        let prompt_manager = PromptManager::new(base_path);
        let clipboard = RefCell::new(ClipboardManager::new());

        Ok(Self {
            repository,
            prompt_manager,
            clipboard,
        })
    }
}

impl PromptApplication for DefaultPromptApplication {
    fn list_prompts(&self, filter: Option<PromptFilter>) -> Result<Vec<PromptMetadata>> {
        let mut prompts = self.repository.list_all()?;
        
        if let Some(filter) = filter {
            if let Some(tags) = filter.tags {
                prompts.retain(|p| p.tags.iter().any(|t| tags.contains(t)));
            }
        }
        
        Ok(prompts)
    }

    fn get_prompt(&self, identifier: &str) -> Result<(PromptMetadata, String)> {
        let metadata = self.repository.find_by_name(identifier)?
            .ok_or_else(|| anyhow::anyhow!("Prompt not found: {}", identifier))?;
        
        let content = self.prompt_manager.get_prompt_content(&metadata.file_path)?;
        
        Ok((metadata, content))
    }

    fn copy_to_clipboard(&self, content: &str) -> Result<()> {
        self.clipboard.borrow_mut().copy(content)
    }

    fn search_prompts(&self, query: &str, search_type: SearchType) -> Result<Vec<PromptMetadata>> {
        self.repository.search(query, search_type)
    }
}