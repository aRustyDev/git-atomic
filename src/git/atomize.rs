use crate::config::Config;
use crate::core::effect::{Effect, PlannedRefEdit};
use crate::core::{ComponentMatcher, Error, GitError};
use crate::git::branch::{BranchManager, BranchState};
use crate::git::commit;
use gix::ObjectId;
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

/// Result of atomizing a single component.
#[derive(Debug)]
pub struct AtomicResult {
    pub component: String,
    pub branch: String,
    pub commit_id: ObjectId,
    pub files: Vec<PathBuf>,
    pub created: bool,
}

/// Plan atomization of a single source commit into per-component branches.
///
/// Returns the results and a list of effects to execute. Tree/commit object
/// writes happen inline (they're immutable and harmless without refs), but
/// the ref transaction is returned as an effect for the caller to execute.
pub fn plan_atomize(
    repo: &gix::Repository,
    config: &Config,
    matcher: &ComponentMatcher,
    source_id: ObjectId,
    force: bool,
) -> Result<(Vec<AtomicResult>, Vec<Effect>), Error> {
    let source_commit = repo
        .find_commit(source_id)
        .map_err(|e| GitError::Operation(format!("find source commit: {e}")))?;
    let source_tree = source_commit
        .tree()
        .map_err(|e| GitError::Operation(format!("source tree: {e}")))?;
    let source_author = source_commit
        .author()
        .map_err(|e| GitError::Operation(format!("source author: {e}")))?;
    let source_summary = extract_summary(&source_commit);

    let files = crate::git::changed_files(repo, source_id)?;
    let path_refs: Vec<&std::path::Path> = files.iter().map(|p| p.as_path()).collect();
    let (grouped, unmatched) = matcher.group_files(&path_refs);

    // Handle unmatched files
    if !unmatched.is_empty() {
        handle_unmatched(config, &unmatched)?;
    }

    // Resolve base commit for branch management
    let base_id = crate::git::resolve_commit(repo, &config.settings.base_branch)?;
    let branch_mgr = BranchManager::new(repo, base_id, config.settings.branch_template.clone());

    // Build all commits (all-or-nothing)
    let mut planned_edits: Vec<PlannedRefEdit> = Vec::new();
    let mut results: Vec<AtomicResult> = Vec::new();

    for (component_name, component_files) in &grouped {
        let component_config = config.components.iter().find(|c| c.name == *component_name);
        let branch_override = component_config.and_then(|c| c.branch.as_deref());
        let ref_name = branch_mgr.branch_ref_name(component_name, branch_override);

        let commit_type = component_config
            .and_then(|c| c.commit_type.as_deref())
            .or(config.settings.default_commit_type.as_deref())
            .unwrap_or("feat");

        let state = branch_mgr.check_state(&ref_name)?;
        let parent_id = branch_mgr.parent_for(&ref_name, force).map_err(|_| {
            Error::DivergedBranch {
                branch: ref_name.clone(),
                base: config.settings.base_branch.clone(),
            }
        })?;

        let created = matches!(state, BranchState::Missing);
        let previous = match &state {
            BranchState::Missing => None,
            BranchState::Current => Some(base_id),
            BranchState::FastForward { tip } | BranchState::Diverged { tip } => Some(*tip),
        };

        let file_refs: Vec<&std::path::Path> = component_files.iter().map(|p| p.as_ref()).collect();
        let tree_id = commit::build_partial_tree(repo, &source_tree, &file_refs)?;

        let message = commit::generate_message(component_name, commit_type, &source_summary);
        let commit_id = commit::create_commit(repo, tree_id, parent_id, &message, source_author)?;

        planned_edits.push(PlannedRefEdit {
            ref_name: ref_name.clone(),
            new_id: commit_id,
            previous,
            component: component_name.to_string(),
            created,
        });

        let branch_display = ref_name
            .strip_prefix("refs/heads/")
            .unwrap_or(&ref_name)
            .to_string();

        results.push(AtomicResult {
            component: component_name.to_string(),
            branch: branch_display,
            commit_id,
            files: component_files.iter().map(|p| p.to_path_buf()).collect(),
            created,
        });
    }

    let mut effects = Vec::new();
    if !planned_edits.is_empty() {
        let repo_path = repo
            .path()
            .parent()
            .unwrap_or(repo.path())
            .to_path_buf();
        effects.push(Effect::RefTransaction {
            repo_path,
            edits: planned_edits,
        });
    }

    Ok((results, effects))
}

