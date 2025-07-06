use clap::Parser;
use fink::presentation::cli::{Commands, execute_command};
use fink::presentation::tui::runner::{run, run_manage_mode};
use fink::utils::config::Config;
use fink::utils::default_prompts::initialize_default_prompts;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Beautiful TUI for managing AI prompts")]
struct Cli {
    /// Path to the prompts directory (overrides config file)
    #[arg(short, long, global = true)]
    path: Option<PathBuf>,

    /// Enter management mode
    #[arg(short, long)]
    manage: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    // Load or create config
    let config_path = Config::default_config_path();
    let mut config = Config::load_or_create(&config_path).unwrap_or_else(|e| {
        eprintln!("Warning: Could not load config: {}. Using defaults.", e);
        Config::default()
    });

    // Override storage path if CLI path is provided
    if let Some(path) = cli.path {
        config.set_storage_path(path);
    }

    let base_path = config.storage_path().to_path_buf();

    // Initialize default prompts if this is the first run
    if let Err(e) = initialize_default_prompts(&base_path) {
        eprintln!("Warning: Failed to initialize default prompts: {}", e);
    }

    let result = match cli.command {
        Some(cmd) => execute_command(cmd, &config),
        None => {
            if cli.manage {
                run_manage_mode(base_path, &config)
            } else {
                run(base_path, &config)
            }
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
