use crate::cli::CommitArgs;
use crate::cli::output::Printer;
use crate::core::effect::{self, Effect};
use crate::core::refspec::RefSpec;
use crate::core::{ComponentMatcher, Error, GitError};
use std::path::Path;

pub fn run(
    args: &CommitArgs,
    config_path: &Path,
    dry_run: bool,
    printer: &Printer,
) -> Result<(), Error> {
    let repo = crate::git::open_repo(
        &std::env::current_dir().map_err(|e| Error::General(e.to_string()))?,
    )?;

    let resolved = crate::config::load_layered_config(Some(&repo), config_path)?;

    if resolved.components.is_empty() {
        return Err(Error::General(
            "No components defined. Create .atomic.toml with [[components]] or run git-atomic init."
                .into(),
        ));
    }

    let cfg = resolved.to_config();
    let matcher = ComponentMatcher::from_config(&cfg)?;

    let refspec = RefSpec::parse(&args.source_ref).map_err(|e| Error::General(e))?;

    let (results, mut effects) = match refspec {
        RefSpec::Single(ref_str) => {
            let source_id = crate::git::resolve_commit(&repo, &ref_str)?;
            crate::git::atomize::plan_atomize(&repo, &cfg, &matcher, source_id, args.force)?
        }
        RefSpec::Range { start, end } => {
            let start_id = crate::git::resolve_commit(&repo, &start).map_err(|e| {
                GitError::Operation(format!(
                    "could not resolve '{}' (left side of range '{}..{}'): {}",
                    start, start, end, e
                ))
            })?;
            let end_id = crate::git::resolve_commit(&repo, &end).map_err(|e| {
                GitError::Operation(format!(
                    "could not resolve '{}' (right side of range '{}..{}'): {}",
                    end, start, end, e
                ))
            })?;
            let commits = crate::git::walk::walk_range(&repo, start_id, end_id)?;
            let effective = crate::git::walk::effective_files(&repo, start_id, end_id)?;
            crate::git::atomize::plan_atomize_range(
                &repo, &cfg, &matcher, &commits, &effective, args.force,
            )?
        }
    };

    if (args.push || args.ci_mode) && !results.is_empty() {
        let branches: Vec<String> = results.iter().map(|r| r.branch.clone()).collect();
        effects.push(Effect::Push {
            remote: args.remote.clone(),
            branches,
        });
    }

    effect::execute(Some(&repo), &effects, dry_run, printer)?;
    printer.print_commit_results(&results, dry_run);

    Ok(())
}
