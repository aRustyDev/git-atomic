use clap::Parser;
use git_atomic::cli::output::Printer;
use git_atomic::cli::{AtomizeArgs, Cli, Command};
use std::process::ExitCode;

fn run() -> Result<(), git_atomic::core::Error> {
    let cli = Cli::parse();
    let printer = Printer::new(cli.json, cli.quiet, cli.verbose);

    match cli.command {
        Some(Command::Atomize(ref args)) => {
            git_atomic::cli::commands::atomize::run(args, &cli.config, cli.dry_run, &printer)
        }
        Some(Command::Status(ref args)) => {
            git_atomic::cli::commands::status::run(args, &cli.config, &printer)
        }
        Some(Command::Validate) => {
            git_atomic::cli::commands::validate::run(&cli.config, &printer)
        }
        Some(Command::Init) => {
            git_atomic::cli::commands::init::run(&cli.config, cli.dry_run, &printer)
        }
        None => {
            let args = AtomizeArgs {
                commit: "HEAD".into(),
                range: None,
                force: false,
                ci_mode: false,
                push: false,
                remote: "origin".into(),
            };
            git_atomic::cli::commands::atomize::run(&args, &cli.config, cli.dry_run, &printer)
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            e.exit_code()
        }
    }
}