/// Plan atomization of a range of commits with partial-squash semantics.
///
/// Net-zero files (unchanged between range endpoints) are filtered out.
/// Commits with no remaining effective changes are skipped. Each component
/// branch gets incremental (cumulative) trees — coherent and checkable-out
/// at every point.
pub fn plan_atomize_range(
    repo: &gix::Repository,
    config: &Config,
    matcher: &ComponentMatcher,
    commits: &[ObjectId],
    effective_files: &HashSet<PathBuf>,
    force: bool,
) -> Result<(Vec<AtomicResult>, Vec<Effect>), Error> {
    if commits.is_empty() {
        return Ok((Vec::new(), Vec::new()));
    }

    let base_id = crate::git::resolve_commit(repo, &config.settings.base_branch)?;
    let branch_mgr = BranchManager::new(repo, base_id, config.settings.branch_template.clone());

    // Track per-component state across commits:
    // - cumulative files seen so far (for incremental trees)
    // - current parent commit id (for chaining)
    // - whether the branch existed before this range
    struct ComponentState {
        cumulative_files: Vec<PathBuf>,
        parent_id: ObjectId,
        ref_name: String,
        branch_display: String,
        created: bool,
        /// The previous ref value before any range processing (for the RefTransaction).
        original_previous: Option<ObjectId>,
        /// The latest commit id written for this component.
        latest_commit_id: ObjectId,
        /// Total effective files across all commits (for the final AtomicResult).
        all_files: Vec<PathBuf>,
    }

    let mut component_states: BTreeMap<String, ComponentState> = BTreeMap::new();
    let mut all_results: Vec<AtomicResult> = Vec::new();

    for &commit_id in commits {
        let changed = crate::git::changed_files(repo, commit_id)?;
        // Filter to only effective files
        let effective_changed: Vec<PathBuf> = changed
            .into_iter()
            .filter(|p| effective_files.contains(p))
            .collect();

        if effective_changed.is_empty() {
            continue;
        }

        let source_commit = repo
            .find_commit(commit_id)
            .map_err(|e| GitError::Operation(format!("find commit: {e}")))?;
        let source_tree = source_commit
            .tree()
            .map_err(|e| GitError::Operation(format!("source tree: {e}")))?;
        let source_author = source_commit
            .author()
            .map_err(|e| GitError::Operation(format!("source author: {e}")))?;
        let source_summary = extract_summary(&source_commit);

        // Handle unmatched effective files
        let path_refs: Vec<&std::path::Path> = effective_changed.iter().map(|p| p.as_path()).collect();
        let (grouped, unmatched) = matcher.group_files(&path_refs);

        if !unmatched.is_empty() {
            handle_unmatched(config, &unmatched)?;
        }

        for (component_name, component_files) in &grouped {
            let component_config = config.components.iter().find(|c| c.name == *component_name);
            let commit_type = component_config
                .and_then(|c| c.commit_type.as_deref())
                .or(config.settings.default_commit_type.as_deref())
                .unwrap_or("feat");

            let state = component_states.get_mut(component_name as &str);

            match state {
                Some(cs) => {
                    // Add new files to cumulative set (avoid duplicates)
                    for f in component_files {
                        let pb = f.to_path_buf();
                        if !cs.cumulative_files.contains(&pb) {
                            cs.cumulative_files.push(pb.clone());
                        }
                        if !cs.all_files.contains(&pb) {
                            cs.all_files.push(pb);
                        }
                    }

                    // Build incremental tree from cumulative files
                    let file_refs: Vec<&std::path::Path> =
                        cs.cumulative_files.iter().map(|p| p.as_path()).collect();
                    let tree_id = commit::build_partial_tree(repo, &source_tree, &file_refs)?;

                    let message = commit::generate_message(component_name, commit_type, &source_summary);
                    let new_commit_id = commit::create_commit(
                        repo, tree_id, cs.parent_id, &message, source_author,
                    )?;

                    cs.parent_id = new_commit_id;
                    cs.latest_commit_id = new_commit_id;
                }
                None => {
                    // First time seeing this component — initialize state
                    let branch_override = component_config.and_then(|c| c.branch.as_deref());
                    let ref_name = branch_mgr.branch_ref_name(component_name, branch_override);

                    let branch_state = branch_mgr.check_state(&ref_name)?;
                    let parent_id = branch_mgr.parent_for(&ref_name, force).map_err(|_| {
                        Error::DivergedBranch {
                            branch: ref_name.clone(),
                            base: config.settings.base_branch.clone(),
                        }
                    })?;

                    let created = matches!(branch_state, BranchState::Missing);
                    let original_previous = match &branch_state {
                        BranchState::Missing => None,
                        BranchState::Current => Some(base_id),
                        BranchState::FastForward { tip } | BranchState::Diverged { tip } => {
                            Some(*tip)
                        }
                    };

                    let cumulative_files: Vec<PathBuf> =
                        component_files.iter().map(|p| p.to_path_buf()).collect();
                    let file_refs: Vec<&std::path::Path> =
                        cumulative_files.iter().map(|p| p.as_path()).collect();
                    let tree_id = commit::build_partial_tree(repo, &source_tree, &file_refs)?;

                    let message = commit::generate_message(component_name, commit_type, &source_summary);
                    let new_commit_id = commit::create_commit(
                        repo, tree_id, parent_id, &message, source_author,
                    )?;

                    let branch_display = ref_name
                        .strip_prefix("refs/heads/")
                        .unwrap_or(&ref_name)
                        .to_string();

                    component_states.insert(
                        component_name.to_string(),
                        ComponentState {
                            cumulative_files: cumulative_files.clone(),
                            parent_id: new_commit_id,
                            ref_name,
                            branch_display,
                            created,
                            original_previous,
                            latest_commit_id: new_commit_id,
                            all_files: cumulative_files,
                        },
                    );
                }
            }
        }
    }

    // Build ref edits and results from final component states
    let mut planned_edits: Vec<PlannedRefEdit> = Vec::new();

    for (name, cs) in &component_states {
        planned_edits.push(PlannedRefEdit {
            ref_name: cs.ref_name.clone(),
            new_id: cs.latest_commit_id,
            previous: cs.original_previous,
            component: name.clone(),
            created: cs.created,
        });

        all_results.push(AtomicResult {
            component: name.clone(),
            branch: cs.branch_display.clone(),
            commit_id: cs.latest_commit_id,
            files: cs.all_files.clone(),
            created: cs.created,
        });
    }

    let mut effects = Vec::new();
    if !planned_edits.is_empty() {
        let repo_path = repo
            .path()
            .parent()
            .unwrap_or(repo.path())
            .to_path_buf();
        effects.push(Effect::RefTransaction {
            repo_path,
            edits: planned_edits,
        });
    }

    Ok((all_results, effects))
}

