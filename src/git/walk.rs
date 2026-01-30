use crate::core::GitError;
use gix::ObjectId;
use std::collections::HashSet;
use std::path::PathBuf;

/// Walk commits reachable from `end` but not from `start`, returned oldest-first.
///
/// This is equivalent to `git log --reverse start..end`.
pub fn walk_range(
    repo: &gix::Repository,
    start: ObjectId,
    end: ObjectId,
) -> Result<Vec<ObjectId>, GitError> {
    // Collect ancestors of start (the exclusion set)
    let mut excluded = HashSet::new();
    let mut queue = vec![start];
    while let Some(id) = queue.pop() {
        if excluded.insert(id)
            && let Ok(commit) = repo.find_commit(id)
        {
            for parent in commit.parent_ids() {
                queue.push(parent.detach());
            }
        }
    }

    // Walk from end, collecting commits not in the exclusion set
    let mut result = Vec::new();
    let mut walk_queue = vec![end];
    let mut visited = HashSet::new();
    while let Some(id) = walk_queue.pop() {
        if excluded.contains(&id) || !visited.insert(id) {
            continue;
        }
        result.push(id);
        if let Ok(commit) = repo.find_commit(id) {
            for parent in commit.parent_ids() {
                walk_queue.push(parent.detach());
            }
        }
    }

    // Reverse to get oldest-first (topological)
    result.reverse();
    Ok(result)
}

/// Compute the set of files that differ between two commits (effective files).
///
/// Files that are identical (or absent) at both endpoints are net-zero and
/// excluded from the result. Only files that actually changed between the
/// two trees are returned.
pub fn effective_files(
    repo: &gix::Repository,
    start: ObjectId,
    end: ObjectId,
) -> Result<HashSet<PathBuf>, GitError> {
    let start_commit = repo
        .find_commit(start)
        .map_err(|e| GitError::Operation(format!("find start commit: {e}")))?;
    let start_tree = start_commit
        .tree()
        .map_err(|e| GitError::Operation(format!("start tree: {e}")))?;

    let end_commit = repo
        .find_commit(end)
        .map_err(|e| GitError::Operation(format!("find end commit: {e}")))?;
    let end_tree = end_commit
        .tree()
        .map_err(|e| GitError::Operation(format!("end tree: {e}")))?;

    let changes = repo
        .diff_tree_to_tree(Some(&start_tree), Some(&end_tree), None)
        .map_err(|e| GitError::Operation(format!("diff start..end: {e}")))?;

    let paths: HashSet<PathBuf> = changes
        .iter()
        .map(|change| PathBuf::from(change.location().to_string()))
        .collect();

    Ok(paths)
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
        git(dir, &["commit", "--allow-empty", "-m", "initial"]);
    }

    #[test]
    fn walk_range_returns_commits_oldest_first() {
        let dir = tempfile::tempdir().unwrap();
        init_repo(dir.path());

        let base = git(dir.path(), &["rev-parse", "HEAD"]);

        std::fs::write(dir.path().join("a.txt"), "a").unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "commit 1"]);

        std::fs::write(dir.path().join("b.txt"), "b").unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "commit 2"]);

        std::fs::write(dir.path().join("c.txt"), "c").unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "commit 3"]);

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let start = crate::git::resolve_commit(&repo, &base).unwrap();
        let end = crate::git::resolve_commit(&repo, "HEAD").unwrap();

        let commits = walk_range(&repo, start, end).unwrap();
        assert_eq!(commits.len(), 3);

        // Verify oldest-first: first commit should be "commit 1"
        let first = repo.find_commit(commits[0]).unwrap();
        let msg = first.message_raw_sloppy().to_string();
        assert!(msg.contains("commit 1"), "expected 'commit 1', got: {msg}");

        let last = repo.find_commit(commits[2]).unwrap();
        let msg = last.message_raw_sloppy().to_string();
        assert!(msg.contains("commit 3"), "expected 'commit 3', got: {msg}");
    }

    #[test]
    fn walk_range_empty_when_same_commit() {
        let dir = tempfile::tempdir().unwrap();
        init_repo(dir.path());

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let head = crate::git::resolve_commit(&repo, "HEAD").unwrap();

        let commits = walk_range(&repo, head, head).unwrap();
        assert!(commits.is_empty());
    }

    #[test]
    fn effective_files_detects_net_zero() {
        let dir = tempfile::tempdir().unwrap();
        init_repo(dir.path());

        let base = git(dir.path(), &["rev-parse", "HEAD"]);

        // Add foo and bar
        std::fs::write(dir.path().join("foo.txt"), "foo").unwrap();
        std::fs::write(dir.path().join("bar.txt"), "bar").unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "add files"]);

        // Delete foo (net-zero: added then deleted)
        std::fs::remove_file(dir.path().join("foo.txt")).unwrap();
        git(dir.path(), &["add", "."]);
        git(dir.path(), &["commit", "-m", "remove foo"]);

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let start = crate::git::resolve_commit(&repo, &base).unwrap();
        let end = crate::git::resolve_commit(&repo, "HEAD").unwrap();

        let effective = effective_files(&repo, start, end).unwrap();

        // bar.txt is effective (added and still present)
        assert!(effective.contains(&PathBuf::from("bar.txt")));
        // foo.txt is net-zero (added then deleted)
        assert!(!effective.contains(&PathBuf::from("foo.txt")));
    }

    #[test]
    fn effective_files_empty_when_same() {
        let dir = tempfile::tempdir().unwrap();
        init_repo(dir.path());

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let head = crate::git::resolve_commit(&repo, "HEAD").unwrap();

        let effective = effective_files(&repo, head, head).unwrap();
        assert!(effective.is_empty());
    }
}
