use crate::core::GitError;
use gix::ObjectId;
use std::path::{Path, PathBuf};

/// Open a git repository at or above `path`.
pub fn open_repo(path: &Path) -> Result<gix::Repository, GitError> {
    gix::open(path).map_err(|e| match e {
        gix::open::Error::NotARepository { path, .. } => GitError::NotARepo { path },
        other => GitError::Gix(Box::new(other)),
    })
}

/// Resolve a revision string (e.g. "HEAD", "abc1234") to an `ObjectId`.
pub fn resolve_commit(repo: &gix::Repository, reference: &str) -> Result<ObjectId, GitError> {
    let obj = repo
        .rev_parse_single(reference.as_bytes())
        .map_err(|e| GitError::ResolveRef {
            reference: reference.to_string(),
            reason: e.to_string(),
        })?;
    Ok(obj.detach())
}

/// Return the list of changed file paths between a commit and its parent.
/// For initial commits (no parent), diffs against the empty tree.
pub fn changed_files(
    repo: &gix::Repository,
    commit_id: ObjectId,
) -> Result<Vec<PathBuf>, GitError> {
    let commit = repo
        .find_commit(commit_id)
        .map_err(|e| GitError::Operation(format!("find commit: {e}")))?;
    let tree = commit
        .tree()
        .map_err(|e| GitError::Operation(format!("get tree: {e}")))?;

    let parent_tree = match commit.parent_ids().next() {
        Some(parent_id) => {
            let parent = repo
                .find_commit(parent_id.detach())
                .map_err(|e| GitError::Operation(format!("find parent: {e}")))?;
            Some(
                parent
                    .tree()
                    .map_err(|e| GitError::Operation(format!("parent tree: {e}")))?,
            )
        }
        None => None,
    };

    let changes = repo
        .diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)
        .map_err(|e| GitError::Operation(format!("diff: {e}")))?;

    let paths: Vec<PathBuf> = changes
        .iter()
        .map(|change| PathBuf::from(change.location().to_string()))
        .collect();

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn init_repo_with_commit(dir: &Path) -> ObjectId {
        Command::new("git")
            .args(["init", "-b", "main"])
            .current_dir(dir)
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(dir)
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(dir)
            .output()
            .unwrap();

        std::fs::write(dir.join("hello.txt"), "hello").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(dir)
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "initial"])
            .current_dir(dir)
            .output()
            .unwrap();

        let repo = open_repo(dir).unwrap();
        resolve_commit(&repo, "HEAD").unwrap()
    }

    #[test]
    fn open_valid_repo() {
        let dir = tempfile::tempdir().unwrap();
        Command::new("git")
            .args(["init", "-b", "main"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        assert!(open_repo(dir.path()).is_ok());
    }

    #[test]
    fn open_not_a_repo() {
        let dir = tempfile::tempdir().unwrap();
        assert!(open_repo(dir.path()).is_err());
    }

    #[test]
    fn initial_commit_changed_files() {
        let dir = tempfile::tempdir().unwrap();
        let id = init_repo_with_commit(dir.path());
        let repo = open_repo(dir.path()).unwrap();
        let files = changed_files(&repo, id).unwrap();
        assert_eq!(files, vec![PathBuf::from("hello.txt")]);
    }
}
