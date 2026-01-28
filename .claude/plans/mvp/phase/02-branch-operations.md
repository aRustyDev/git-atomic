# Phase 2: Branch Creation & Commit Generation

**Status**: Not Started
**Issue**: TBD
**Parent**: [MVP Plan](../index.md)

## Scope

Core atomization: create/update atomic branches and generate conventional commits.

## Deliverables

1. Atomic branch creation from main
2. Branch update (fast-forward)
3. Partial file application to branches
4. Conventional commit message generation

## Dependencies

- Phase 1: Core Parsing (commit analysis, component matching)

## Acceptance Criteria

- [ ] `atomic/{component}` branches created from main
- [ ] Only component-relevant files included in atomic branch
- [ ] Existing branches fast-forward correctly
- [ ] Diverged branches detected and reported
- [ ] Conventional commit messages generated with scope
- [ ] Source branch remains unchanged

## Implementation Tasks

### 2.1 Branch Manager

- [ ] Define `BranchManager` struct
- [ ] Find or create `atomic/{component}` branch
- [ ] Determine base commit (main HEAD)
- [ ] Check if branch exists and its state

### 2.2 Branch State Detection

- [ ] Implement `BranchState` enum:
  - `Missing` - branch doesn't exist
  - `Current` - already up-to-date
  - `FastForward` - can be updated
  - `Diverged` - needs force or manual resolution
- [ ] Compare branch tip with expected parent
- [ ] Handle `--force` flag for diverged branches

### 2.3 Partial File Application

- [ ] Create new tree with only component files
- [ ] Cherry-pick approach: apply diff to base tree
- [ ] Handle file additions, modifications, deletions
- [ ] Preserve file modes and attributes

### 2.4 Commit Generation

- [ ] Define `CommitGenerator` struct
- [ ] Generate conventional commit message:
  - Type: `feat`, `fix`, etc. (from config or source)
  - Scope: component name
  - Description: from source commit
- [ ] Create commit object with correct parent
- [ ] Update branch reference

### 2.5 Atomic Operations

- [ ] Ensure all-or-nothing semantics
- [ ] Rollback on failure (or defer ref updates)
- [ ] No partial state left on error

## Sequence Diagram

```
sequenceDiagram
    participant Core
    participant BM as BranchManager
    participant CG as CommitGenerator
    participant Git as gix

    Core->>Git: resolve_reference("main")
    Git-->>Core: main_commit

    loop For each component
        Core->>BM: get_branch_state("atomic/{component}")
        BM->>Git: resolve_reference("atomic/{component}")
        Git-->>BM: Option<branch_commit>
        BM-->>Core: BranchState

        alt Missing
            Core->>Git: create_branch("atomic/{component}", main_commit)
        else Diverged
            Core-->>Core: Error (unless --force)
        end

        Core->>Git: create_tree(component_files)
        Git-->>Core: new_tree_oid

        Core->>CG: generate_message(source_commit, component)
        CG-->>Core: "feat(component): description"

        Core->>Git: create_commit(tree, parent, message)
        Git-->>Core: commit_oid

        Core->>Git: update_reference("atomic/{component}", commit_oid)
    end
```

## Test Cases

| Test | Description |
|------|-------------|
| `branch_create_from_main` | New atomic branch forks from main |
| `branch_fast_forward` | Existing branch updates correctly |
| `branch_diverged_error` | Diverged branch without --force errors |
| `branch_force_update` | --force overwrites diverged branch |
| `tree_partial_files` | Only component files in atomic tree |
| `commit_conventional_message` | Message follows format |
| `commit_preserves_source` | Source branch unchanged |
| `atomic_rollback` | Failure leaves no partial state |

## Review Gate

Before proceeding to Phase 3:

- [ ] All acceptance criteria met
- [ ] Integration tests with real git operations pass
- [ ] Edge cases documented (empty components, no changes)
- [ ] Code reviewed

## Open Questions

1. **Tree manipulation**: Use gix tree builder or worktree checkout?
2. **Commit author**: Preserve source author or use configured?
3. **Signature**: GPG sign atomic commits?

## References

- [Requirements: Section 3.2](../reference/requirements.md#32-branch-management)
- [Requirements: Section 10](../reference/requirements.md#10-technical-decisions)
- [gix reference documentation](https://docs.rs/gix)
