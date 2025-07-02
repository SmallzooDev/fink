use anyhow::Result;
use std::path::PathBuf;
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
        
        let content = self.repository.get_content(&metadata.file_path)?;
        
        Ok((metadata, content))
    }

    fn copy_to_clipboard(&self, content: &str) -> Result<()> {
        self.clipboard.borrow_mut().copy(content)
    }

    fn search_prompts(&self, query: &str, search_type: SearchType) -> Result<Vec<PromptMetadata>> {
        self.repository.search(query, search_type)
    }

    fn create_prompt(&self, name: &str, template: Option<&str>) -> Result<()> {
        let normalized_name = name.to_lowercase().replace(' ', "-");
        
        // Check if prompt already exists
        if self.repository.prompt_exists(&normalized_name) {
            return Err(anyhow::anyhow!("Prompt '{}' already exists", name));
        }
        
        let content = match template {
            Some("basic") => {
                format!(r#"---
name: "{}"
tags: []
---
# {}

# Instruction
(a specific task or instruction you want the model to perform)
Please input your prompt's instruction in here!

# Context
(external information or additional context that can steer the model to better responses)
Please input your prompt's context in here!

# Input Data
(the input or question that we are interested to find a response for)
Please input your prompt's input data in here!

# Output Indicator
(the type or format of the output)
Please input your prompt's output indicator here!
"#, name, name)
            }
            Some(template_name) => {
                return Err(anyhow::anyhow!("Unknown template: {}", template_name));
            }
            None => {
                // Create the default content
                format!(r#"---
name: "{}"
tags: []
---
# {}

"#, name, name)
            }
        };
        
        // Create the prompt using repository
        self.repository.create_prompt(&normalized_name, &content)?;
        Ok(())
    }

    fn edit_prompt(&self, name: &str) -> Result<()> {
        // Find the prompt
        let metadata = self.repository.find_by_name(name)?
            .ok_or_else(|| anyhow::anyhow!("Prompt not found: {}", name))?;
        
        // Get the file path
        let file_path = std::path::Path::new(&self.repository.get_base_path())
            .join("jkms")
            .join(&metadata.file_path);
        
        // Launch the editor using the EditorLauncher
        self.editor_launcher.launch(&file_path)?;
        
        Ok(())
    }

    fn delete_prompt(&self, name: &str, force: bool) -> Result<()> {
        // Find the prompt
        let metadata = self.repository.find_by_name(name)?
            .ok_or_else(|| anyhow::anyhow!("Prompt not found: {}", name))?;
        
        // If not forced, we would normally ask for confirmation here
        // For now, we'll implement the force flag behavior
        if !force {
            // In a real implementation, we would prompt for confirmation
            // For CLI testing, we'll skip this for now
            return Err(anyhow::anyhow!("Deletion cancelled. Use --force to skip confirmation."));
        }
        
        // Delete the prompt
        self.repository.delete_prompt(&metadata.file_path)?;
        
        Ok(())
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
}