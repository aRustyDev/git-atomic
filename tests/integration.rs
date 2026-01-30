mod support;

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use git_atomic::cli::output::Printer;
use git_atomic::config::{Config, Settings, UnmatchedPolicy, load_config};
use git_atomic::core::ComponentMatcher;
use git_atomic::core::effect::{self, Effect};
use git_atomic::git;
use git_atomic::git::atomize::{plan_atomize, plan_atomize_range};
use git_atomic::git::walk;

use support::{commit_file, git, init_test_repo, test_config, write_atomic_toml};

// ---------------------------------------------------------------------------
// 1. multi_component_split_single_commit
// ---------------------------------------------------------------------------
#[test]
fn multi_component_split_single_commit() {
    let dir = tempfile::tempdir().unwrap();
    let repo = init_test_repo(dir.path());

    write_atomic_toml(
        dir.path(),
        &[("frontend", &["src/ui/**"]), ("backend", &["src/api/**"])],
    );

    // Create a single commit touching both components.
    std::fs::create_dir_all(dir.path().join("src/ui")).unwrap();
    std::fs::create_dir_all(dir.path().join("src/api")).unwrap();
    std::fs::write(dir.path().join("src/ui/app.ts"), "ui").unwrap();
    std::fs::write(dir.path().join("src/api/handler.rs"), "api").unwrap();
    git(dir.path(), &["add", "."]);
    git(dir.path(), &["commit", "-m", "add both components"]);

    let config = test_config(&[("frontend", &["src/ui/**"]), ("backend", &["src/api/**"])]);
    let matcher = ComponentMatcher::from_config(&config).unwrap();
    let head = git::resolve_commit(&repo, "HEAD").unwrap();

    let (results, effects) = plan_atomize(&repo, &config, &matcher, head, false).unwrap();

    assert_eq!(results.len(), 2, "expected 2 component results");
    assert_eq!(effects.len(), 1, "expected 1 RefTransaction effect");

    let names: Vec<&str> = results.iter().map(|r| r.component.as_str()).collect();
    assert!(names.contains(&"frontend"));
    assert!(names.contains(&"backend"));
}

// ---------------------------------------------------------------------------
// 2. range_mode_partial_squash
// ---------------------------------------------------------------------------
#[test]
fn range_mode_partial_squash() {
    let dir = tempfile::tempdir().unwrap();
    let _repo = init_test_repo(dir.path());
    let base = git(dir.path(), &["rev-parse", "HEAD"]);

    // c1: add foo.ts and bar.ts
    std::fs::create_dir_all(dir.path().join("src/ui")).unwrap();
    std::fs::write(dir.path().join("src/ui/foo.ts"), "foo").unwrap();
    std::fs::write(dir.path().join("src/ui/bar.ts"), "bar").unwrap();
    git(dir.path(), &["add", "."]);
    git(dir.path(), &["commit", "-m", "add foo and bar"]);

    // c2: modify foo.ts
    std::fs::write(dir.path().join("src/ui/foo.ts"), "foo v2").unwrap();
    git(dir.path(), &["add", "."]);
    git(dir.path(), &["commit", "-m", "update foo"]);

    // c3: delete foo.ts (net-zero for foo.ts)
    std::fs::remove_file(dir.path().join("src/ui/foo.ts")).unwrap();
    git(dir.path(), &["add", "."]);
    git(dir.path(), &["commit", "-m", "remove foo"]);

    let repo = git::open_repo(dir.path()).unwrap();
    let start_id = git::resolve_commit(&repo, &base).unwrap();
    let end_id = git::resolve_commit(&repo, "HEAD").unwrap();

    let commits = walk::walk_range(&repo, start_id, end_id).unwrap();
    let effective = walk::effective_files(&repo, start_id, end_id).unwrap();

    // foo.ts is net-zero; bar.ts survives
    assert!(effective.contains(&PathBuf::from("src/ui/bar.ts")));
    assert!(!effective.contains(&PathBuf::from("src/ui/foo.ts")));

    let config = test_config(&[("frontend", &["src/ui/**"])]);
    let matcher = ComponentMatcher::from_config(&config).unwrap();

    let (results, _effects) =
        plan_atomize_range(&repo, &config, &matcher, &commits, &effective, false).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].component, "frontend");
    assert!(results[0].files.contains(&PathBuf::from("src/ui/bar.ts")));
    assert!(!results[0].files.contains(&PathBuf::from("src/ui/foo.ts")));
}