/// Handle unmatched files according to the configured policy.
fn handle_unmatched(config: &Config, unmatched: &[&std::path::Path]) -> Result<(), Error> {
    match config.settings.unmatched_files {
        crate::config::UnmatchedPolicy::Error => {
            Err(Error::UnmatchedFiles {
                paths: unmatched.iter().map(|p| p.to_path_buf()).collect(),
            })
        }
        crate::config::UnmatchedPolicy::Warn => {
            eprintln!(
                "warning: unmatched files: {}",
                unmatched
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            Ok(())
        }
        crate::config::UnmatchedPolicy::Ignore => Ok(()),
    }
}

/// Extract the first line of the commit message as a summary.
fn extract_summary(commit: &gix::Commit<'_>) -> String {
    let msg = commit.message_raw_sloppy();
    let msg_str = String::from_utf8_lossy(msg.as_ref());
    msg_str
        .lines()
        .next()
        .unwrap_or("commit")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::core::ComponentMatcher;
    use std::path::Path;
    use std::process::Command;

    fn git(dir: &Path, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(dir)
            .output()
            .unwrap();
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }

    fn setup_multi_component_repo(dir: &Path) {
        git(dir, &["init"]);
        git(dir, &["config", "user.email", "test@test.com"]);
        git(dir, &["config", "user.name", "Test"]);

        // Initial empty commit on main
        git(dir, &["commit", "--allow-empty", "-m", "initial"]);

        // Add multi-component files
        std::fs::create_dir_all(dir.join("src/ui")).unwrap();
        std::fs::create_dir_all(dir.join("src/api")).unwrap();
        std::fs::write(dir.join("src/ui/app.ts"), "// frontend").unwrap();
        std::fs::write(dir.join("src/api/handler.rs"), "// backend").unwrap();
        git(dir, &["add", "."]);
        git(dir, &["commit", "-m", "add components"]);
    }

    fn test_config() -> Config {
        let toml_str = r#"
[settings]
base_branch = "main"
unmatched_files = "ignore"

[[components]]
name = "frontend"
globs = ["src/ui/**"]

[[components]]
name = "backend"
globs = ["src/api/**"]
commit_type = "fix"
"#;
        toml::from_str(toml_str).unwrap()
    }

    #[test]
    fn plan_atomize_returns_effects_without_mutating() {
        let dir = tempfile::tempdir().unwrap();
        setup_multi_component_repo(dir.path());

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let config = test_config();
        let matcher = ComponentMatcher::from_config(&config).unwrap();
        let head = crate::git::resolve_commit(&repo, "HEAD").unwrap();

        let (results, effects) =
            plan_atomize(&repo, &config, &matcher, head, false).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(effects.len(), 1);

        for result in &results {
            let ref_name = format!("refs/heads/{}", result.branch);
            let reference = repo.try_find_reference(&ref_name).unwrap();
            assert!(reference.is_none(), "branch {} should not exist yet", result.branch);
        }
    }

    #[test]
    fn atomize_creates_branches() {
        let dir = tempfile::tempdir().unwrap();
        setup_multi_component_repo(dir.path());

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let config = test_config();
        let matcher = ComponentMatcher::from_config(&config).unwrap();
        let head = crate::git::resolve_commit(&repo, "HEAD").unwrap();

        let (results, effects) =
            plan_atomize(&repo, &config, &matcher, head, false).unwrap();

        let printer = crate::cli::output::Printer::new(false, true, 0);
        crate::core::effect::execute(Some(&repo), &effects, false, &printer).unwrap();

        assert_eq!(results.len(), 2);

        let names: Vec<&str> = results.iter().map(|r| r.component.as_str()).collect();
        assert!(names.contains(&"frontend"));
        assert!(names.contains(&"backend"));

        for result in &results {
            assert!(result.created);
            let ref_name = format!("refs/heads/{}", result.branch);
            let reference = repo.try_find_reference(&ref_name).unwrap();
            assert!(reference.is_some(), "branch {} should exist", result.branch);
        }

        for result in &results {
            let c = repo.find_commit(result.commit_id).unwrap();
            let msg = c.message_raw_sloppy().to_string();
            if result.component == "frontend" {
                assert!(msg.starts_with("feat(frontend):"), "got: {msg}");
            } else {
                assert!(msg.starts_with("fix(backend):"), "got: {msg}");
            }
        }
    }

    #[test]
    fn atomize_partial_trees_are_isolated() {
        let dir = tempfile::tempdir().unwrap();
        setup_multi_component_repo(dir.path());

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let config = test_config();
        let matcher = ComponentMatcher::from_config(&config).unwrap();
        let head = crate::git::resolve_commit(&repo, "HEAD").unwrap();

        let (results, effects) =
            plan_atomize(&repo, &config, &matcher, head, false).unwrap();

        let printer = crate::cli::output::Printer::new(false, true, 0);
        crate::core::effect::execute(Some(&repo), &effects, false, &printer).unwrap();

        for result in &results {
            let c = repo.find_commit(result.commit_id).unwrap();
            let tree = c.tree().unwrap();

            if result.component == "frontend" {
                assert!(tree.lookup_entry_by_path("src/ui/app.ts").unwrap().is_some());
                assert!(tree
                    .lookup_entry_by_path("src/api/handler.rs")
                    .unwrap()
                    .is_none());
            } else {
                assert!(tree
                    .lookup_entry_by_path("src/api/handler.rs")
                    .unwrap()
                    .is_some());
                assert!(tree.lookup_entry_by_path("src/ui/app.ts").unwrap().is_none());
            }
        }
    }

    #[test]
    fn range_filters_net_zero_files() {
        let dir = tempfile::tempdir().unwrap();
        git(dir.path(), &["init"]);
        git(dir.path(), &["config", "user.email", "test@test.com"]);
        git(dir.path(), &["config", "user.name", "Test"]);
        git(dir.path(), &["commit", "--allow-empty", "-m", "initial"]);

        let base = git(dir.path(), &["rev-parse", "HEAD"]);

        // c1: add foo.ts and bar.ts
        std::fs::create_dir_all(dir.path().join("src/ui")).unwrap();
        std::fs::write(dir.path().join("src/ui/foo.ts"), "foo").unwrap();
        std::fs::write(dir.path().join("src/ui/bar.ts"), "bar").unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "feat: initial UI"]);

        // c2: modify foo.ts
        std::fs::write(dir.path().join("src/ui/foo.ts"), "foo modified").unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "fix: layout bug"]);

        // c3: delete foo.ts
        std::fs::remove_file(dir.path().join("src/ui/foo.ts")).unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "refactor: remove foo"]);

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let start_id = crate::git::resolve_commit(&repo, &base).unwrap();
        let end_id = crate::git::resolve_commit(&repo, "HEAD").unwrap();

        let commits = crate::git::walk::walk_range(&repo, start_id, end_id).unwrap();
        let effective = crate::git::walk::effective_files(&repo, start_id, end_id).unwrap();

        // foo.ts is net-zero (added then deleted), bar.ts is effective
        assert!(effective.contains(&PathBuf::from("src/ui/bar.ts")));
        assert!(!effective.contains(&PathBuf::from("src/ui/foo.ts")));

        let config_str = r#"
