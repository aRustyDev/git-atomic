# Phase 7: Unified Ref Argument

**Status**: Not Started
**Issue**: TBD
**Parent**: [MVP Plan](../index.md)

## Scope

Merge `--ref` and `--range` into a single positional argument that accepts either a single ref or a `..` range. Remove the `--range` flag entirely. Implement range mode with partial-squash semantics.

## Context

The `commit` subcommand currently has two overlapping flags:

- `--ref <REF>` — a single commit to diff against its parent
- `--range <START..END>` — a range of commits (defined but **not implemented**)

These should be a single argument. Git users already understand `..` syntax from `git log`, `git diff`, etc. A unified argument follows git conventions and eliminates ambiguity.

## Supported Syntax

| Input | Interpretation | Behavior |
|-------|---------------|----------|
| *(none)* | `HEAD` | Diff HEAD against its parent (single commit) |
| `HEAD` | `HEAD` | Diff HEAD against its parent |
| `HEAD~3` | `HEAD~3` | Diff HEAD~3 against its parent |
| `abc123` | `abc123` | Diff abc123 against its parent |
| `main..feature` | `main..feature` | Partial-squash: every commit in the range |
| `HEAD~3..HEAD` | `HEAD~3..HEAD` | Partial-squash: last 3 commits |
| `v1.0.0..v2.0.0` | `v1.0.0..v2.0.0` | Partial-squash: commits between tags |
| `..feature` | `HEAD..feature` | Implicit HEAD on left side |
| `main..` | `main..HEAD` | Implicit HEAD on right side |

### Rules

1. **Contains `..`** → range mode with partial-squash semantics
2. **No `..`** → single-commit mode: diff the resolved commit against its parent (current behavior)
3. **`...` (three dots)** → error with message: "triple-dot syntax not supported, use `A..B`"
4. **Empty side** → implicit `HEAD` (e.g. `..feature` = `HEAD..feature`)

### Deferred (Post-MVP)

- Branch shorthand: `feat/foo` → `<base_branch>..feat/foo` (implicit base from config)
- Progress reporting for large ranges (e.g. "processing commit 3/50...")
- `...` (symmetric difference) support

## Partial-Squash Semantics

Range mode produces a clean component branch history by filtering out net-zero changes and preserving only effective work.

### Algorithm

```
1. Walk commits in range A..B (oldest-first, topological order)
2. Compute net-zero files: files whose cumulative change across the
   entire range is zero (added then deleted, or unchanged start-to-end)
3. For each source commit:
   a. Filter out net-zero files from its changed-file set
   b. If no effective changes remain → skip this commit entirely
4. Group remaining effective changes by component (via ComponentMatcher)
5. For each component, build incremental commits:
   a. Tree = cumulative state of all effective component files
      up to this point (built from the source commit's full tree,
      filtered to component files, excluding net-zero files)
   b. Parent = previous commit on this component branch
      (or base branch if first)
   c. Message = original source commit message
   d. Author = original source commit author
   e. Co-author trailer = user running the tool
6. Single RefTransaction for all component branch updates
```

### Net-Zero Detection

A file is **net-zero** if its state at range-start equals its state at range-end. This includes files that are identical in both trees, or absent in both trees (added then deleted within the range).

Implementation: diff the tree at commit A against the tree at commit B. The resulting set of changed paths are the **effective files**. Any file changed by an intermediate commit but NOT in this set is net-zero.

### Incremental Trees

Each commit on a component branch contains the cumulative state of all effective component files up to that point:

```
Source commits (frontend component, src/ui/**):
  c1: add foo.ts, bar.ts     (foo.ts is net-zero)
  c4: add baz.ts
  c6: modify baz.ts

Component branch commits (incremental):
  p1 (from c1): tree = {bar.ts}
  p2 (from c4): tree = {bar.ts, baz.ts}
  p3 (from c6): tree = {bar.ts, baz.ts*}
```

Each commit builds on the previous — the branch is checkable-out and coherent at every point.

### Commit Messages

Original source commit messages are preserved verbatim. Authorship is retained with a co-author trailer for the tool operator:

```
feat: initial User UI

Co-Authored-By: Tool Operator <operator@example.com>
Original-Author: Alice <alice@example.com>
```

When the source commit author IS the tool operator, no co-author trailer is added.

### Worked Example

Source range `main..feature`:

```
c1 (alice): add src/ui/foo.ts, src/ui/bar.ts   — "feat: initial User UI"
c2 (bob):   modify src/ui/foo.ts               — "fix: layout bug"
c3 (alice): delete src/ui/foo.ts               — "refactor: remove foo"
c4 (bob):   add src/ui/boo.ts, src/ui/baz.ts   — "feat: initial Admin UI"
c5 (bob):   modify src/ui/boo.ts               — "fix: layout bug"
c6 (bob):   modify src/ui/baz.ts               — "fix: layout bug"
c7 (alice): delete src/ui/boo.ts               — "refactor: remove boo"
c8 (alice): add docs/ui/bar.ts                 — "docs: initial user docs"
```

