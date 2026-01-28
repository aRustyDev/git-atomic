# Test Cleanup Procedures

**Purpose:** Shared cleanup procedures for all test phases

---

## After Each Test

### 1. Delete Test Branches

```bash
# Delete local branch
git branch -D <branch-name>

# Delete remote branch
git push origin --delete <branch-name>

# Or use gh
gh pr close <PR-number> --delete-branch
```

### 2. Close Test PRs

```bash
# Close PR and delete branch
gh pr close <PR-number> --delete-branch

# Close without deleting branch
gh pr close <PR-number>
```

### 3. Revert Test Changes (if on protected branch)

```bash
# Create revert commit
git revert <commit-sha>

# Or cherry-pick revert
git cherry-pick -n <commit-sha>
git checkout -- <files-to-keep>
git commit -m "revert: remove test changes"
```

---

## Artifacts to Preserve

| Artifact | Keep? | Reason |
|----------|-------|--------|
| Test chart (`charts/test-workflow/`) | YES | Permanent test fixture |
| Test results documentation | YES | Add to `docs/testing/` |
| Workflow fixes discovered | YES | Commit improvements |
| Test plan documents | YES | Reference for future testing |
| Valid releases from E2E-1 | YES | Reference for attestation tests |

---

## Artifacts to Remove

| Artifact | When to Remove | How to Remove |
|----------|----------------|---------------|
| Test branches | After each test | `git push origin --delete <branch>` |
| Test PRs | After test complete | `gh pr close --delete-branch` |
| Failed test tags | If test-only | `git push origin --delete <tag>` |
| Failed test releases | If test-only | Delete via GitHub UI |
| Temporary files | After verification | `rm -f *.tgz *.json` |

---

## Branch Cleanup Commands

```bash
# List all test branches
git branch -r | grep -E "(test/|charts/test)"

# Delete all test branches (be careful!)
git branch -r | grep "test/" | sed 's/origin\///' | \
  xargs -I {} git push origin --delete {}

# Delete specific chart branches
git push origin --delete charts/test-workflow
```

---

## PR Cleanup Commands

```bash
# List open test PRs
gh pr list --state open --search "test"

# Close all test PRs (interactive)
gh pr list --state open --search "test" --json number --jq '.[].number' | \
  xargs -I {} gh pr close {} --delete-branch

# Close specific PR
gh pr close <number> --delete-branch
```

---

## Tag Cleanup Commands

```bash
# List test tags
git tag -l "test-workflow-*"

# Delete local tag
git tag -d <tag-name>

# Delete remote tag
git push origin --delete <tag-name>

# Delete both
git tag -d <tag-name> && git push origin --delete <tag-name>
```

---

## Release Cleanup

Releases must be deleted via GitHub UI:

1. Go to repository → Releases
2. Find the test release
3. Click "Delete" (trash icon)
4. Confirm deletion

Or via CLI:
```bash
gh release delete <tag-name> --yes
```

---

## GHCR Cleanup

OCI images in GHCR must be deleted via GitHub UI:

1. Go to repository → Packages
2. Find the test chart package
3. Click package → Package settings
4. Delete specific version or entire package

---

## Full Test Suite Cleanup

After completing all tests:

```bash
#!/usr/bin/env bash
set -euo pipefail

REPO="aRustyDev/helm-charts"

echo "=== Cleaning up test artifacts ==="

# 1. Close all test PRs
echo "Closing test PRs..."
gh pr list --repo "$REPO" --state open --search "test" --json number --jq '.[].number' | \
  while read -r pr; do
    echo "  Closing PR #$pr"
    gh pr close "$pr" --repo "$REPO" --delete-branch 2>/dev/null || true
  done

# 2. Delete test branches
echo "Deleting test branches..."
git fetch --prune
git branch -r | grep -E "origin/(test/|feature/test)" | sed 's/origin\///' | \
  while read -r branch; do
    echo "  Deleting $branch"
    git push origin --delete "$branch" 2>/dev/null || true
  done

# 3. Delete chart branches (if test-only)
# Uncomment if needed:
# git push origin --delete charts/test-workflow 2>/dev/null || true

# 4. Clean local files
echo "Cleaning local files..."
rm -f *.tgz *.json 2>/dev/null || true

echo "=== Cleanup complete ==="
```

---

## Recovery Procedures

### Accidentally Deleted Production Branch

```bash
# Find the commit SHA from reflog or GitHub
git reflog show origin/<branch>

# Recreate branch
git push origin <sha>:refs/heads/<branch>
```

### Accidentally Deleted Production Tag

```bash
# Find tag SHA from GitHub releases or reflog
# Recreate tag
git tag <tag-name> <sha>
git push origin <tag-name>
```

### Accidentally Deleted Release

- GitHub Releases are not recoverable
- Must re-run release workflow to recreate
- Tag must still exist or be recreated first

---

## Notes

### Test Isolation

To avoid interference between tests:

- Use unique branch names with test ID prefix: `test/am-t1`, `test/w2-t5`
- Clean up immediately after each test
- Don't reuse chart versions without bumping

### Protected Resources

These should NEVER be deleted during testing:

- `main` branch
- `integration` branch
- `release` branch
- Production tags (`cloudflared-v*`)
- Production releases

### Safe Test Targets

- `charts/test-workflow` - Dedicated test chart
- `test/*` branches - Test-only branches
- `test-workflow-v*` tags - Test chart tags only
