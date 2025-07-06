use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PromptType {
    Instruction,
    Context,
    InputIndicator,
    OutputIndicator,
    Etc,
    Whole,
}

impl Default for PromptType {
    fn default() -> Self {
        PromptType::Whole
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptMetadata {
    pub name: String,
    pub file_path: String,
    pub tags: Vec<String>,
    pub prompt_type: PromptType,
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