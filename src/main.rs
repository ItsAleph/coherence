pub mod ast;
pub mod chrpatch;
pub mod cli;
pub mod commands;
pub mod utils;

use crate::cli::{Cli, Commands};
use crate::commands::parse::main as parse;
use clap::Parser;
use colored::Colorize;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let _ = match &cli.command {
        Some(Commands::Parse { file }) => parse(file),
        None => {
            return Err(anyhow::anyhow!(
                "Unknown command. Run {} for help.",
                "chr help".green()
            ))
        }
    };

    Ok(())
}
