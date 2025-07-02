use anyhow::Result;
use crate::application::models::{PromptMetadata, PromptFilter, SearchType};

/// Application layer for business operations
pub trait PromptApplication {
    fn list_prompts(&self, filter: Option<PromptFilter>) -> Result<Vec<PromptMetadata>>;
    fn get_prompt(&self, identifier: &str) -> Result<(PromptMetadata, String)>;
    fn copy_to_clipboard(&self, content: &str) -> Result<()>;
    fn search_prompts(&self, query: &str, search_type: SearchType) -> Result<Vec<PromptMetadata>>;
    fn create_prompt(&self, name: &str, template: Option<&str>) -> Result<()>;
}