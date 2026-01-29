use crate::git::atomize::AtomicResult;
use crate::git::branch::BranchState;
use owo_colors::OwoColorize;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    Human,
    Json,
    Quiet,
}

pub struct Printer {
    pub mode: OutputMode,
    pub verbosity: u8,
}

impl Printer {
    pub fn new(json: bool, quiet: bool, verbosity: u8) -> Self {
        let mode = if json {
            OutputMode::Json
        } else if quiet {
            OutputMode::Quiet
        } else {
            OutputMode::Human
        };
        Self { mode, verbosity }
    }

    pub fn print_atomize_results(&self, results: &[AtomicResult], dry_run: bool) {
        match self.mode {
            OutputMode::Quiet => {}
            OutputMode::Json => {
                let output = serde_json::json!({
                    "dry_run": dry_run,
                    "results": results.iter().map(|r| {
                        serde_json::json!({
                            "component": r.component,
                            "branch": r.branch,
                            "commit": r.commit_id.to_string(),
                            "files": r.files.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
                            "created": r.created,
                        })
                    }).collect::<Vec<_>>(),
                });
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
            }
            OutputMode::Human => {
                let mut out = io::stdout().lock();
                for r in results {
                    let prefix = if dry_run { "would " } else { "" };
                    let action = if r.created { "create" } else { "update" };
                    let short_id = r.commit_id.to_string();
                    let short_id = short_id.get(..8).unwrap_or(&short_id);

                    let _ = writeln!(
                        out,
                        "{} [{}] {} → {} ({} file{})",
                        "✓".green(),
                        r.component.cyan(),
                        short_id.dimmed(),
                        r.branch.bold(),
                        format!("{prefix}{action}, {}", r.files.len()),
                        if r.files.len() == 1 { "" } else { "s" }
                    );

                    if self.verbosity > 0 {
                        for f in &r.files {
                            let _ = writeln!(out, "    {}", f.display().dimmed());
                        }
                    }
                }
            }
        }
    }

    pub fn print_status(
        &self,
        components: &[(String, Vec<std::path::PathBuf>, BranchState, String)],
    ) {
        match self.mode {
            OutputMode::Quiet => {}
            OutputMode::Json => {
                let output: Vec<_> = components
                    .iter()
                    .map(|(name, files, state, branch)| {
                        serde_json::json!({
                            "component": name,
                            "branch": branch,
                            "state": format!("{state:?}"),
                            "file_count": files.len(),
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
            }
            OutputMode::Human => {
                let mut out = io::stdout().lock();
                for (name, files, state, branch) in components {
                    let state_str = match state {
                        BranchState::Missing => "missing".yellow().to_string(),
                        BranchState::Current => "current".green().to_string(),
                        BranchState::FastForward { .. } => "ahead".cyan().to_string(),
                        BranchState::Diverged { .. } => "diverged".red().to_string(),
                    };
                    let _ = writeln!(
                        out,
                        "  {} {} ({}, {} file{})",
                        name.bold(),
                        branch.dimmed(),
                        state_str,
                        files.len(),
                        if files.len() == 1 { "" } else { "s" }
                    );
                }
            }
        }
    }

    pub fn print_validate_ok(&self) {
        match self.mode {
            OutputMode::Quiet => {}
            OutputMode::Json => println!(r#"{{"valid": true}}"#),
            OutputMode::Human => println!("{} configuration is valid", "✓".green()),
        }
    }

    pub fn print_init(&self, path: &std::path::Path) {
        match self.mode {
            OutputMode::Quiet => {}
            OutputMode::Json => {
                println!(
                    "{}",
                    serde_json::json!({"created": path.display().to_string()})
                );
            }
            OutputMode::Human => {
                println!("{} created {}", "✓".green(), path.display());
            }
        }
    }

    pub fn print_validate_error(&self, err: &crate::core::Error) {
        match self.mode {
            OutputMode::Quiet => {}
            OutputMode::Json => {
                println!(
                    "{}",
                    serde_json::json!({"valid": false, "error": err.to_string()})
                );
            }
            OutputMode::Human => {
                eprintln!("{} {}", "✗".red(), err);
            }
        }
    }
}
