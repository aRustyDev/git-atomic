use crate::cli::output::Printer;
use crate::core::{Error, GitError};
use std::path::PathBuf;

/// A planned reference edit for atomic batch updates.
#[derive(Debug)]
pub struct PlannedRefEdit {
    pub ref_name: String,
    pub new_id: gix::ObjectId,
    pub previous: Option<gix::ObjectId>,
    pub component: String,
    pub created: bool,
}

/// A side effect that a command wants to perform.
#[derive(Debug)]
pub enum Effect {
    /// Atomic batch ref update (preserves all-or-nothing semantics).
    RefTransaction {
        repo_path: PathBuf,
        edits: Vec<PlannedRefEdit>,
    },
    /// Push branches to a remote via `git push`.
    Push {
        remote: String,
        branches: Vec<String>,
    },
    /// Write a file to disk.
    WriteFile { path: PathBuf, content: String },
}

/// Execute or preview a list of effects.
pub fn execute(
    repo: Option<&gix::Repository>,
    effects: &[Effect],
    dry_run: bool,
    printer: &Printer,
) -> Result<(), Error> {
    for effect in effects {
        if dry_run {
            printer.print_effect_preview(effect);
        } else {
            run_effect(repo, effect)?;
        }
    }
    Ok(())
}

fn run_effect(repo: Option<&gix::Repository>, effect: &Effect) -> Result<(), Error> {
    match effect {
        Effect::RefTransaction { edits, .. } => {
            let repo = repo.ok_or_else(|| {
                Error::General("RefTransaction requires a repository".into())
            })?;

            let mut gix_edits: Vec<gix::refs::transaction::RefEdit> = Vec::new();
            for e in edits {
                let target = gix::refs::Target::Object(e.new_id);
                let expected = match e.previous {
                    Some(id) => gix::refs::transaction::PreviousValue::MustExistAndMatch(
                        gix::refs::Target::Object(id),
                    ),
                    None => gix::refs::transaction::PreviousValue::MustNotExist,
                };

                gix_edits.push(gix::refs::transaction::RefEdit {
                    change: gix::refs::transaction::Change::Update {
                        log: gix::refs::transaction::LogChange {
                            mode: gix::refs::transaction::RefLog::AndReference,
                            force_create_reflog: false,
                            message: "git-atomic: atomize".into(),
                        },
                        expected,
                        new: target,
                    },
                    name: gix::refs::FullName::try_from(e.ref_name.clone()).map_err(|err| {
                        GitError::RefUpdate {
                            branch: e.ref_name.clone(),
                            reason: format!("invalid ref name: {err}"),
                        }
                    })?,
                    deref: false,
                });
            }

            if !gix_edits.is_empty() {
                repo.edit_references(gix_edits)
                    .map_err(|e| GitError::RefUpdate {
                        branch: "batch update".into(),
                        reason: e.to_string(),
                    })?;
            }
        }
        Effect::Push { remote, branches } => {
            let mut cmd = std::process::Command::new("git");
            cmd.arg("push").arg(remote);
            for b in branches {
                cmd.arg(b);
            }
            let status = cmd
                .status()
                .map_err(|e| Error::General(format!("failed to run git push: {e}")))?;
            if !status.success() {
                return Err(Error::General(format!(
                    "git push exited with status {}",
                    status
                )));
            }
        }
        Effect::WriteFile { path, content } => {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    Error::General(format!("failed to create directory {}: {e}", parent.display()))
                })?;
            }
            std::fs::write(path, content).map_err(|e| {
                Error::General(format!("failed to write {}: {e}", path.display()))
            })?;
        }
    }
    Ok(())
}
