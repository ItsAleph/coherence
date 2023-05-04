use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Parse {
        /// .chrpatch file to parse
        #[arg(short, long)]
        file: String,
    },
    Init {
        /// Path to initialize Coherence repository in
        #[arg(short, long)]
        path: String,
    },
}
