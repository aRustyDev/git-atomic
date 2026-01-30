use crate::cli::output::Printer;
use crate::config::Config;
use crate::core::Error;
use crate::core::effect::{self, Effect};
use std::path::Path;

const HEADER: &str = "\
# git-atomic configuration
# See: https://github.com/aRustyDev/git-atomic
#
# Order matters: first matching component wins.

";

pub fn run(config_path: &Path, dry_run: bool, printer: &Printer) -> Result<(), Error> {
    if config_path.exists() {
        return Err(Error::General(format!(
            "config already exists: {}",
            config_path.display()
        )));
    }

    let sample = Config::sample();
    let toml_body = toml::to_string_pretty(&sample)
        .map_err(|e| Error::General(format!("failed to serialize config: {e}")))?;

    let content = format!("{HEADER}{toml_body}");
    let structured = serde_json::to_value(&sample).ok();
    let effects = vec![Effect::WriteFile {
        path: config_path.to_path_buf(),
        content,
        structured,
    }];

    effect::execute(None, &effects, dry_run, printer)?;

    if !dry_run {
        printer.print_init(config_path);
    }

    Ok(())
}
