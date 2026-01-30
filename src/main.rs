use clap::Parser;
use git_atomic::cli::output::Printer;
use git_atomic::cli::{Cli, Command, CommitArgs};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let printer = Printer::new(cli.json, cli.quiet, cli.verbose);

    let result = match cli.command {
        Some(Command::Commit(ref args)) => {
            git_atomic::cli::commands::commit::run(args, &cli.config, cli.dry_run, &printer)
        }
        Some(Command::Status(ref args)) => {
            git_atomic::cli::commands::status::run(args, &cli.config, &printer)
        }
        Some(Command::Validate) => git_atomic::cli::commands::validate::run(&cli.config, &printer),
        Some(Command::Init) => {
            git_atomic::cli::commands::init::run(&cli.config, cli.dry_run, &printer)
        }
        None => {
            let args = CommitArgs {
                source_ref: "HEAD".into(),
                force: false,
                ci_mode: false,
                push: false,
                remote: "origin".into(),
            };
            git_atomic::cli::commands::commit::run(&args, &cli.config, cli.dry_run, &printer)
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            printer.print_error(&e);
            e.exit_code()
        }
    }
}
