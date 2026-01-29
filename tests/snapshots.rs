mod support;

use std::process::Command;
use support::{commit_file, init_test_repo, write_atomic_toml};
use tempfile::TempDir;

fn git_atomic_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_git-atomic"))
}

fn redacted_settings() -> insta::Settings {
    let mut settings = insta::Settings::clone_current();
    settings.add_filter(r"[0-9a-f]{8,40}", "[COMMIT_HASH]");
    settings.add_filter(r"/tmp/[^\s]+", "[TMPDIR]");
    settings.add_filter(r"/var/folders[^\s]+", "[TMPDIR]");
    settings.add_filter(r"/private/tmp[^\s]+", "[TMPDIR]");
    settings
}

fn setup_repo_with_components() -> TempDir {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);
    write_atomic_toml(
        dir,
        &[
            ("frontend", &["src/ui/**"][..]),
            ("backend", &["src/api/**"][..]),
        ],
    );
    commit_file(dir, "src/ui/app.tsx", "export default function App() {}", "feat: add frontend");
    commit_file(dir, "src/api/handler.rs", "pub fn handle() {}", "feat: add backend");
    tmp
}

#[test]
fn snapshot_human_commit_output() {
    let tmp = setup_repo_with_components();
    let output = git_atomic_bin()
        .arg("commit")
        .current_dir(tmp.path())
        .output()
        .expect("failed to execute git-atomic commit");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let settings = redacted_settings();
    settings.bind(|| {
        insta::assert_snapshot!(stdout);
    });
}

#[test]
fn snapshot_json_commit_output() {
    let tmp = setup_repo_with_components();
    let output = git_atomic_bin()
        .args(["--json", "commit"])
        .current_dir(tmp.path())
        .output()
        .expect("failed to execute git-atomic --json commit");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let settings = redacted_settings();
    settings.bind(|| {
        insta::assert_snapshot!(stdout);
    });
}

#[test]
fn snapshot_dry_run_output() {
    let tmp = setup_repo_with_components();
    let output = git_atomic_bin()
        .args(["--dry-run", "commit"])
        .current_dir(tmp.path())
        .output()
        .expect("failed to execute git-atomic --dry-run commit");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let settings = redacted_settings();
    settings.bind(|| {
        insta::assert_snapshot!(stdout);
    });
}

#[test]
fn snapshot_status_output() {
    let tmp = setup_repo_with_components();
    let output = git_atomic_bin()
        .arg("status")
        .current_dir(tmp.path())
        .output()
        .expect("failed to execute git-atomic status");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let settings = redacted_settings();
    settings.bind(|| {
        insta::assert_snapshot!(stdout);
    });
}

#[test]
fn snapshot_validate_ok() {
    let tmp = setup_repo_with_components();
    let output = git_atomic_bin()
        .arg("validate")
        .current_dir(tmp.path())
        .output()
        .expect("failed to execute git-atomic validate");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let settings = redacted_settings();
    settings.bind(|| {
        insta::assert_snapshot!(stdout);
    });
}

#[test]
fn snapshot_init_dry_run() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);
    let output = git_atomic_bin()
        .args(["--dry-run", "init"])
        .current_dir(dir)
        .output()
        .expect("failed to execute git-atomic --dry-run init");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let settings = redacted_settings();
    settings.bind(|| {
        insta::assert_snapshot!(stdout);
    });
}

#[test]
fn snapshot_error_no_config() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);
    let output = git_atomic_bin()
        .arg("validate")
        .current_dir(dir)
        .output()
        .expect("failed to execute git-atomic validate");

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let settings = redacted_settings();
    settings.bind(|| {
        insta::assert_snapshot!(stderr);
    });
}

#[test]
fn snapshot_json_status() {
    let tmp = setup_repo_with_components();
    let output = git_atomic_bin()
        .args(["--json", "status"])
        .current_dir(tmp.path())
        .output()
        .expect("failed to execute git-atomic --json status");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let settings = redacted_settings();
    settings.bind(|| {
        insta::assert_snapshot!(stdout);
    });
}
