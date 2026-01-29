use crate::cli::output::Printer;
use crate::core::Error;
use std::path::Path;

pub fn run(config_path: &Path, printer: &Printer) -> Result<(), Error> {
    match crate::config::load_config(config_path) {
        Ok(_cfg) => {
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
