use crate::config::Config;
use crate::core::effect::{Effect, PlannedRefEdit};
use crate::core::{ComponentMatcher, Error, GitError};
use crate::git::branch::{BranchManager, BranchState};
use crate::git::commit;
use gix::ObjectId;
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

/// Plan atomization of a source commit into per-component branches.
///
/// Returns the results and a list of effects to execute. Tree/commit object
/// writes happen inline (they're immutable and harmless without refs), but
/// the ref transaction is returned as an effect for the caller to execute.
pub fn plan_atomize(
    repo: &gix::Repository,
    config: &Config,
    matcher: &ComponentMatcher,
    source_ref: &str,
    force: bool,
) -> Result<(Vec<AtomicResult>, Vec<Effect>), Error> {
    // 1. Resolve source commit and get changed files
    let source_id = crate::git::resolve_commit(repo, source_ref)?;
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

    // 2. Handle unmatched files
    if !unmatched.is_empty() {
        match config.settings.unmatched_files {
            crate::config::UnmatchedPolicy::Error => {
                return Err(Error::UnmatchedFiles {
                    paths: unmatched.iter().map(|p| p.to_path_buf()).collect(),
                });
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
            }
            crate::config::UnmatchedPolicy::Ignore => {}
        }
    }

    // 3. Resolve base commit for branch management
    let base_id = crate::git::resolve_commit(repo, &config.settings.base_branch)?;
    let branch_mgr = BranchManager::new(repo, base_id, config.settings.branch_template.clone());

    // 4. Build all commits first (all-or-nothing)
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

        // Determine parent and previous ref value
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

        // Build partial tree
        let file_refs: Vec<&std::path::Path> = component_files.iter().map(|p| p.as_ref()).collect();
        let tree_id = commit::build_partial_tree(repo, &source_tree, &file_refs)?;

        // Generate message and create commit object (immutable, safe without refs)
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

    // 5. Collect ref transaction as an effect
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

/// Extract the first line of the commit message as a summary.
fn extract_summary(commit: &gix::Commit<'_>) -> String {
    let msg = commit.message_raw_sloppy();
    let msg_str = String::from_utf8_lossy(msg.as_ref());
    msg_str
        .lines()
        .next()
        .unwrap_or("atomize")
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

        let (results, effects) =
            plan_atomize(&repo, &config, &matcher, "HEAD", false).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(effects.len(), 1); // one RefTransaction

        // Branches should NOT exist yet (effects not executed)
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

        let (results, effects) =
            plan_atomize(&repo, &config, &matcher, "HEAD", false).unwrap();

        // Execute effects to create branches
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

        let (results, effects) =
            plan_atomize(&repo, &config, &matcher, "HEAD", false).unwrap();

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
}
