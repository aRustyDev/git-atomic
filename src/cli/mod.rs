pub mod commands;
pub mod output;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Create atomic commits & branches from a single changeset.
///
/// Splits a multi-component commit into isolated per-component branches,
/// each containing only the files that belong to that component.
/// Run without a subcommand to split HEAD with default settings.
#[derive(Debug, Parser)]
#[command(
    name = "git-atomic",
    version,
    about,
    disable_help_subcommand = true,
    subcommand_negates_reqs = true
)]
pub struct Cli {
    /// Path to the .atomic.toml configuration file.
    #[arg(long, default_value = ".atomic.toml", global = true)]
    pub config: PathBuf,

    /// Increase log verbosity (-v for files, -vv for debug details).
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Suppress all non-error output.
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Emit machine-readable JSON instead of human-friendly text.
    #[arg(long, global = true)]
    pub json: bool,

    /// Show what would happen without performing any mutations.
    #[arg(long, global = true)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Split a commit into per-component branches [default when omitted].
    Commit(CommitArgs),

    /// Show each component's branch state relative to the base branch.
    Status(StatusArgs),

    /// Check the configuration file for errors (bad globs, missing fields).
    Validate,

    /// Generate a starter .atomic.toml in the current directory.
    Init,
}

#[derive(Debug, Parser)]
pub struct CommitArgs {
    /// Git ref or range to split (e.g. HEAD, main..feature).
    #[arg(default_value = "HEAD")]
    pub source_ref: String,

    /// Overwrite branches that have diverged from the base branch.
    #[arg(long)]
    pub force: bool,

    /// Split and push in one step; exit non-zero on any failure.
    #[arg(long)]
    pub ci_mode: bool,

    /// Push component branches to the remote after splitting.
    #[arg(long)]
    pub push: bool,

    /// Git remote to push to when --push or --ci-mode is used.
    #[arg(long, default_value = "origin")]
    pub remote: String,
}

#[derive(Debug, Parser)]
pub struct StatusArgs {
    /// Git ref whose changed files to inspect.
    #[arg(long = "ref", default_value = "HEAD")]
    pub source_ref: String,

    /// Suppress configuration display (show only branch state).
    #[arg(long)]
    pub no_config: bool,
}
