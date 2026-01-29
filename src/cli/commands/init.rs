use crate::cli::output::Printer;
use crate::config::Config;
use crate::core::Error;
use std::path::Path;

const HEADER: &str = "\
# git-atomic configuration
# See: https://github.com/aRustyDev/git-atomic
#
# Order matters: first matching component wins.

";

pub fn run(config_path: &Path, printer: &Printer) -> Result<(), Error> {
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
    std::fs::write(config_path, &content).map_err(|e| {
        Error::General(format!(
            "failed to write {}: {e}",
            config_path.display()
        ))
    })?;

    printer.print_init(config_path);
    Ok(())
}
