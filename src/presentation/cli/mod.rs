use crate::application::repository::{PromptRepository, FileSystemRepository};
use crate::storage::FileSystem;
use anyhow::Result;
use clap::Subcommand;
use std::path::PathBuf;

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
    let storage = FileSystem::new(base_path.clone());
    let repository = FileSystemRepository::new(storage);
    
    match command {
        Commands::List => {
            let prompts = repository.list_all()?;

            if prompts.is_empty() {
                println!("No prompts found");
            } else {
                for prompt in prompts {
                    let tags_str = if prompt.tags.is_empty() {
                        String::new()
                    } else {
                        format!(" [{}]", prompt.tags.join(", "))
                    };
                    println!("{}{}", prompt.name, tags_str);
                }
            }
            Ok(())
        }
        Commands::Get { name } => {
            if let Some(prompt) = repository.find_by_name(&name)? {
                match repository.get_content(&prompt.file_path) {
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
            let normalized_name = name.to_lowercase().replace(' ', "-");
            
            // Check if prompt already exists
            if repository.prompt_exists(&normalized_name) {
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
            
            // Create the prompt using repository
            repository.create_prompt(&normalized_name, &content)?;
            
            Ok(())
        }
    }
}

