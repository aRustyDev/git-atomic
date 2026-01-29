use crate::cli::output::Printer;
use crate::cli::CommitArgs;
use crate::core::effect::{self, Effect};
use crate::core::{ComponentMatcher, Error};
use std::path::Path;

pub fn run(
    args: &CommitArgs,
    config_path: &Path,
    dry_run: bool,
    printer: &Printer,
) -> Result<(), Error> {
    let repo =
        crate::git::open_repo(&std::env::current_dir().map_err(|e| Error::General(e.to_string()))?)?;

    let resolved = crate::config::load_layered_config(Some(&repo), config_path)?;

    if resolved.components.is_empty() {
        return Err(Error::General(
            "No components defined. Create .atomic.toml with [[components]] or run git-atomic init."
                .into(),
        ));
    }

    let cfg = resolved.to_config();
    let matcher = ComponentMatcher::from_config(&cfg)?;

    let (results, mut effects) =
        crate::git::atomize::plan_atomize(&repo, &cfg, &matcher, &args.source_ref, args.force)?;

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