[settings]
base_branch = "main"
unmatched_files = "ignore"

[[components]]
name = "frontend"
globs = ["src/ui/**"]
"#;
        let config: Config = toml::from_str(config_str).unwrap();
        let matcher = ComponentMatcher::from_config(&config).unwrap();

        let (results, effects) =
            plan_atomize_range(&repo, &config, &matcher, &commits, &effective, false).unwrap();

        // Only one component result (frontend)
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].component, "frontend");
        // Files should only include bar.ts
        assert_eq!(results[0].files, vec![PathBuf::from("src/ui/bar.ts")]);

        // Execute and verify the branch
        let printer = crate::cli::output::Printer::new(false, true, 0);
        crate::core::effect::execute(Some(&repo), &effects, false, &printer).unwrap();

        let c = repo.find_commit(results[0].commit_id).unwrap();
        let tree = c.tree().unwrap();
        assert!(tree.lookup_entry_by_path("src/ui/bar.ts").unwrap().is_some());
        assert!(tree.lookup_entry_by_path("src/ui/foo.ts").unwrap().is_none());
    }

    #[test]
    fn range_incremental_trees() {
        let dir = tempfile::tempdir().unwrap();
        git(dir.path(), &["init"]);
        git(dir.path(), &["config", "user.email", "test@test.com"]);
        git(dir.path(), &["config", "user.name", "Test"]);
        git(dir.path(), &["commit", "--allow-empty", "-m", "initial"]);

        let base = git(dir.path(), &["rev-parse", "HEAD"]);

        // c1: add bar.ts
        std::fs::create_dir_all(dir.path().join("src/ui")).unwrap();
        std::fs::write(dir.path().join("src/ui/bar.ts"), "bar").unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "feat: add bar"]);

        // c2: add baz.ts
        std::fs::write(dir.path().join("src/ui/baz.ts"), "baz").unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "feat: add baz"]);

        // c3: modify baz.ts
        std::fs::write(dir.path().join("src/ui/baz.ts"), "baz modified").unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "fix: update baz"]);

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let start_id = crate::git::resolve_commit(&repo, &base).unwrap();
        let end_id = crate::git::resolve_commit(&repo, "HEAD").unwrap();

        let commits = crate::git::walk::walk_range(&repo, start_id, end_id).unwrap();
        let effective = crate::git::walk::effective_files(&repo, start_id, end_id).unwrap();

        assert_eq!(commits.len(), 3);
        // bar.ts, baz.ts, plus directory entries (src, src/ui)
        assert_eq!(effective.len(), 4);

        let config_str = r#"
