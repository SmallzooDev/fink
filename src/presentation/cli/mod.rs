use crate::application::application::DefaultPromptApplication;
use crate::application::traits::PromptApplication;
use anyhow::Result;
use crate::utils::error::JkmsError;
use clap::Subcommand;
use std::path::PathBuf;

fn handle_error(error: JkmsError) -> ! {
    eprintln!("Error: {}", error);
    
    // Show user-friendly message if available
    if error.is_recoverable() {
        eprintln!("\n{}", error.user_message());
    }
    
    std::process::exit(1);
}

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
    /// Search for prompts
    Search {
        /// Search query
        query: String,
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
                Err(e) => handle_error(e),
            }
        }
        Commands::Create { name, template } => {
            match application.create_prompt(&name, template.as_deref()) {
                Ok(()) => Ok(()),
                Err(e) => handle_error(e),
            }
        }
        Commands::Edit { name } => {
            match application.edit_prompt(&name) {
                Ok(()) => Ok(()),
                Err(e) => handle_error(e),
            }
        }
        Commands::Delete { name, force } => {
            match application.delete_prompt(&name, force) {
                Ok(()) => Ok(()),
                Err(e) => handle_error(e),
            }
        }
        Commands::Copy { name } => {
            match application.copy_prompt(&name) {
                Ok(()) => {
                    println!("Copied to clipboard");
                    Ok(())
                }
                Err(e) => handle_error(e),
            }
        }
        Commands::Search { query } => {
            use crate::application::models::SearchType;
            let results = application.search_prompts(&query, SearchType::All)?;
            
            if results.is_empty() {
                println!("No prompts found matching '{}'", query);
            } else {
                for prompt in results {
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
    }
}

