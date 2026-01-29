use clap::Parser;
use std::path::PathBuf;

/// git-atomic: create atomic commits & branches from a single changeset.
#[derive(Debug, Parser)]
#[command(name = "git-atomic", version, about)]
pub struct Cli {
    /// Path to config file.
    #[arg(long, default_value = ".atomic.toml")]
    pub config: PathBuf,

    /// Increase verbosity (-v, -vv).
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Suppress non-error output.
    #[arg(short, long)]
    pub quiet: bool,

    /// Output as JSON.
    #[arg(long)]
    pub json: bool,
}
