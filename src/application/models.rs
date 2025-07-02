use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptMetadata {
    pub name: String,
    pub file_path: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PromptFilter {
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub enum SearchType {
    Name,
    Content,
    Tags,
    All,
}