// ---------------------------------------------------------------------------
// 3. range_mode_incremental_trees
// ---------------------------------------------------------------------------
#[test]
fn range_mode_incremental_trees() {
    let dir = tempfile::tempdir().unwrap();
    let _repo = init_test_repo(dir.path());
    let base = git(dir.path(), &["rev-parse", "HEAD"]);

    // 3 commits adding files progressively
    commit_file(dir.path(), "src/ui/a.ts", "a", "add a");
    commit_file(dir.path(), "src/ui/b.ts", "b", "add b");
    commit_file(dir.path(), "src/ui/c.ts", "c", "add c");

    let repo = git::open_repo(dir.path()).unwrap();
    let start_id = git::resolve_commit(&repo, &base).unwrap();
    let end_id = git::resolve_commit(&repo, "HEAD").unwrap();

    let commits = walk::walk_range(&repo, start_id, end_id).unwrap();
    let effective = walk::effective_files(&repo, start_id, end_id).unwrap();

    assert_eq!(commits.len(), 3);

    let config = test_config(&[("frontend", &["src/ui/**"])]);
    let matcher = ComponentMatcher::from_config(&config).unwrap();

    let (results, effects) =
        plan_atomize_range(&repo, &config, &matcher, &commits, &effective, false).unwrap();

    let printer = Printer::new(false, true, 0);
    effect::execute(Some(&repo), &effects, false, &printer).unwrap();

    assert_eq!(results.len(), 1);

    // Final tree should contain all three files
    let final_commit = repo.find_commit(results[0].commit_id).unwrap();
    let tree = final_commit.tree().unwrap();
    assert!(tree.lookup_entry_by_path("src/ui/a.ts").unwrap().is_some());
    assert!(tree.lookup_entry_by_path("src/ui/b.ts").unwrap().is_some());
    assert!(tree.lookup_entry_by_path("src/ui/c.ts").unwrap().is_some());
}

// ---------------------------------------------------------------------------
// 4. range_mode_empty_range
// ---------------------------------------------------------------------------
#[test]
fn range_mode_empty_range() {
    let dir = tempfile::tempdir().unwrap();
    let _repo = init_test_repo(dir.path());

    let config = test_config(&[("app", &["src/**"])]);
    let matcher = ComponentMatcher::from_config(&config).unwrap();
    let repo = git::open_repo(dir.path()).unwrap();

    let (results, effects) =
        plan_atomize_range(&repo, &config, &matcher, &[], &HashSet::new(), false).unwrap();

    assert!(results.is_empty());
    assert!(effects.is_empty());
}

// ---------------------------------------------------------------------------
// 5. init_creates_config
// ---------------------------------------------------------------------------
#[test]
fn init_creates_config() {
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join(".atomic.toml");

    let sample = Config::sample();
    let content = toml::to_string_pretty(&sample).unwrap();

    let effect = Effect::WriteFile {
        path: config_path.clone(),
        content,
        structured: None,
    };

    let printer = Printer::new(false, true, 0);
    effect::execute(None, &[effect], false, &printer).unwrap();

    assert!(config_path.exists());

    // Verify it round-trips through load_config
    let loaded = load_config(&config_path).unwrap();
    assert_eq!(loaded.components.len(), 2);
    assert_eq!(loaded.settings.base_branch, "main");
}

