use crate::cli::StatusArgs;
use crate::cli::output::Printer;
use crate::core::{ComponentMatcher, Error};
use crate::git::branch::BranchManager;
use std::path::{Path, PathBuf};

pub fn run(args: &StatusArgs, config_path: &Path, printer: &Printer) -> Result<(), Error> {
    let repo = crate::git::open_repo(
        &std::env::current_dir().map_err(|e| Error::General(e.to_string()))?,
    )?;

    let resolved = crate::config::load_layered_config(Some(&repo), config_path)?;

    // Show config provenance unless suppressed
    if !args.no_config {
        printer.print_config_provenance(&resolved);
    }

    // Show warnings (including "no components defined")
    let warnings = crate::config::layered::validate_resolved(&resolved);
    for w in &warnings {
        printer.print_config_warning(w);
    }

    if resolved.components.is_empty() {
        return Ok(());
    }

    let cfg = resolved.to_config();
    let matcher = ComponentMatcher::from_config(&cfg)?;

    let source_id = crate::git::resolve_commit(&repo, &args.source_ref)?;
    let files = crate::git::changed_files(&repo, source_id)?;
    let path_refs: Vec<&Path> = files.iter().map(|p| p.as_path()).collect();
    let (grouped, _) = matcher.group_files(&path_refs);

    let base_id = crate::git::resolve_commit(&repo, &cfg.settings.base_branch)?;
    let branch_mgr = BranchManager::new(&repo, base_id, cfg.settings.branch_template.clone());

    let mut components = Vec::new();
    for (name, comp_files) in &grouped {
        let comp_cfg = cfg.components.iter().find(|c| c.name == *name);
        let branch_override = comp_cfg.and_then(|c| c.branch.as_deref());
        let ref_name = branch_mgr.branch_ref_name(name, branch_override);
        let state = branch_mgr.check_state(&ref_name)?;
        let branch_display = ref_name
            .strip_prefix("refs/heads/")
            .unwrap_or(&ref_name)
            .to_string();

        components.push((
            name.to_string(),
            comp_files
                .iter()
                .map(|p| p.to_path_buf())
                .collect::<Vec<PathBuf>>(),
            state,
            branch_display,
        ));
    }

    printer.print_status(&components);
    Ok(())
}
