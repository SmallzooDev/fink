pub mod editor;

use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};

pub struct ClipboardManager {
    context: ClipboardContext,
}

impl ClipboardManager {
    pub fn new() -> Self {
        Self {
            context: ClipboardContext::new().expect("Failed to initialize clipboard"),
        }
    }

    pub fn copy(&mut self, text: &str) -> Result<()> {
        self.context
            .set_contents(text.to_string())
            .map_err(|e| anyhow::anyhow!("Failed to copy to clipboard: {}", e))
    }
    
    pub fn get_content(&mut self) -> Result<String> {
        self.context
            .get_contents()
            .map_err(|e| anyhow::anyhow!("Failed to get clipboard content: {}", e))
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new()
    }
}
