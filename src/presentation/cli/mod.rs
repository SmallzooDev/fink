use crate::storage::FileSystem;
use anyhow::Result;
use clap::Subcommand;
use std::path::{Path, PathBuf};

#[derive(Subcommand)]
pub enum Commands {
    /// List all prompts
    List,
    /// Get a specific prompt
    Get {
        /// Name of the prompt
        name: String,
    },
    /// Create a new prompt
    Create {
        /// Name of the prompt
        name: String,
        /// Template to use for the prompt
        #[arg(short, long)]
        template: Option<String>,
    },
}


pub fn execute_command(command: Commands, base_path: PathBuf) -> Result<()> {
    match command {
        Commands::List => {
            let storage = FileSystem::new(base_path.clone());
            let prompts = storage.list_prompts()?;

            if prompts.is_empty() {
                println!("No prompts found");
            } else {
                for prompt in prompts {
                    // Extract tags from the prompt content
                    let tags = extract_tags(&base_path, &prompt);
                    let tags_str = if tags.is_empty() {
                        String::new()
                    } else {
                        format!(" [{}]", tags.join(", "))
                    };
                    println!("{}{}", prompt.name, tags_str);
                }
            }
            Ok(())
        }
        Commands::Get { name } => {
            let storage = FileSystem::new(base_path.clone());
            let prompts = storage.list_prompts()?;

            // Find prompt by name or filename
            let prompt = prompts.iter().find(|p| {
                p.name.to_lowercase() == name.to_lowercase()
                    || p.file_path.trim_end_matches(".md") == name
            });

            if let Some(prompt) = prompt {
                let manager = crate::core::PromptManager::new(base_path);
                match manager.get_prompt_content(&prompt.file_path) {
                    Ok(content) => {
                        println!("{}", content);
                        Ok(())
                    }
                    Err(e) => {
                        eprintln!("Error reading prompt: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Prompt not found: {}", name);
                std::process::exit(1);
            }
        }
        Commands::Create { name, template } => {
            let prompts_dir = base_path.join("jkms");
            
            // Ensure the prompts directory exists
            std::fs::create_dir_all(&prompts_dir)?;
            
            // Create the filename from the name
            let filename = format!("{}.md", name.to_lowercase().replace(' ', "-"));
            let file_path = prompts_dir.join(&filename);
            
            // Check if file already exists
            if file_path.exists() {
                eprintln!("Error: Prompt '{}' already exists", name);
                std::process::exit(1);
            }
            
            let content = if let Some(template_name) = template {
                match template_name.as_str() {
                    "basic" => {
                        // TODO: In the future, this will open an external editor with the template
                        // For now, we'll just create the file with the template content
                        format!(r#"---
name: "{}"
tags: []
---
# {}

# Instruction
(a specific task or instruction you want the model to perform)
Please input your prompt's instruction in here!

# Context
(external information or additional context that can steer the model to better responses)
Please input your prompt's context in here!

# Input Data
(the input or question that we are interested to find a response for)
Please input your prompt's input data in here!

# Output Indicator
(the type or format of the output)
Please input your prompt's output indicator here!
"#, name, name)
                    }
                    _ => {
                        eprintln!("Error: Unknown template: {}", template_name);
                        std::process::exit(1);
                    }
                }
            } else {
                // Create the default content
                format!(r#"---
name: "{}"
tags: []
---
# {}

"#, name, name)
            };
            
            // Write the file
            std::fs::write(&file_path, content)?;
            
            Ok(())
        }
    }
}

fn extract_tags(base_path: &Path, prompt: &crate::storage::Prompt) -> Vec<String> {
    // For now, we'll need to read the file to get tags
    // This is a simple implementation
    let full_path = base_path.join("jkms").join(&prompt.file_path);
    if let Ok(content) = std::fs::read_to_string(&full_path) {
        if content.starts_with("---\n") {
            let parts: Vec<&str> = content.splitn(3, "---\n").collect();
            if parts.len() >= 2 {
                for line in parts[1].lines() {
                    if line.starts_with("tags:") {
                        let tags_str = line.trim_start_matches("tags:").trim();
                        // Parse YAML array format: ["tag1", "tag2"]
                        if tags_str.starts_with('[') && tags_str.ends_with(']') {
                            let tags_str = &tags_str[1..tags_str.len() - 1];
                            return tags_str
                                .split(',')
                                .map(|s| s.trim().trim_matches('"').to_string())
                                .collect();
                        }
                    }
                }
            }
        }
    }
    Vec::new()
}
