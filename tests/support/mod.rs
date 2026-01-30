use std::path::Path;
use std::process::Command;

/// Run a git command in `dir`, return trimmed stdout.
pub fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// git init + config user + initial empty commit. Returns gix::Repository.
pub fn init_test_repo(dir: &Path) -> gix::Repository {
    git(dir, &["init", "-b", "main"]);
    git(dir, &["config", "user.email", "test@test.com"]);
    git(dir, &["config", "user.name", "Test"]);
    git(dir, &["commit", "--allow-empty", "-m", "initial"]);
    git_atomic::git::open_repo(dir).unwrap()
}

/// Write file, git add, git commit.
pub fn commit_file(dir: &Path, path: &str, content: &str, message: &str) {
    let full = dir.join(path);
    if let Some(parent) = full.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(&full, content).unwrap();
    git(dir, &["add", path]);
    git(dir, &["commit", "-m", message]);
}

/// Build a Config with given component specs: vec of (name, globs).
pub fn test_config(components: &[(&str, &[&str])]) -> git_atomic::config::Config {
    git_atomic::config::Config {
        settings: git_atomic::config::Settings {
            unmatched_files: git_atomic::config::UnmatchedPolicy::Ignore,
            ..Default::default()
        },
        components: components
            .iter()
            .map(|(name, globs)| git_atomic::config::Component {
                name: name.to_string(),
                globs: globs.iter().map(|g| g.to_string()).collect(),
                commit_type: None,
                branch: None,
            })
            .collect(),
    }
}

/// Write a .atomic.toml config file from component specs.
pub fn write_atomic_toml(dir: &Path, components: &[(&str, &[&str])]) {
    let mut content =
        String::from("[settings]\nbase_branch = \"main\"\nunmatched_files = \"ignore\"\n\n");
    for (name, globs) in components {
        content.push_str(&format!("[[components]]\nname = \"{name}\"\nglobs = ["));
        let glob_strs: Vec<String> = globs.iter().map(|g| format!("\"{g}\"")).collect();
        content.push_str(&glob_strs.join(", "));
        content.push_str("]\n\n");
    }
    std::fs::write(dir.join(".atomic.toml"), content).unwrap();
}
