use crate::cli::output::Printer;
use crate::core::Error;
use std::path::Path;

const DEFAULT_CONFIG: &str = r#"# git-atomic configuration
# See: https://github.com/aRustyDev/git-atomic

[settings]
base_branch = "main"
branch_template = "atomic/{component}"
unmatched_files = "error"
# default_commit_type = "feat"

# Define components with glob patterns.
# Order matters: first matching component wins.
#
# [components.frontend]
# globs = ["src/ui/**", "src/components/**"]
#
# [components.backend]
# globs = ["src/api/**", "src/db/**"]
# commit_type = "fix"
# branch = "custom/branch-name"
"#;

pub fn run(config_path: &Path, printer: &Printer) -> Result<(), Error> {
    if config_path.exists() {
        let msg = format!("config already exists: {}", config_path.display());
        return Err(Error::General(msg));
    }

    std::fs::write(config_path, DEFAULT_CONFIG).map_err(|e| {
        Error::General(format!(
            "failed to write {}: {e}",
            config_path.display()
        ))
    })?;

    printer.print_init(config_path);
    Ok(())
}