[settings]
base_branch = "main"
unmatched_files = "ignore"

[[components]]
name = "frontend"
globs = ["src/ui/**"]
"#;
        let config: Config = toml::from_str(config_str).unwrap();
        let matcher = ComponentMatcher::from_config(&config).unwrap();

        let (results, effects) =
            plan_atomize_range(&repo, &config, &matcher, &commits, &effective, false).unwrap();

        let printer = crate::cli::output::Printer::new(false, true, 0);
        crate::core::effect::execute(Some(&repo), &effects, false, &printer).unwrap();

        assert_eq!(results.len(), 1);
        let final_commit = repo.find_commit(results[0].commit_id).unwrap();
        let final_tree = final_commit.tree().unwrap();

        // Final tree should have both bar.ts and baz.ts (incremental)
        assert!(final_tree.lookup_entry_by_path("src/ui/bar.ts").unwrap().is_some());
        assert!(final_tree.lookup_entry_by_path("src/ui/baz.ts").unwrap().is_some());

        // Walk back: the final commit's parent should also have bar.ts
        let parent_id = final_commit.parent_ids().next().unwrap().detach();
        let parent_commit = repo.find_commit(parent_id).unwrap();
        let parent_tree = parent_commit.tree().unwrap();
        assert!(parent_tree.lookup_entry_by_path("src/ui/bar.ts").unwrap().is_some());
        assert!(parent_tree.lookup_entry_by_path("src/ui/baz.ts").unwrap().is_some());

        // And the grandparent (first component commit) should have only bar.ts
        let grandparent_id = parent_commit.parent_ids().next().unwrap().detach();
        let grandparent_commit = repo.find_commit(grandparent_id).unwrap();
        let grandparent_tree = grandparent_commit.tree().unwrap();
        assert!(grandparent_tree.lookup_entry_by_path("src/ui/bar.ts").unwrap().is_some());
        assert!(grandparent_tree.lookup_entry_by_path("src/ui/baz.ts").unwrap().is_none());
    }

    #[test]
    fn range_empty_produces_no_results() {
        let dir = tempfile::tempdir().unwrap();
        git(dir.path(), &["init"]);
        git(dir.path(), &["config", "user.email", "test@test.com"]);
        git(dir.path(), &["config", "user.name", "Test"]);
        git(dir.path(), &["commit", "--allow-empty", "-m", "initial"]);

        let config_str = r#"
[settings]
base_branch = "main"
unmatched_files = "ignore"

[[components]]
name = "app"
globs = ["src/**"]
"#;
        let config: Config = toml::from_str(config_str).unwrap();
        let matcher = ComponentMatcher::from_config(&config).unwrap();
        let repo = crate::git::open_repo(dir.path()).unwrap();

        let (results, effects) =
            plan_atomize_range(&repo, &config, &matcher, &[], &HashSet::new(), false).unwrap();

        assert!(results.is_empty());
        assert!(effects.is_empty());
    }
}
