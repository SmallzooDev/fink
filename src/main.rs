use clap::Parser;
use jkms::presentation::cli::{Commands, execute_command};
use jkms::presentation::tui::runner::run;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Beautiful TUI for managing AI prompts")]
struct Cli {
    /// Path to the prompts directory (defaults to ~/.jkms)
    #[arg(short, long, global = true)]
    path: Option<PathBuf>,

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
        None => run(base_path),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
