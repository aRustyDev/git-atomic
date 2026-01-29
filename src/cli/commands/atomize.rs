use crate::cli::output::Printer;
use crate::cli::AtomizeArgs;
use crate::core::effect::{self, Effect};
use crate::core::{ComponentMatcher, Error};
use std::path::Path;

pub fn run(
    args: &AtomizeArgs,
    config_path: &Path,
    dry_run: bool,
    printer: &Printer,
) -> Result<(), Error> {
    let cfg = crate::config::load_config(config_path)?;
    let matcher = ComponentMatcher::from_config(&cfg)?;
    let repo =
        crate::git::open_repo(&std::env::current_dir().map_err(|e| Error::General(e.to_string()))?)?;

    let (results, mut effects) =
        crate::git::atomize::plan_atomize(&repo, &cfg, &matcher, &args.commit, args.force)?;

    if (args.push || args.ci_mode) && !results.is_empty() {
        let branches: Vec<String> = results.iter().map(|r| r.branch.clone()).collect();
        effects.push(Effect::Push {
            remote: args.remote.clone(),
            branches,
        });
    }

    effect::execute(Some(&repo), &effects, dry_run, printer)?;
    printer.print_atomize_results(&results, dry_run);

    Ok(())
}
