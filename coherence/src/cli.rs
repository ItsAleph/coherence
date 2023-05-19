use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Parse .chrpatch file and pretty-print resulting CHRFile
    Parse {
        /// .chrpatch file to parse
        #[arg(long)]
        file: String,
    },
    /// Initialize Coherence repository
    Init {
        /// Path to initialize Coherence repository in
        #[arg(long)]
        path: Option<PathBuf>,
    },
    /// Commit changes
    Commit {
        /// Commit message
        #[arg(short = 'm')]
        msg: String,
    },
}
