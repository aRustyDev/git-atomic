use crate::cli::output::Printer;
use crate::cli::AtomizeArgs;
use crate::core::{ComponentMatcher, Error};
use std::path::Path;

pub fn run(args: &AtomizeArgs, config_path: &Path, printer: &Printer) -> Result<(), Error> {
    let cfg = crate::config::load_config(config_path)?;
    let matcher = ComponentMatcher::from_config(&cfg)?;
    let repo =
        crate::git::open_repo(&std::env::current_dir().map_err(|e| Error::General(e.to_string()))?)?;

    let results =
        crate::git::atomize::atomize(&repo, &cfg, &matcher, &args.commit, args.force, args.dry_run)?;

    printer.print_atomize_results(&results, args.dry_run);

    if !args.dry_run && (args.push || args.ci_mode) && !results.is_empty() {
        let branches: Vec<&str> = results.iter().map(|r| r.branch.as_str()).collect();
        push_branches(&args.remote, &branches)?;
    }

    Ok(())
}

fn push_branches(remote: &str, branches: &[&str]) -> Result<(), Error> {
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
    Ok(())
}
