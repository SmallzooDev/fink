use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use crate::utils::error::Result;
use crate::utils::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// Last selected prompt name
    last_selected_prompt: Option<String>,
    /// Last active filters
    #[serde(default)]
    last_tag_filters: Vec<String>,
    /// Last search query
    #[serde(default)]
    last_search_query: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            last_selected_prompt: None,
            last_tag_filters: Vec::new(),
            last_search_query: String::new(),
        }
    }
    
    pub fn last_selected_prompt(&self) -> Option<&str> {
        self.last_selected_prompt.as_deref()
    }
    
    pub fn set_last_selected_prompt(&mut self, prompt_name: Option<String>) {
        self.last_selected_prompt = prompt_name;
    }
    
    pub fn last_tag_filters(&self) -> &[String] {
        &self.last_tag_filters
    }
    
    pub fn set_last_tag_filters(&mut self, filters: Vec<String>) {
        self.last_tag_filters = filters;
    }
    
    pub fn last_search_query(&self) -> &str {
        &self.last_search_query
    }
    
    pub fn set_last_search_query(&mut self, query: String) {
        self.last_search_query = query;
    }
    
    pub fn state_file_path() -> PathBuf {
        // Check for test environment variable first
        if let Ok(test_state_path) = std::env::var("FINK_TEST_STATE_PATH") {
            return PathBuf::from(test_state_path);
        }
        
        Config::config_dir().join("state.json")
    }
    
    pub fn load() -> Result<Self> {
        let path = Self::state_file_path();
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&content).unwrap_or_else(|_| Self::new()))
        } else {
            Ok(Self::new())
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let path = Self::state_file_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}