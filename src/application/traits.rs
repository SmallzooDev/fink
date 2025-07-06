use crate::utils::error::Result;
use crate::application::models::{PromptMetadata, PromptFilter, SearchType, PromptType};

/// Application layer for business operations
pub trait PromptApplication {
    fn list_prompts(&self, filter: Option<PromptFilter>) -> Result<Vec<PromptMetadata>>;
    fn get_prompt(&self, identifier: &str) -> Result<(PromptMetadata, String)>;
    fn copy_to_clipboard(&self, content: &str) -> Result<()>;
    fn search_prompts(&self, query: &str, search_type: SearchType) -> Result<Vec<PromptMetadata>>;
    fn create_prompt(&self, name: &str, template: Option<&str>) -> Result<()>;
    fn create_prompt_with_content(&self, name: &str, template: Option<&str>, content: Option<String>) -> Result<()>;
    fn create_prompt_with_type(&self, name: &str, template: Option<&str>, prompt_type: PromptType) -> Result<()>;
    fn create_prompt_with_content_and_type(&self, name: &str, template: Option<&str>, content: Option<String>, prompt_type: PromptType) -> Result<()>;
    fn edit_prompt(&self, name: &str) -> Result<()>;
    fn delete_prompt(&self, name: &str, force: bool) -> Result<()>;
    fn copy_prompt(&self, name: &str) -> Result<()>;
    fn get_base_path(&self) -> &std::path::Path;
    fn update_prompt_tags(&self, name: &str, tags: Vec<String>) -> Result<()>;
    fn get_clipboard_content(&self) -> Result<String>;
}