**Step 1 — Net-zero files** (diff tree at `main` vs tree at `feature`):
- `foo.ts`: absent at both endpoints → net-zero
- `boo.ts`: absent at both endpoints → net-zero
- `bar.ts`: absent at start, present at end → effective
- `baz.ts`: absent at start, present at end → effective
- `docs/ui/bar.ts`: absent at start, present at end → effective

**Step 2 — Filter commits:**

| Source | Changed files | After net-zero filter | Result |
|--------|--------------|----------------------|--------|
| c1 | foo.ts, bar.ts | bar.ts | effective |
| c2 | foo.ts | *(empty)* | skipped |
| c3 | foo.ts | *(empty)* | skipped |
| c4 | boo.ts, baz.ts | baz.ts | effective |
| c5 | boo.ts | *(empty)* | skipped |
| c6 | baz.ts | baz.ts | effective |
| c7 | boo.ts | *(empty)* | skipped |
| c8 | docs/ui/bar.ts | docs/ui/bar.ts | effective |

**Step 3 — Component branches** (frontend=`src/ui/**`, docs=`docs/**`):

Frontend branch (incremental trees):
```
p1 (alice): tree={bar.ts}           — "feat: initial User UI"
p2 (bob):   tree={bar.ts, baz.ts}   — "feat: initial Admin UI"
p3 (bob):   tree={bar.ts, baz.ts*}  — "fix: layout bug"
```

Docs branch:
```
p4 (alice): tree={docs/ui/bar.ts}   — "docs: initial user docs"
```

## Architecture

```
User input: "main..feature"
                │
                ▼
        ┌──────────────┐
        │ RefSpec::parse│
        └──────┬───────┘
               │
      ┌────────┴────────┐
      ▼                 ▼
 RefSpec::Single   RefSpec::Range
 "HEAD~3"          ("main","feature")
      │                 │
      ▼                 ▼
 plan_atomize()    plan_atomize_range()
 (current path)         │
                        ├─ walk_range() → [c1..cN]
                        ├─ compute net-zero files (diff A..B trees)
                        ├─ filter commits, group by component
                        ├─ build incremental trees per component
                        └─ single RefTransaction
```

## Deliverables

1. `RefSpec` type with `parse()` that splits on `..`
2. `walk_range()` — gix revision walk returning commits oldest-first
3. `plan_atomize_range()` — partial-squash implementation
4. Refactor `plan_atomize()` to accept `ObjectId` instead of `&str`
5. Remove `--range` from `CommitArgs`, make `source_ref` positional
6. Update `StatusArgs.commit` → `StatusArgs.source_ref` for consistency
7. Error messages for range resolution include which side failed

## Implementation Tasks

### 7.1 Add `RefSpec` type

File: `src/core/refspec.rs`

```rust
pub enum RefSpec {
    Single(String),
    Range { start: String, end: String },
}

impl RefSpec {
    pub fn parse(input: &str) -> Result<Self, String> {
        if input.contains("...") {
            return Err("triple-dot syntax not supported, use `A..B`".into());
        }
        match input.split_once("..") {
            Some((start, end)) => {
                let start = if start.is_empty() { "HEAD" } else { start };
                let end = if end.is_empty() { "HEAD" } else { end };
                Ok(RefSpec::Range {
                    start: start.to_string(),
                    end: end.to_string(),
                })
            }
            None => Ok(RefSpec::Single(input.to_string())),
        }
    }
}
```

Update `src/core/mod.rs` to add `pub mod refspec`.

### 7.2 Refactor `plan_atomize` signature

File: `src/git/atomize.rs`

Change `plan_atomize` to accept `ObjectId` instead of `&str`:

```rust
pub fn plan_atomize(
    repo: &gix::Repository,
    config: &Config,
    matcher: &ComponentMatcher,
    source_id: ObjectId,  // was: source_ref: &str
    force: bool,
) -> Result<(Vec<AtomicResult>, Vec<Effect>), Error>
```

Move `resolve_commit()` call to the caller (commit command handler). Update all call sites and tests.

### 7.3 Implement range commit walking

File: `src/git/walk.rs` (new)

```rust
/// Walk commits reachable from `end` but not from `start`, oldest-first.
pub fn walk_range(
    repo: &gix::Repository,
    start: ObjectId,
    end: ObjectId,
) -> Result<Vec<ObjectId>, GitError>
```

- Uses gix revision walking with topological ordering
- Returns commits in oldest-first order (so incremental trees accumulate correctly)
- Update `src/git/mod.rs` to add `pub mod walk`

### 7.4 Compute net-zero files

File: `src/git/walk.rs` (or `src/git/diff.rs`)

```rust
/// Diff tree at `start` against tree at `end`.
/// Returns the set of file paths that differ (effective files).
pub fn effective_files(
    repo: &gix::Repository,
    start: ObjectId,
    end: ObjectId,
) -> Result<HashSet<PathBuf>, GitError>
```

