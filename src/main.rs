use clap::Parser;
use git_atomic::cli::Cli;
use git_atomic::config;
use git_atomic::core::{ComponentMatcher, Error};
use git_atomic::git;
use std::process::ExitCode;

fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    let cfg = config::load_config(&cli.config)?;
    let matcher = ComponentMatcher::from_config(&cfg)?;

    let repo = git::open_repo(&std::env::current_dir().map_err(|e| Error::General(e.to_string()))?)?;
    let head = git::resolve_commit(&repo, "HEAD")?;
    let files = git::changed_files(&repo, head)?;

    let path_refs: Vec<&std::path::Path> = files.iter().map(|p| p.as_path()).collect();
    let (grouped, unmatched) = matcher.group_files(&path_refs);

    if !unmatched.is_empty() {
        match cfg.settings.unmatched_files {
            config::UnmatchedPolicy::Error => {
                return Err(Error::UnmatchedFiles {
                    paths: unmatched.iter().map(|p| p.to_path_buf()).collect(),
                });
            }
            config::UnmatchedPolicy::Warn => {
                eprintln!(
                    "warning: unmatched files: {}",
                    unmatched
                        .iter()
                        .map(|p| p.display().to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            config::UnmatchedPolicy::Ignore => {}
        }
    }

    if cli.json {
        let output = serde_json::json!({
            "components": grouped.iter().map(|(name, files)| {
                serde_json::json!({
                    "name": name,
                    "files": files.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
                })
            }).collect::<Vec<_>>(),
            "unmatched": unmatched.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else if !cli.quiet {
        for (name, files) in &grouped {
            println!("[{name}]");
            for f in files {
                println!("  {}", f.display());
            }
        }
    }

    Ok(())
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
