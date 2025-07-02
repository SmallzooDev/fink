use clap::Parser;
use jkms::presentation::cli::{Commands, execute_command};
use jkms::presentation::tui::runner::{run, run_manage_mode};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Beautiful TUI for managing AI prompts")]
struct Cli {
    /// Path to the prompts directory (defaults to ~/.jkms)
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

    let base_path = cli.path.unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".jkms")
    });

    let result = match cli.command {
        Some(cmd) => execute_command(cmd, base_path),
        None => {
            if cli.manage {
                run_manage_mode(base_path)
            } else {
                run(base_path)
            }
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
