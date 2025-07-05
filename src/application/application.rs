use crate::utils::error::{Result, JkmsError, PromptError, ExternalError, StorageError};
use crate::utils::frontmatter::FrontmatterUpdater;
use crate::utils::templates::TemplateGenerator;
use std::path::{Path, PathBuf};
use std::cell::RefCell;
use crate::application::models::{PromptMetadata, PromptFilter, SearchType};
use crate::application::repository::{PromptRepository, FileSystemRepository};
use crate::application::traits::PromptApplication;
use crate::storage::FileSystem;
use crate::external::{ClipboardManager, editor::EditorLauncher};

pub struct DefaultPromptApplication {
    repository: Box<dyn PromptRepository>,
    clipboard: RefCell<ClipboardManager>,
    editor_launcher: EditorLauncher,
}

impl DefaultPromptApplication {
    pub fn new(base_path: PathBuf) -> Result<Self> {
        let storage = FileSystem::new(base_path);
        let repository = Box::new(FileSystemRepository::new(storage));
        let clipboard = RefCell::new(ClipboardManager::new());

        Ok(Self {
            repository,
            clipboard,
            editor_launcher: EditorLauncher::new(),
        })
    }
    
    // Helper methods for cleaner code
    fn find_prompt_metadata(&self, name: &str) -> Result<PromptMetadata> {
        self.repository.find_by_name(name)
            .map_err(|e| JkmsError::from(e))?
            .ok_or_else(|| JkmsError::Prompt(PromptError::NotFound(name.to_string())))
    }
    
    fn get_prompt_file_path(&self, metadata: &PromptMetadata) -> PathBuf {
        Path::new(&self.repository.get_base_path())
            .join("jkms")
            .join(&metadata.file_path)
    }
    
    fn read_prompt_file(&self, path: &Path) -> Result<String> {
        std::fs::read_to_string(path)
            .map_err(|e| JkmsError::Storage(StorageError::Io(e)))
    }
    
    fn write_prompt_file(&self, path: &Path, content: &str) -> Result<()> {
        std::fs::write(path, content)
            .map_err(|e| JkmsError::Storage(StorageError::Io(e)))
    }
}

impl PromptApplication for DefaultPromptApplication {
    fn list_prompts(&self, filter: Option<PromptFilter>) -> Result<Vec<PromptMetadata>> {
        let mut prompts = self.repository.list_all()
            .map_err(|e| JkmsError::from(e))?;
        
        if let Some(filter) = filter {
            if let Some(tags) = filter.tags {
                prompts.retain(|p| p.tags.iter().any(|t| tags.contains(t)));
            }
        }
        
        Ok(prompts)
    }

    fn get_prompt(&self, identifier: &str) -> Result<(PromptMetadata, String)> {
        let metadata = self.find_prompt_metadata(identifier)?;
        
        let content = self.repository.get_content(&metadata.file_path)
            .map_err(|e| JkmsError::from(e))?;
        
        Ok((metadata, content))
    }

    fn copy_to_clipboard(&self, content: &str) -> Result<()> {
        self.clipboard.borrow_mut().copy(content)
            .map_err(|e| JkmsError::External(ExternalError::ClipboardError(e.to_string())))
    }

    fn search_prompts(&self, query: &str, search_type: SearchType) -> Result<Vec<PromptMetadata>> {
        self.repository.search(query, search_type)
            .map_err(|e| JkmsError::from(e))
    }

    fn create_prompt(&self, name: &str, template: Option<&str>) -> Result<()> {
        let normalized_name = name.to_lowercase().replace(' ', "-");
        
        // Check if prompt already exists
        if self.repository.prompt_exists(&normalized_name) {
            return Err(JkmsError::Prompt(PromptError::AlreadyExists(name.to_string())));
        }
        
        let content = TemplateGenerator::generate(name, template)?;
        
        // Create the prompt using repository
        self.repository.create_prompt(&normalized_name, &content)
            .map_err(|e| JkmsError::from(e))?;
        Ok(())
    }

    fn edit_prompt(&self, name: &str) -> Result<()> {
        let metadata = self.find_prompt_metadata(name)?;
        let file_path = self.get_prompt_file_path(&metadata);
        
        self.editor_launcher.launch(&file_path)?;
        
        Ok(())
    }

    fn delete_prompt(&self, name: &str, force: bool) -> Result<()> {
        let metadata = self.find_prompt_metadata(name)?;
        
        if !force {
            return Err(JkmsError::Validation(crate::utils::error::ValidationError::InvalidInput(
                "confirmation", 
                "Deletion cancelled. Use --force to skip confirmation.".to_string()
            )));
        }
        
        self.repository.delete_prompt(&metadata.file_path)
            .map_err(|e| JkmsError::from(e))
    }

    fn copy_prompt(&self, name: &str) -> Result<()> {
        // Get the prompt content
        let (_, content) = self.get_prompt(name)?;
        
        // Copy to clipboard
        self.copy_to_clipboard(&content)?;
        
        Ok(())
    }

    fn get_base_path(&self) -> &std::path::Path {
        self.repository.get_base_path()
    }

    fn update_prompt_tags(&self, name: &str, tags: Vec<String>) -> Result<()> {
        let metadata = self.find_prompt_metadata(name)?;
        let file_path = self.get_prompt_file_path(&metadata);
        
        let content = self.read_prompt_file(&file_path)?;
        let updated_content = FrontmatterUpdater::update_tags(&content, name, &tags)?;
        
        self.write_prompt_file(&file_path, &updated_content)?;
        
        Ok(())
    }
}