use clap::Parser;
use jkms::cli::run;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Beautiful TUI for managing AI prompts")]
struct Cli {
    /// Path to the prompts directory (defaults to ~/.jkms)
    #[arg(short, long)]
    path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let base_path = cli.path.unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Could not find home directory")
            .join(".jkms")
    });

    if let Err(e) = run(base_path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
