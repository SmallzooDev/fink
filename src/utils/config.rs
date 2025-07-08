use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use crate::utils::error::{Result, FinkError, StorageError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    editor: String,
    storage_path: PathBuf,
}

impl Config {
    pub fn editor(&self) -> &str {
        &self.editor
    }
    
    pub fn storage_path(&self) -> &Path {
        &self.storage_path
    }
    
    pub fn set_storage_path(&mut self, path: PathBuf) {
        self.storage_path = path;
    }
    
    pub fn default_config_path() -> PathBuf {
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
        
        toml::from_str(&content)
            .map_err(|e| FinkError::Storage(StorageError::ParseError(e.to_string())))
    }
    
    pub fn load_or_create(config_path: &Path) -> Result<Self> {
        Self::ensure_config_exists(config_path)?;
        Self::load_from_file(config_path)
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
            let default_config = format!(r#"# jkms configuration file

# Default editor for editing prompts
editor = "vim"

# Path where prompts are stored
storage_path = "{}"
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
        }
    }
}