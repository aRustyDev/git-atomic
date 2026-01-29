use crate::core::GitError;
use gix::ObjectId;
use std::path::Path;

/// Build a partial tree containing only the specified files from a source tree.
pub fn build_partial_tree(
    repo: &gix::Repository,
    source_tree: &gix::Tree<'_>,
    files: &[&Path],
) -> Result<ObjectId, GitError> {
    let mut editor = repo
        .edit_tree(gix::hash::ObjectId::empty_tree(repo.object_hash()))
        .map_err(|e| GitError::Operation(format!("create tree editor: {e}")))?;

    for file in files {
        let path_str = file.to_str().ok_or_else(|| GitError::Operation(format!(
            "non-UTF8 path: {}",
            file.display()
        )))?;

        let entry = source_tree
            .lookup_entry_by_path(path_str)
            .map_err(|e| {
                GitError::TreeEntryNotFound {
                    path: format!("{}: {e}", file.display()),
                }
            })?
            .ok_or_else(|| GitError::TreeEntryNotFound {
                path: file.display().to_string(),
            })?;

        editor
            .upsert(path_str, entry.mode().kind(), entry.object_id())
            .map_err(|e| GitError::Operation(format!("upsert tree entry: {e}")))?;
    }

    let tree_id = editor
        .write()
        .map_err(|e| GitError::Operation(format!("write tree: {e}")))?;

    Ok(tree_id.detach())
}

/// Generate a conventional commit message for a component.
pub fn generate_message(component: &str, commit_type: &str, source_summary: &str) -> String {
    format!("{commit_type}({component}): {source_summary}")
}

/// Create a commit object on the repository (does NOT update any ref).
pub fn create_commit(
    repo: &gix::Repository,
    tree_id: ObjectId,
    parent_id: ObjectId,
    message: &str,
    source_author: gix::actor::SignatureRef<'_>,
) -> Result<ObjectId, GitError> {
    let committer_ref = repo
        .committer()
        .transpose()
        .map_err(|e| GitError::Operation(format!("get committer: {e}")))?
        .ok_or_else(|| GitError::Operation("no committer configured".into()))?;

    let commit = gix::objs::Commit {
        tree: tree_id,
        parents: vec![parent_id].into(),
        author: source_author.into(),
        committer: committer_ref.into(),
        encoding: None,
        message: message.into(),
        extra_headers: vec![],
    };

    let id = repo
        .write_object(&commit)
        .map_err(|e| GitError::Operation(format!("write commit: {e}")))?;

    Ok(id.detach())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn git(dir: &std::path::Path, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(dir)
            .output()
            .unwrap();
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }

    fn setup_repo(dir: &std::path::Path) {
        git(dir, &["init"]);
        git(dir, &["config", "user.email", "test@test.com"]);
        git(dir, &["config", "user.name", "Test"]);
        std::fs::create_dir_all(dir.join("src/ui")).unwrap();
        std::fs::create_dir_all(dir.join("src/api")).unwrap();
        std::fs::write(dir.join("src/ui/app.ts"), "// app").unwrap();
        std::fs::write(dir.join("src/api/handler.rs"), "// handler").unwrap();
        git(dir, &["add", "."]);
        git(dir, &["commit", "-m", "add files"]);
    }

    #[test]
    fn partial_tree_contains_only_selected_files() {
        let dir = tempfile::tempdir().unwrap();
        setup_repo(dir.path());

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let head = crate::git::resolve_commit(&repo, "HEAD").unwrap();
        let commit = repo.find_commit(head).unwrap();
        let source_tree = commit.tree().unwrap();

        let files = [Path::new("src/ui/app.ts")];
        let file_refs: Vec<&Path> = files.iter().copied().collect();
        let tree_id = build_partial_tree(&repo, &source_tree, &file_refs).unwrap();

        let tree = repo.find_tree(tree_id).unwrap();
        assert!(tree.lookup_entry_by_path("src/ui/app.ts").unwrap().is_some());
        assert!(tree
            .lookup_entry_by_path("src/api/handler.rs")
            .unwrap()
            .is_none());
    }

    #[test]
    fn generate_message_format() {
        assert_eq!(
            generate_message("frontend", "feat", "add login page"),
            "feat(frontend): add login page"
        );
    }

    #[test]
    fn create_commit_works() {
        let dir = tempfile::tempdir().unwrap();
        setup_repo(dir.path());

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let head = crate::git::resolve_commit(&repo, "HEAD").unwrap();
        let commit = repo.find_commit(head).unwrap();
        let source_tree = commit.tree().unwrap();
        let author = commit.author().unwrap();

        let files = [Path::new("src/ui/app.ts")];
        let file_refs: Vec<&Path> = files.iter().copied().collect();
        let tree_id = build_partial_tree(&repo, &source_tree, &file_refs).unwrap();

        let commit_id =
            create_commit(&repo, tree_id, head, "feat(ui): test commit", author).unwrap();

        let new_commit = repo.find_commit(commit_id).unwrap();
        assert_eq!(
            new_commit.message_raw_sloppy().to_string(),
            "feat(ui): test commit"
        );
    }

    #[test]
    fn missing_entry_errors() {
        let dir = tempfile::tempdir().unwrap();
        setup_repo(dir.path());

        let repo = crate::git::open_repo(dir.path()).unwrap();
        let head = crate::git::resolve_commit(&repo, "HEAD").unwrap();
        let commit = repo.find_commit(head).unwrap();
        let source_tree = commit.tree().unwrap();

        let files = [Path::new("nonexistent.txt")];
        let file_refs: Vec<&Path> = files.iter().copied().collect();
        assert!(build_partial_tree(&repo, &source_tree, &file_refs).is_err());
    }
}