Any file changed by an intermediate commit but NOT in this set is net-zero and should be filtered.

### 7.5 Implement `plan_atomize_range`

File: `src/git/atomize.rs`

```rust
pub fn plan_atomize_range(
    repo: &gix::Repository,
    config: &Config,
    matcher: &ComponentMatcher,
    commits: &[ObjectId],
    effective: &HashSet<PathBuf>,
    force: bool,
) -> Result<(Vec<AtomicResult>, Vec<Effect>), Error>
```

Algorithm:
1. For each commit, get `changed_files()` and intersect with `effective`
2. Skip commits with no effective changes
3. Group effective files by component
4. For each component, track cumulative file set across commits
5. Build incremental partial tree from source commit's full tree, filtered to cumulative component files (excluding net-zero)
6. Chain commits: each component commit's parent is the previous component commit (or base branch)
7. Preserve original author; add co-author trailer for tool operator
8. Collect all ref edits into a single `RefTransaction`

### 7.6 Update `CommitArgs`

File: `src/cli/mod.rs`

```rust
pub struct CommitArgs {
    /// Git ref or range to split (e.g. HEAD, main..feature).
    #[arg(default_value = "HEAD")]
    pub source_ref: String,

    // Remove: range field
    // Keep: force, ci_mode, push, remote
}
```

### 7.7 Update commit command handler

File: `src/cli/commands/commit.rs`

```rust
let refspec = RefSpec::parse(&args.source_ref)?;
match refspec {
    RefSpec::Single(ref_str) => {
        let source_id = resolve_commit(&repo, &ref_str)?;
        plan_atomize(&repo, &cfg, &matcher, source_id, args.force)?
    }
    RefSpec::Range { start, end } => {
        let start_id = resolve_commit(&repo, &start)
            .map_err(|e| /* include "left side of range" context */)?;
        let end_id = resolve_commit(&repo, &end)
            .map_err(|e| /* include "right side of range" context */)?;
        let commits = walk_range(&repo, start_id, end_id)?;
        let effective = effective_files(&repo, start_id, end_id)?;
        plan_atomize_range(&repo, &cfg, &matcher, &commits, &effective, args.force)?
    }
}
```

### 7.8 Update `StatusArgs` for consistency

File: `src/cli/mod.rs`

Rename `StatusArgs.commit` → `StatusArgs.source_ref`. Status keeps single-ref semantics only — ranges don't apply to branch state inspection.

Update `src/cli/commands/status.rs` to use `args.source_ref`.

### 7.9 Ref resolution error context

When resolving refs in range mode, wrap errors to indicate which side failed:

```
error: could not resolve 'foobar' (left side of range 'foobar..HEAD')
```

### 7.10 Tests

**Unit tests:**
- `RefSpec::parse` — single refs, ranges, empty sides, triple-dot error
- `walk_range` — correct commits in topological oldest-first order
- `effective_files` — correctly identifies net-zero vs effective files
- `plan_atomize_range` — partial-squash produces correct incremental trees

**Integration tests:**
- Range mode with net-zero files: verify skipped commits and filtered files
- Range mode with multiple components: verify independent component branches
- Range mode incremental trees: verify each commit's tree is cumulative
- Single-ref behavior unchanged (regression)
- Empty range (A..A) produces no results
- Range where all changes are net-zero produces no results

**The worked example above should be an integration test.**

## Skills

- `lang-rust-dev`
- `lang-rust-library-dev`

## Dependencies

- Phase 1: Core Parsing (`resolve_commit`, `changed_files`)
- Phase 2: Branch Operations (`plan_atomize`, tree building)
- Phase 3: CLI Interface (`CommitArgs`)
- Phase 4: Effect Collection (effect merging)
- Phase 6: Git Config Layered Configuration

## Acceptance Criteria

- [ ] `git-atomic commit` splits HEAD (default, unchanged)
- [ ] `git-atomic commit HEAD~3` splits a single older commit
- [ ] `git-atomic commit main..feature` partial-squashes commits in the range
- [ ] Net-zero files are filtered; commits with no effective changes are skipped
- [ ] Component branch commits use incremental (cumulative) trees
- [ ] Original commit messages and authors are preserved
- [ ] `--range` flag removed; positional `source_ref` is the sole mechanism
- [ ] `...` produces a clear error message
- [ ] `..feature` and `main..` expand implicit HEAD
- [ ] Range ref resolution errors indicate which side failed
- [ ] Range mode produces a single atomic ref transaction
- [ ] `git-atomic commit --dry-run main..feature` previews all effects
- [ ] All existing tests pass
- [ ] New tests cover RefSpec parsing, range walking, net-zero detection, and partial-squash

## Review Gate

Before implementation:

- [ ] Plan approved
- [ ] gix revision walk API confirmed (topological ordering, exclusion)
- [ ] `effective_files` approach validated (tree diff at endpoints)
- [ ] Incremental tree construction approach validated