// ---------------------------------------------------------------------------
// 6. validate_detects_bad_globs
// ---------------------------------------------------------------------------
#[test]
fn validate_detects_bad_globs() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(".atomic.toml");
    std::fs::write(
        &path,
        r#"
[[components]]
name = "bad"
globs = ["[invalid"]
"#,
    )
    .unwrap();

    let err = load_config(&path).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("invalid") || msg.contains("glob"),
        "expected glob error, got: {msg}"
    );
}

// ---------------------------------------------------------------------------
// 7. validate_detects_duplicate_names
// ---------------------------------------------------------------------------
#[test]
fn validate_detects_duplicate_names() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(".atomic.toml");
    std::fs::write(
        &path,
        r#"
[[components]]
name = "app"
globs = ["src/**"]

[[components]]
name = "app"
globs = ["lib/**"]
"#,
    )
    .unwrap();

    let err = load_config(&path).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("duplicate"),
        "expected duplicate error, got: {msg}"
    );
}

// ---------------------------------------------------------------------------
// 8. unmatched_files_error_policy
// ---------------------------------------------------------------------------
#[test]
fn unmatched_files_error_policy() {
    let dir = tempfile::tempdir().unwrap();
    let _repo = init_test_repo(dir.path());

    // Commit a file outside any component glob
    commit_file(dir.path(), "README.md", "hello", "add readme");

    let repo = git::open_repo(dir.path()).unwrap();

    // Config with default Error policy (no "ignore")
    let config = Config {
        settings: Settings {
            unmatched_files: UnmatchedPolicy::Error,
            ..Default::default()
        },
        components: vec![git_atomic::config::Component {
            name: "app".into(),
            globs: vec!["src/**".into()],
            commit_type: None,
            branch: None,
        }],
    };
    let matcher = ComponentMatcher::from_config(&config).unwrap();
    let head = git::resolve_commit(&repo, "HEAD").unwrap();

    let result = plan_atomize(&repo, &config, &matcher, head, false);
    assert!(result.is_err(), "should error on unmatched files");
}

// ---------------------------------------------------------------------------
// 9. matcher_groups_files_correctly
// ---------------------------------------------------------------------------
#[test]
fn matcher_groups_files_correctly() {
    let config = test_config(&[
        ("frontend", &["src/ui/**"]),
        ("backend", &["src/api/**"]),
        ("docs", &["docs/**"]),
    ]);
    let matcher = ComponentMatcher::from_config(&config).unwrap();

    let paths: Vec<&Path> = vec![
        Path::new("src/ui/app.ts"),
        Path::new("src/api/handler.rs"),
        Path::new("docs/guide.md"),
        Path::new("README.md"),
    ];

    let (grouped, unmatched) = matcher.group_files(&paths);

    assert_eq!(grouped.len(), 3);
    assert_eq!(grouped[0].0, "frontend");
    assert_eq!(grouped[0].1, vec![Path::new("src/ui/app.ts")]);
    assert_eq!(grouped[1].0, "backend");
    assert_eq!(grouped[1].1, vec![Path::new("src/api/handler.rs")]);
    assert_eq!(grouped[2].0, "docs");
    assert_eq!(grouped[2].1, vec![Path::new("docs/guide.md")]);
    assert_eq!(unmatched, vec![Path::new("README.md")]);
}

// ---------------------------------------------------------------------------
// 10. config_defaults_applied
// ---------------------------------------------------------------------------
#[test]
fn config_defaults_applied() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(".atomic.toml");
    std::fs::write(
        &path,
        r#"
[[components]]
name = "app"
globs = ["**"]
"#,
    )
    .unwrap();

    let config = load_config(&path).unwrap();
    assert_eq!(config.settings.base_branch, "main");
    assert_eq!(config.settings.branch_template, "atomic/{component}");
    assert_eq!(config.settings.unmatched_files, UnmatchedPolicy::Error);
    assert!(config.settings.default_commit_type.is_none());
}
