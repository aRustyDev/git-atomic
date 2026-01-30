use crate::config::ResolvedConfig;
use crate::config::layered::ConfigWarning;
use crate::core::effect::Effect;
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

    pub fn print_commit_results(&self, results: &[AtomicResult], dry_run: bool) {
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

    pub fn print_error(&self, err: &crate::core::Error) {
        match self.mode {
            OutputMode::Quiet => {}
            OutputMode::Json => {
                println!("{}", serde_json::json!({"error": err.to_string()}));
            }
            OutputMode::Human => {
                eprintln!("{} {}", "✗".red(), err);
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

    pub fn print_config_provenance(&self, resolved: &ResolvedConfig) {
        match self.mode {
            OutputMode::Quiet => {}
            OutputMode::Json => {
                let config = serde_json::json!({
                    "config": {
                        "base_branch": {
                            "value": resolved.base_branch.value,
                            "source": resolved.base_branch.source.label(),
                        },
                        "branch_template": {
                            "value": resolved.branch_template.value,
                            "source": resolved.branch_template.source.label(),
                        },
                        "unmatched_files": {
                            "value": resolved.unmatched_files.value.to_string(),
                            "source": resolved.unmatched_files.source.label(),
                        },
                        "default_commit_type": {
                            "value": resolved.default_commit_type.value,
                            "source": resolved.default_commit_type.source.label(),
                        },
                    }
                });
                println!("{}", serde_json::to_string_pretty(&config).unwrap());
            }
            OutputMode::Human => {
                let mut out = io::stdout().lock();
                let _ = writeln!(out, "{}", "Settings:".bold());
                let _ = writeln!(
                    out,
                    "  {:<20} = {:<30} ({})",
                    "base_branch",
                    resolved.base_branch.value,
                    resolved.base_branch.source.label().dimmed()
                );
                let _ = writeln!(
                    out,
                    "  {:<20} = {:<30} ({})",
                    "branch_template",
                    resolved.branch_template.value,
                    resolved.branch_template.source.label().dimmed()
                );
                let _ = writeln!(
                    out,
                    "  {:<20} = {:<30} ({})",
                    "unmatched_files",
                    resolved.unmatched_files.value.to_string(),
                    resolved.unmatched_files.source.label().dimmed()
                );
                if let Some(ref ct) = resolved.default_commit_type.value {
                    let _ = writeln!(
                        out,
                        "  {:<20} = {:<30} ({})",
                        "default_commit_type",
                        ct,
                        resolved.default_commit_type.source.label().dimmed()
                    );
                }
                let _ = writeln!(out);
            }
        }
    }

    pub fn print_config_warning(&self, warning: &ConfigWarning) {
        match self.mode {
            OutputMode::Quiet => {}
            OutputMode::Json => {
                println!("{}", serde_json::json!({"warning": warning.message}));
            }
            OutputMode::Human => {
                eprintln!("{} {}", "⚠".yellow(), warning.message);
            }
        }
    }

    pub fn print_effect_preview(&self, effect: &Effect) {
        match self.mode {
            OutputMode::Quiet => {}
            OutputMode::Json => {
                let desc = match effect {
                    Effect::RefTransaction { edits, .. } => {
                        serde_json::json!({
                            "effect": "ref_transaction",
                            "dry_run": true,
                            "refs": edits.iter().map(|e| {
                                serde_json::json!({
                                    "ref": e.ref_name,
                                    "component": e.component,
                                    "action": if e.created { "create" } else { "update" },
                                })
                            }).collect::<Vec<_>>(),
                        })
                    }
                    Effect::Push { remote, branches } => {
                        serde_json::json!({
                            "effect": "push",
                            "dry_run": true,
                            "remote": remote,
                            "branches": branches,
                        })
                    }
                    Effect::WriteFile {
                        path,
                        content,
                        structured,
                    } => {
                        let content_value = structured
                            .clone()
                            .unwrap_or_else(|| serde_json::Value::String(content.clone()));
                        serde_json::json!({
                            "effect": "write_file",
                            "dry_run": true,
                            "path": path.display().to_string(),
                            "content": content_value,
                        })
                    }
                };
                println!("{}", serde_json::to_string_pretty(&desc).unwrap());
            }
            OutputMode::Human => {
                let mut out = io::stdout().lock();
                match effect {
                    Effect::RefTransaction { edits, .. } => {
                        for e in edits {
                            let action = if e.created { "create" } else { "update" };
                            let branch = e
                                .ref_name
                                .strip_prefix("refs/heads/")
                                .unwrap_or(&e.ref_name);
                            let _ = writeln!(
                                out,
                                "  {} would {} branch {}",
                                "▸".dimmed(),
                                action,
                                branch.bold()
                            );
                        }
                    }
                    Effect::Push { remote, branches } => {
                        let _ = writeln!(
                            out,
                            "  {} would push {} branch{} to {}",
                            "▸".dimmed(),
                            branches.len(),
                            if branches.len() == 1 { "" } else { "es" },
                            remote.bold()
                        );
                    }
                    Effect::WriteFile { path, content, .. } => {
                        let _ = writeln!(
                            out,
                            "  {} would create {}",
                            "▸".dimmed(),
                            path.display().bold()
                        );
                        if self.verbosity > 0 || content.len() < 4096 {
                            let _ = writeln!(out);
                            for line in content.lines() {
                                let _ = writeln!(out, "    {}", line.dimmed());
                            }
                        }
                    }
                }
            }
        }
    }
}
