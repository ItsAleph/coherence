pub mod ast;
pub mod chrpatch;
pub mod cli;
pub mod commands;
pub mod utils;
pub mod built;
pub mod macros;

use crate::cli::{Cli, Commands};
use crate::commands::{init::main as init, parse::main as parse};
use clap::Parser;
use colored::Colorize;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Parse { file }) => parse(file)?,
        Some(Commands::Init { path }) => init(path)?,
        _ => {
            anyhow::bail!("Unknown command. Run {} for help.", "chr help".green());
        }
    };

    Ok(())
}
