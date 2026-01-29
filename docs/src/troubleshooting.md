# Troubleshooting

## Configuration Not Found

**Error:** `Configuration file not found: .atomic.toml`

**Cause:** git-atomic looks for `.atomic.toml` in the current directory by
default.

**Fix:**
- Create the file: `git-atomic init`
- Or specify a path: `git-atomic commit --config path/to/config.toml`
- Make sure you are running git-atomic from the repository root.

## Unmatched Files

**Error:** `Unmatched files: src/shared/utils.ts (exit code 4)`

**Cause:** Changed files do not match any component's glob patterns, and
`unmatched_files` is set to `"error"` (the default).

**Fix options:**

1. Add the file path to an existing component's globs:

```toml
[[components]]
name = "shared"
globs = ["src/shared/**"]
```

2. Change the policy to warn instead of fail:

```toml
[settings]
unmatched_files = "warn"
```

3. Use `"ignore"` to silently skip unmatched files (not recommended -- you may
   miss files that should be tracked).

## Diverged Branch

**Error:** `Branch atomic/frontend has diverged from main (exit code 5)`

**Cause:** The component branch exists but its history has diverged from the
base branch. This happens when someone pushes directly to a component branch
or when the base branch was force-pushed.

**Fix:**

1. Check the divergence:

```sh
git log --oneline main..atomic/frontend
git log --oneline atomic/frontend..main
```

2. If the component branch can be safely reset:

```sh
git branch -D atomic/frontend
git-atomic commit
```

3. If the component branch has work you need to keep, rebase it:

```sh
git checkout atomic/frontend
git rebase main
git checkout main
git-atomic commit
```

## Invalid Glob Patterns

**Error:** `Invalid glob pattern in component "frontend": [unclosed bracket`

**Cause:** A glob pattern in `.atomic.toml` has a syntax error.

**Fix:** Check your glob patterns for:

- Unclosed brackets `[`
- Unescaped special characters
- Missing closing `**`

Valid examples:

```toml
globs = ["src/**", "lib/*.rs", "config/[a-z]*.toml"]
```

## Dirty Working Tree

**Error:** `Working tree has uncommitted changes (exit code 3)`

**Cause:** git-atomic needs a clean working tree to create component branches.

**Fix:**

```sh
git stash
git-atomic commit
git stash pop
```

Or commit your changes first.

## No Changes to Split

**Error:** `No changed files found in commit`

**Cause:** The specified commit (or HEAD) has no file changes, or all changes
are net-zero in a range.

**Fix:**

- Verify you are pointing at the correct commit: `git log --oneline -1`
- For ranges, check that the range is correct: `git log --oneline main..feature`

## Git Subcommand Not Found

**Error:** `git: 'atomic' is not a git command`

**Cause:** The `git-atomic` binary is not in your `PATH`.

**Fix:**

```sh
# Check if installed
which git-atomic

# If not found, install it
cargo install git-atomic

# Or add cargo bin to PATH
export PATH="$HOME/.cargo/bin:$PATH"
```
