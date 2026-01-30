use crate::core::GitError;
use gix::ObjectId;

/// State of an atomic branch relative to the base commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchState {
    /// Branch does not exist yet.
    Missing,
    /// Branch tip equals the base commit.
    Current,
    /// Base is an ancestor of tip (safe to append).
    FastForward { tip: ObjectId },
    /// Neither is an ancestor of the other.
    Diverged { tip: ObjectId },
}

/// Manages atomic branch resolution and state detection.
pub struct BranchManager<'r> {
    repo: &'r gix::Repository,
    base_id: ObjectId,
    branch_template: String,
}

impl<'r> BranchManager<'r> {
    pub fn new(repo: &'r gix::Repository, base_id: ObjectId, branch_template: String) -> Self {
        Self {
            repo,
            base_id,
            branch_template,
        }
    }

    /// Compute the full ref name for a component branch.
    /// Uses the component's branch override if present, otherwise the template.
    pub fn branch_ref_name(&self, component: &str, branch_override: Option<&str>) -> String {
        let branch_name = match branch_override {
            Some(name) => name.to_string(),
            None => self.branch_template.replace("{component}", component),
        };
        format!("refs/heads/{branch_name}")
    }

    /// Determine the state of a branch relative to the base.
    pub fn check_state(&self, ref_name: &str) -> Result<BranchState, GitError> {
        let reference = self
            .repo
            .try_find_reference(ref_name)
            .map_err(|e| GitError::Operation(format!("find reference {ref_name}: {e}")))?;

        let reference = match reference {
            Some(r) => r,
            None => return Ok(BranchState::Missing),
        };

        let tip = reference
            .into_fully_peeled_id()
            .map_err(|e| GitError::Operation(format!("peel reference {ref_name}: {e}")))?
            .detach();

        if tip == self.base_id {
            return Ok(BranchState::Current);
        }

        let merge_base = self
            .repo
            .merge_base(tip, self.base_id)
            .map_err(|e| GitError::Operation(format!("merge_base: {e}")))?;

        if merge_base == self.base_id {
            Ok(BranchState::FastForward { tip })
        } else {
            Ok(BranchState::Diverged { tip })
        }
    }

    /// Determine the parent commit for a new atomic commit on this branch.
    /// Returns the appropriate parent ObjectId, or an error if diverged and not forced.
    pub fn parent_for(&self, ref_name: &str, force: bool) -> Result<ObjectId, GitError> {
        match self.check_state(ref_name)? {
            BranchState::Missing | BranchState::Current => Ok(self.base_id),
            BranchState::FastForward { tip } => Ok(tip),
            BranchState::Diverged { .. } => {
                if force {
                    Ok(self.base_id)
                } else {
                    Err(GitError::Operation(format!(
                        "branch {ref_name} has diverged from base; use --force to override"
                    )))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    fn init_repo(dir: &Path) {
        git(dir, &["init", "-b", "main"]);
        git(dir, &["config", "user.email", "test@test.com"]);
        git(dir, &["config", "user.name", "Test"]);
        std::fs::write(dir.join("file.txt"), "init").unwrap();
        git(dir, &["add", "."]);
        git(dir, &["commit", "-m", "initial"]);
    }

    #[test]
    fn missing_branch() {
        let dir = tempfile::tempdir().unwrap();
        init_repo(dir.path());
        let repo = crate::git::open_repo(dir.path()).unwrap();
        let base = crate::git::resolve_commit(&repo, "HEAD").unwrap();
        let mgr = BranchManager::new(&repo, base, "atomic/{component}".into());

        let ref_name = mgr.branch_ref_name("frontend", None);
        assert_eq!(ref_name, "refs/heads/atomic/frontend");
        assert_eq!(mgr.check_state(&ref_name).unwrap(), BranchState::Missing);
        assert_eq!(mgr.parent_for(&ref_name, false).unwrap(), base);
    }

    #[test]
    fn current_branch() {
        let dir = tempfile::tempdir().unwrap();
        init_repo(dir.path());
        git(dir.path(), &["branch", "atomic/test"]);
        let repo = crate::git::open_repo(dir.path()).unwrap();
        let base = crate::git::resolve_commit(&repo, "HEAD").unwrap();
        let mgr = BranchManager::new(&repo, base, "atomic/{component}".into());

        let state = mgr.check_state("refs/heads/atomic/test").unwrap();
        assert_eq!(state, BranchState::Current);
    }

    #[test]
    fn fast_forward_branch() {
        let dir = tempfile::tempdir().unwrap();
        init_repo(dir.path());
        // Create atomic branch, add a commit to it
        git(dir.path(), &["checkout", "-b", "atomic/ff"]);
        std::fs::write(dir.path().join("extra.txt"), "more").unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "extra"]);
        git(dir.path(), &["checkout", "main"]);

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let base = crate::git::resolve_commit(&repo, "HEAD").unwrap();
        let mgr = BranchManager::new(&repo, base, "atomic/{component}".into());

        let state = mgr.check_state("refs/heads/atomic/ff").unwrap();
        assert!(matches!(state, BranchState::FastForward { .. }));
        // parent_for should return the tip
        let parent = mgr.parent_for("refs/heads/atomic/ff", false).unwrap();
        assert_ne!(parent, base);
    }

    #[test]
    fn diverged_branch() {
        let dir = tempfile::tempdir().unwrap();
        init_repo(dir.path());
        // Create atomic branch with a commit
        git(dir.path(), &["checkout", "-b", "atomic/div"]);
        std::fs::write(dir.path().join("branch.txt"), "branch").unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "branch commit"]);
        // Go back to main and add a different commit
        git(dir.path(), &["checkout", "main"]);
        std::fs::write(dir.path().join("main.txt"), "main").unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "main commit"]);

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let base = crate::git::resolve_commit(&repo, "HEAD").unwrap();
        let mgr = BranchManager::new(&repo, base, "atomic/{component}".into());

        let state = mgr.check_state("refs/heads/atomic/div").unwrap();
        assert!(matches!(state, BranchState::Diverged { .. }));

        // Without force: error
        assert!(mgr.parent_for("refs/heads/atomic/div", false).is_err());
        // With force: returns base
        assert_eq!(mgr.parent_for("refs/heads/atomic/div", true).unwrap(), base);
    }

    #[test]
    fn branch_override() {
        let dir = tempfile::tempdir().unwrap();
        init_repo(dir.path());
        let repo = crate::git::open_repo(dir.path()).unwrap();
        let base = crate::git::resolve_commit(&repo, "HEAD").unwrap();
        let mgr = BranchManager::new(&repo, base, "atomic/{component}".into());

        let ref_name = mgr.branch_ref_name("frontend", Some("custom/branch"));
        assert_eq!(ref_name, "refs/heads/custom/branch");
    }
}
