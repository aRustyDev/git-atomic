# Exit Codes

git-atomic uses specific exit codes to indicate the outcome of an operation.

| Code | Name | Description |
|------|------|-------------|
| 0 | Success | Operation completed successfully |
| 1 | General error | An unexpected error occurred |
| 2 | Configuration error | The `.atomic.toml` file is missing, malformed, or contains invalid values |
| 3 | Git operation error | A git command failed (branch creation, checkout, commit) |
| 4 | Unmatched files | Changed files did not match any component's glob patterns and `unmatched_files` is set to `"error"` |
| 5 | Diverged branch | A component branch exists but has diverged from the base branch, requiring manual resolution |

## When Each Code Occurs

### Code 0 -- Success

All files were classified and all component branches were created or updated
without issues.

### Code 1 -- General Error

An unexpected runtime error such as an I/O failure, permission denied, or an
internal bug. Check the error message for details.

### Code 2 -- Configuration Error

The configuration file could not be loaded. Common causes:

- File not found (no `.atomic.toml` and no `--config` path)
- Invalid TOML syntax
- Missing required fields (`name`, `globs`)
- Duplicate component names

Run `git-atomic validate` to diagnose.

### Code 3 -- Git Operation Error

A git operation failed during the split. Common causes:

- Dirty working tree preventing checkout
- Branch creation conflict
- Repository in an unexpected state (rebase in progress, detached HEAD)

### Code 4 -- Unmatched Files

One or more changed files did not match any component's glob patterns. This
only triggers when `unmatched_files` is set to `"error"` (the default).

Fix by either adding the file paths to an existing component's globs or
changing `unmatched_files` to `"warn"` or `"ignore"`.

### Code 5 -- Diverged Branch

A component branch already exists and its history has diverged from the base
branch. git-atomic refuses to force-update to avoid losing work.

Resolve by manually rebasing or resetting the component branch, then re-run
git-atomic.

## Usage in Scripts

```sh
git-atomic commit
case $? in
  0) echo "Split successful" ;;
  2) echo "Fix your .atomic.toml" ;;
  4) echo "Update your component globs" ;;
  5) echo "Resolve diverged branches" ;;
  *) echo "Error: exit code $?" ;;
esac
```
