use anyhow::Result;
use std::path::PathBuf;

pub struct PromptManager {
    base_path: PathBuf,
}

impl PromptManager {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    pub fn get_prompt_content(&self, filename: &str) -> Result<String> {
        let path = self.base_path.join("jkms").join(filename);
        let content = std::fs::read_to_string(path)?;

        // Extract content after frontmatter
        if content.starts_with("---\n") {
            let parts: Vec<&str> = content.splitn(3, "---\n").collect();
            if parts.len() >= 3 {
                return Ok(parts[2].trim().to_string());
            }
        }

        Ok(content)
    }
}
