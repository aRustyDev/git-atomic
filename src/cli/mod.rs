pub mod commands;
pub mod output;

use clap::{Parser, Subcommand};
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

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Atomize a commit into per-component branches (default).
    Atomize(AtomizeArgs),
    /// Show component/branch status for a commit.
    Status(StatusArgs),
    /// Validate the configuration file.
    Validate,
}

#[derive(Debug, Parser)]
pub struct AtomizeArgs {
    /// Source commit to atomize.
    #[arg(long, default_value = "HEAD")]
    pub commit: String,

    /// Commit range (start..end) to atomize.
    #[arg(long)]
    pub range: Option<String>,

    /// Preview changes without mutating refs.
    #[arg(long)]
    pub dry_run: bool,

    /// Force-update diverged branches.
    #[arg(long)]
    pub force: bool,

    /// CI mode: atomize + push, fail on error.
    #[arg(long)]
    pub ci_mode: bool,

    /// Push branches after atomizing.
    #[arg(long)]
    pub push: bool,

    /// Remote to push to.
    #[arg(long, default_value = "origin")]
    pub remote: String,
}

#[derive(Debug, Parser)]
pub struct StatusArgs {
    /// Commit to inspect.
    #[arg(long, default_value = "HEAD")]
    pub commit: String,
}
