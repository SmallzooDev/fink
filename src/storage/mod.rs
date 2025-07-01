use anyhow::Result;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Prompt {
    pub name: String,
    pub file_path: String,
}

pub struct FileSystem {
    base_path: PathBuf,
}

impl FileSystem {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    pub fn list_prompts(&self) -> Result<Vec<Prompt>> {
        let prompts_dir = self.base_path.join("jkms");
        let mut prompts = Vec::new();

        if prompts_dir.exists() {
            for entry in std::fs::read_dir(prompts_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    let content = std::fs::read_to_string(&path)?;

                    // Simple parsing to get the name from frontmatter
                    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                    if let Some(name) = extract_name_from_content(&content) {
                        prompts.push(Prompt {
                            name,
                            file_path: file_name.to_string(),
                        });
                    } else {
                        // Use filename as fallback
                        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                            prompts.push(Prompt {
                                name: stem.to_string(),
                                file_path: file_name.to_string(),
                            });
                        }
                    }
                }
            }
        }

        Ok(prompts)
    }
}

fn extract_name_from_content(content: &str) -> Option<String> {
    // Very simple frontmatter parsing for now
    if content.starts_with("---\n") {
        let parts: Vec<&str> = content.splitn(3, "---\n").collect();
        if parts.len() >= 2 {
            for line in parts[1].lines() {
                if line.starts_with("name: ") {
                    let name = line.trim_start_matches("name: ").trim().trim_matches('"');
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}
