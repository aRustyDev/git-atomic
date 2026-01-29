use crate::cli::output::Printer;
use crate::config::layered;
use crate::core::Error;
use std::path::Path;

pub fn run(config_path: &Path, printer: &Printer) -> Result<(), Error> {
    // Try to open repo for git config layer (non-fatal if outside repo)
    let repo = crate::git::open_repo(
        &std::env::current_dir().map_err(|e| Error::General(e.to_string()))?,
    )
    .ok();

    match crate::config::load_layered_config(repo.as_ref(), config_path) {
        Ok(resolved) => {
            let warnings = layered::validate_resolved(&resolved);
            for w in &warnings {
                printer.print_config_warning(w);
            }
            printer.print_validate_ok();
            Ok(())
        }
        Err(e) => {
            let err = Error::Config(e);
            printer.print_validate_error(&err);
            Err(err)
        }
    }
}
