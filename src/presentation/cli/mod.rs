use crate::application::application::DefaultPromptApplication;
use crate::application::traits::PromptApplication;
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
    /// Edit an existing prompt
    Edit {
        /// Name of the prompt to edit
        name: String,
    },
    /// Delete a prompt
    Delete {
        /// Name of the prompt to delete
        name: String,
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
    /// Copy a prompt to clipboard
    Copy {
        /// Name of the prompt to copy
        name: String,
    },
}


pub fn execute_command(command: Commands, base_path: PathBuf) -> Result<()> {
    let application = DefaultPromptApplication::new(base_path)?;
    
    match command {
        Commands::List => {
            let prompts = application.list_prompts(None)?;

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
            match application.get_prompt(&name) {
                Ok((_, content)) => {
                    println!("{}", content);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Create { name, template } => {
            match application.create_prompt(&name, template.as_deref()) {
                Ok(()) => Ok(()),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Edit { name } => {
            match application.edit_prompt(&name) {
                Ok(()) => Ok(()),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Delete { name, force } => {
            match application.delete_prompt(&name, force) {
                Ok(()) => Ok(()),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Copy { name } => {
            match application.copy_prompt(&name) {
                Ok(()) => {
                    println!("Copied to clipboard");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

