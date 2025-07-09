use std::path::Path;
use std::process::Command;
use crate::utils::error::{Result, FinkError, ExternalError};

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
        // First check if we have a config editor (config has highest priority)
        if let Some(ref editor) = self.config_editor {
            return editor.clone();
        }
        
        // Then check environment variables
        if let Ok(editor) = std::env::var("EDITOR") {
            return editor;
        }
        
        if let Ok(visual) = std::env::var("VISUAL") {
            return visual;
        }
        
        // Finally fall back to default
        "vim".to_string()
    }
    
    pub fn is_vscode(&self) -> bool {
        let editor = self.get_editor();
        editor == "code" || editor == "code-insiders" || editor.ends_with("/code") || editor.ends_with("/code-insiders")
    }
    
    pub fn create_command(&self, file_path: &Path) -> Command {
        let editor = self.get_editor();
        
        // Special handling for VS Code on macOS
        if self.is_vscode() && cfg!(target_os = "macos") {
            let mut command = Command::new("open");
            command.arg("-n");
            command.arg("-b");
            command.arg("com.microsoft.VSCode");
            command.arg("--args");
            command.arg("--wait");
            command.arg(file_path);
            command
        } else {
            let mut command = Command::new(&editor);
            
            // Special handling for VS Code
            if self.is_vscode() {
                // Add --wait flag to make VS Code wait until the file is closed
                command.arg("--wait");
            }
            
            command.arg(file_path);
            command
        }
    }
    
    pub fn launch(&self, file_path: &Path) -> Result<()> {
        let editor = self.get_editor();
        let status = self.create_command(file_path)
            .status()
            .map_err(|e| FinkError::External(ExternalError::EditorError(
                format!("Failed to launch editor '{}': {}", editor, e)
            )))?;
        
        if !status.success() {
            return Err(FinkError::External(ExternalError::EditorError(
                format!("Editor '{}' exited with non-zero status", editor)
            )));
        }
        
        Ok(())
    }
    
    pub fn launch_at_line(&self, file_path: &Path, line: usize) -> Result<()> {
        let editor = self.get_editor();
        
        let mut command = if self.is_vscode() && cfg!(target_os = "macos") {
            // VS Code on macOS needs special handling
            let mut cmd = Command::new("open");
            cmd.arg("-n");
            cmd.arg("-b");
            cmd.arg("com.microsoft.VSCode");
            cmd.arg("--args");
            cmd.arg("--wait");
            cmd.arg("--goto");
            cmd.arg(format!("{}:{}", file_path.display(), line));
            cmd
        } else if self.is_vscode() {
            // VS Code on other platforms
            let mut cmd = Command::new(&editor);
            cmd.arg("--wait");
            cmd.arg("--goto");
            cmd.arg(format!("{}:{}", file_path.display(), line));
            cmd
        } else if editor.contains("vim") || editor.contains("nvim") || editor.contains("vi") {
            // Vim-like editors use +line syntax
            let mut cmd = Command::new(&editor);
            cmd.arg(format!("+{}", line));
            cmd.arg(file_path);
            cmd
        } else if editor.contains("emacs") {
            // Emacs uses +line:column syntax
            let mut cmd = Command::new(&editor);
            cmd.arg(format!("+{}:1", line));
            cmd.arg(file_path);
            cmd
        } else if editor.contains("nano") {
            // Nano uses +line syntax
            let mut cmd = Command::new(&editor);
            cmd.arg(format!("+{}", line));
            cmd.arg(file_path);
            cmd
        } else if editor.contains("helix") || editor == "hx" {
            // Helix uses file:line syntax
            let mut cmd = Command::new(&editor);
            cmd.arg(format!("{}:{}", file_path.display(), line));
            cmd
        } else {
            // Default: just open the file
            let mut cmd = Command::new(&editor);
            cmd.arg(file_path);
            cmd
        };
        
        let status = command
            .status()
            .map_err(|e| FinkError::External(ExternalError::EditorError(
                format!("Failed to launch editor '{}': {}", editor, e)
            )))?;
        
        if !status.success() {
            return Err(FinkError::External(ExternalError::EditorError(
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