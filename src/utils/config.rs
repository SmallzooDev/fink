use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use crate::utils::error::{Result, FinkError, StorageError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    editor: String,
    storage_path: PathBuf,
    #[serde(default)]
    clipboard_prefix: String,
    #[serde(default)]
    clipboard_postfix: String,
}

impl Config {
    pub fn editor(&self) -> &str {
        &self.editor
    }
    
    pub fn set_editor(&mut self, editor: String) {
        self.editor = editor;
    }
    
    pub fn storage_path(&self) -> &Path {
        &self.storage_path
    }
    
    pub fn set_storage_path(&mut self, path: PathBuf) {
        self.storage_path = path;
    }
    
    pub fn clipboard_prefix(&self) -> &str {
        &self.clipboard_prefix
    }
    
    pub fn set_clipboard_prefix(&mut self, prefix: String) {
        self.clipboard_prefix = prefix;
    }
    
    pub fn clipboard_postfix(&self) -> &str {
        &self.clipboard_postfix
    }
    
    pub fn set_clipboard_postfix(&mut self, postfix: String) {
        self.clipboard_postfix = postfix;
    }
    
    pub fn default_config_path() -> PathBuf {
        // Check for test environment variable first
        if let Ok(test_config_path) = std::env::var("FINK_TEST_CONFIG_PATH") {
            return PathBuf::from(test_config_path);
        }
        
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".config/fink/config.toml")
    }
    
    pub fn config_dir() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".config/fink")
    }
    
    pub fn load_from_file(config_path: &Path) -> Result<Self> {
        let content = fs::read_to_string(config_path)
            .map_err(|e| FinkError::Storage(StorageError::Io(e)))?;
        
        let mut config: Self = toml::from_str(&content)
            .map_err(|e| FinkError::Storage(StorageError::ParseError(e.to_string())))?;
        
        // Expand ~ in storage_path
        if let Some(path_str) = config.storage_path.to_str() {
            if path_str.starts_with("~/") {
                if let Some(home) = dirs::home_dir() {
                    let expanded = path_str.replacen("~/", &format!("{}/", home.display()), 1);
                    config.storage_path = PathBuf::from(expanded);
                }
            }
        }
        
        // Fix the path if it ends with /prompts (it shouldn't)
        if config.storage_path.ends_with("prompts") {
            config.storage_path = config.storage_path.parent()
                .unwrap_or(&config.storage_path)
                .to_path_buf();
        }
        
        Ok(config)
    }
    
    pub fn load_or_create(config_path: &Path) -> Result<Self> {
        Self::ensure_config_exists(config_path)?;
        Self::load_from_file(config_path)
    }
    
    pub fn save(&self, config_path: &Path) -> Result<()> {
        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| FinkError::Storage(StorageError::ParseError(e.to_string())))?;
        
        fs::write(config_path, toml_str)
            .map_err(|e| FinkError::Storage(StorageError::Io(e)))?;
        
        Ok(())
    }
    
    pub fn ensure_config_exists(config_path: &Path) -> Result<()> {
        if !config_path.exists() {
            // Create parent directories
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| FinkError::Storage(StorageError::Io(e)))?;
            }
            
            let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
            let default_storage = home.join(".fink");
            
            // Write default config
            let default_config = format!(r#"# fink configuration file

# Default editor for editing prompts
editor = "vim"

# Path where prompts are stored
storage_path = "{}"

# Text to prepend to copied prompts
clipboard_prefix = ""

# Text to append to copied prompts
clipboard_postfix = ""
"#, default_storage.display());
            
            fs::write(config_path, default_config)
                .map_err(|e| FinkError::Storage(StorageError::Io(e)))?;
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        Self {
            editor: "vim".to_string(),
            storage_path: home.join(".fink"),
            clipboard_prefix: String::new(),
            clipboard_postfix: String::new(),
        }
    }
}