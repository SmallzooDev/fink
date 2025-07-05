use crate::utils::error::{Result, JkmsError, PromptError, ExternalError};
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
        let metadata = self.repository.find_by_name(identifier)
            .map_err(|e| JkmsError::from(e))?
            .ok_or_else(|| JkmsError::Prompt(PromptError::NotFound(identifier.to_string())))?;
        
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
                return Err(JkmsError::Prompt(PromptError::InvalidFormat(format!("Unknown template: {}", template_name))));
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
        self.repository.create_prompt(&normalized_name, &content)
            .map_err(|e| JkmsError::from(e))?;
        Ok(())
    }

    fn edit_prompt(&self, name: &str) -> Result<()> {
        // Find the prompt
        let metadata = self.repository.find_by_name(name)
            .map_err(|e| JkmsError::from(e))?
            .ok_or_else(|| JkmsError::Prompt(PromptError::NotFound(name.to_string())))?;
        
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
        let metadata = self.repository.find_by_name(name)
            .map_err(|e| JkmsError::from(e))?
            .ok_or_else(|| JkmsError::Prompt(PromptError::NotFound(name.to_string())))?;
        
        // If not forced, we would normally ask for confirmation here
        // For now, we'll implement the force flag behavior
        if !force {
            // In a real implementation, we would prompt for confirmation
            // For CLI testing, we'll skip this for now
            return Err(JkmsError::Validation(crate::utils::error::ValidationError::InvalidInput("confirmation", "Deletion cancelled. Use --force to skip confirmation.".to_string())));
        }
        
        // Delete the prompt
        self.repository.delete_prompt(&metadata.file_path)
            .map_err(|e| JkmsError::from(e))?;
        
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

    fn update_prompt_tags(&self, name: &str, tags: Vec<String>) -> Result<()> {
        // Find the prompt
        let metadata = self.repository.find_by_name(name)
            .map_err(|e| JkmsError::from(e))?
            .ok_or_else(|| JkmsError::Prompt(PromptError::NotFound(name.to_string())))?;
        
        // Get the file path
        let file_path = std::path::Path::new(&self.repository.get_base_path())
            .join("jkms")
            .join(&metadata.file_path);
        
        // Read the current content
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| JkmsError::Storage(crate::utils::error::StorageError::Io(e)))?;
        
        // Parse and update the frontmatter
        let updated_content = if content.starts_with("---\n") {
            let parts: Vec<&str> = content.splitn(3, "---\n").collect();
            if parts.len() >= 3 {
                // Parse existing frontmatter
                let frontmatter = parts[1].to_string();
                
                // Update tags in frontmatter
                let lines: Vec<&str> = frontmatter.lines().collect();
                let mut new_lines = Vec::new();
                let mut tags_updated = false;
                
                for line in lines {
                    if line.starts_with("tags:") {
                        new_lines.push(format!("tags: [{}]", 
                            tags.iter()
                                .map(|t| format!("\"{}\"", t))
                                .collect::<Vec<_>>()
                                .join(", ")
                        ));
                        tags_updated = true;
                    } else {
                        new_lines.push(line.to_string());
                    }
                }
                
                // If tags didn't exist, add them
                if !tags_updated {
                    new_lines.push(format!("tags: [{}]", 
                        tags.iter()
                            .map(|t| format!("\"{}\"", t))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
                
                format!("---\n{}\n---\n{}", new_lines.join("\n"), parts[2])
            } else {
                content
            }
        } else {
            // No frontmatter, add it
            format!("---\nname: \"{}\"\ntags: [{}]\n---\n{}", name, 
                tags.iter()
                    .map(|t| format!("\"{}\"", t))
                    .collect::<Vec<_>>()
                    .join(", "),
                content)
        };
        
        // Write the updated content back
        std::fs::write(&file_path, updated_content)
            .map_err(|e| JkmsError::Storage(crate::utils::error::StorageError::Io(e)))?;
        
        Ok(())
    }
}