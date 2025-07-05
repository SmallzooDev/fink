use std::path::Path;
use std::process::Command;
use crate::utils::error::{Result, JkmsError, ExternalError};

pub struct EditorLauncher {
    config_editor: Option<String>,
}

impl EditorLauncher {
    pub fn new() -> Self {
        Self {
            config_editor: None,
        }
    }
    
    pub fn with_editor(editor: &str) -> Self {
        Self {
            config_editor: Some(editor.to_string()),
        }
    }
    
    pub fn get_editor(&self) -> String {
        // First check environment variables (they have highest priority)
        if let Ok(editor) = std::env::var("EDITOR") {
            return editor;
        }
        
        if let Ok(visual) = std::env::var("VISUAL") {
            return visual;
        }
        
        // Then check if we have a config editor
        if let Some(ref editor) = self.config_editor {
            return editor.clone();
        }
        
        // Finally fall back to default
        "vim".to_string()
    }
    
    pub fn create_command(&self, file_path: &Path) -> Command {
        let editor = self.get_editor();
        let mut command = Command::new(editor);
        command.arg(file_path);
        command
    }
    
    pub fn launch(&self, file_path: &Path) -> Result<()> {
        let editor = self.get_editor();
        let status = self.create_command(file_path)
            .status()
            .map_err(|e| JkmsError::External(ExternalError::EditorError(
                format!("Failed to launch editor '{}': {}", editor, e)
            )))?;
        
        if !status.success() {
            return Err(JkmsError::External(ExternalError::EditorError(
                format!("Editor '{}' exited with non-zero status", editor)
            )));
        }
        
        Ok(())
    }
}

impl Default for EditorLauncher {
    fn default() -> Self {
        Self::new()
    }
}