use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub enum PromptType {
    Instruction,
    Context,
    InputIndicator,
    OutputIndicator,
    Etc,
    #[default]
    Whole,
}

impl fmt::Display for PromptType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PromptType::Instruction => write!(f, "Instruction"),
            PromptType::Context => write!(f, "Context"),
            PromptType::InputIndicator => write!(f, "Input Indicator"),
            PromptType::OutputIndicator => write!(f, "Output Indicator"),
            PromptType::Etc => write!(f, "Etc"),
            PromptType::Whole => write!(f, "Whole"),
        }
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