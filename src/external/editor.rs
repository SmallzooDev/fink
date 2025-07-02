use std::path::Path;
use std::process::Command;
use crate::utils::error::{Result, JkmsError, ExternalError};

pub struct EditorLauncher;

impl EditorLauncher {
    pub fn new() -> Self {
        Self
    }
    
    pub fn get_editor(&self) -> String {
        std::env::var("EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .unwrap_or_else(|_| "vim".to_string())
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