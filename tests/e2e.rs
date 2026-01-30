use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

mod support;

use support::{commit_file, init_test_repo, write_atomic_toml};

fn cmd(dir: &std::path::Path) -> Command {
    let mut c = Command::cargo_bin("git-atomic").unwrap();
    c.current_dir(dir);
    c
}

#[test]
fn commit_creates_branches() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);
    write_atomic_toml(dir, &[("app", &["src/**"])]);
    commit_file(dir, "src/main.rs", "fn main() {}", "feat: add main");

    cmd(dir).arg("commit").assert().success();

    let output = support::git(dir, &["branch"]);
    assert!(
        output.contains("atomic/app"),
        "expected atomic/app branch, got: {output}"
    );
}

#[test]
fn commit_range_mode() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);
    write_atomic_toml(dir, &[("app", &["src/**"])]);
    let base = support::git(dir, &["rev-parse", "HEAD"]);
    commit_file(dir, "src/lib.rs", "// lib", "feat: add lib");

    cmd(dir)
        .args(["commit", &format!("{base}..HEAD")])
        .assert()
        .success();
}

#[test]
fn dry_run_no_mutation() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);
    write_atomic_toml(dir, &[("app", &["src/**"])]);
    commit_file(dir, "src/lib.rs", "// lib", "feat: add lib");

    let branches_before = support::git(dir, &["branch"]);

    cmd(dir).args(["--dry-run", "commit"]).assert().success();

    let branches_after = support::git(dir, &["branch"]);
    assert_eq!(
        branches_before, branches_after,
        "dry-run should not create branches"
    );
}

#[test]
fn json_output_valid() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);
    write_atomic_toml(dir, &[("app", &["src/**"])]);
    commit_file(dir, "src/lib.rs", "// lib", "feat: add lib");

    let output = cmd(dir).args(["--json", "commit"]).output().unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        serde_json::from_str::<serde_json::Value>(&stdout).is_ok(),
        "stdout should be valid JSON, got: {stdout}"
    );
}

#[test]
fn init_creates_config() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);

    cmd(dir).arg("init").assert().success();

    assert!(
        dir.join(".atomic.toml").exists(),
        ".atomic.toml should be created"
    );
}

#[test]
fn init_dry_run_no_file() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);

    cmd(dir).args(["--dry-run", "init"]).assert().success();

    assert!(
        !dir.join(".atomic.toml").exists(),
        ".atomic.toml should not be created in dry-run"
    );
}

#[test]
fn validate_good_config() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);
    write_atomic_toml(dir, &[("app", &["src/**"]), ("docs", &["docs/**"])]);

    cmd(dir).arg("validate").assert().success();
}

#[test]
fn validate_bad_config() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);
    std::fs::write(dir.join(".atomic.toml"), "this is not valid toml [[[").unwrap();

    cmd(dir).arg("validate").assert().failure();
}

#[test]
fn status_output() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);
    write_atomic_toml(dir, &[("app", &["src/**"])]);
    commit_file(dir, "src/lib.rs", "// lib", "feat: add lib");

    cmd(dir)
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn default_no_subcommand() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);
    write_atomic_toml(dir, &[("app", &["src/**"])]);
    commit_file(dir, "src/lib.rs", "// lib", "feat: add lib");

    cmd(dir).assert().success();
}

#[test]
fn error_exit_codes() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);
    // No .atomic.toml present — should fail

    cmd(dir)
        .arg("commit")
        .assert()
        .failure()
        .stderr(predicate::str::is_empty().not());
}

#[test]
fn init_existing_config_fails() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    init_test_repo(dir);
    write_atomic_toml(dir, &[("app", &["src/**"])]);

    cmd(dir)
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::is_empty().not());
}
