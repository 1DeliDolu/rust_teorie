use anyhow::Context;
use clap::{Parser, Subcommand};

use error_handling_project::read_first_number;

#[derive(Parser)]
#[command(name = "error_handling_project")]
#[command(about = "Demo for Rust error handling (thiserror + anyhow + clap)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read first number from a file
    Read { path: String },
    /// Generate a sample file with a given integer as first line
    Generate { path: String, value: i32 },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Read { path } => {
            let n = read_first_number(&path).context("reading first number failed")?;
            println!("First number: {}", n);
        }
        Commands::Generate { path, value } => {
            use std::fs::OpenOptions;
            use std::io::Write;

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .with_context(|| format!("failed to open sample file '{}'", path))?;

            writeln!(file, "{}", value)
                .with_context(|| format!("failed to append to sample file '{}'", path))?;

            println!("Appended value to sample file: {}", path);
        }
    }

    Ok(())
}
