# Beads Workflows & Patterns

## Session Lifecycle

### Starting a session

```bash
bd prime              # Get full workflow context (hooks auto-call this)
bd ready              # Find unblocked work
bd show <id>          # Review issue details
bd update <id> --status in_progress   # Claim it
```

### During a session

```bash
# Discovering new work while implementing
bd create "Found: edge case in parser" -p 2 -l "bug" --deps "discovered-from:<current-id>"

# Breaking down a task
bd create "Subtask A" --parent <id>
bd create "Subtask B" --parent <id>
bd dep add <subtask-b> <subtask-a>    # B depends on A

# Quick capture without interrupting flow
bd q "TODO: handle empty input case" -l "tech-debt"
```

### Closing a session

```bash
bd close <id1> <id2>   # Close completed issues (batch)
bd sync                # Sync to git
git add <files>        # Stage code changes
git commit -m "..."    # Commit code
git push               # Push everything
```

## Plan Decomposition Pattern

Converting a phase plan into beads:

```bash
# 1. Create parent issue
PARENT=$(bd create "Phase N: Title" -d "Description" -t epic -p 1 --json | jq -r '.id')

# 2. Create subtasks as children
T1=$(bd create "N.1 First task" -d "Details" --parent $PARENT --json | jq -r '.id')
T2=$(bd create "N.2 Second task" -d "Details" --parent $PARENT --json | jq -r '.id')
T3=$(bd create "N.3 Third task" -d "Details" --parent $PARENT --json | jq -r '.id')

# 3. Add dependencies
bd dep add $T2 $T1    # T2 depends on T1
bd dep add $T3 $T2    # T3 depends on T2

# 4. Verify
bd children $PARENT
bd graph $PARENT
bd ready
```

## Dependency Patterns

### Sequential chain
```bash
bd dep add B A    # A → B → C
bd dep add C B
```

### Fan-out (parallel from one)
```bash
bd dep add B A    # A → {B, C, D} (all can run after A)
bd dep add C A
bd dep add D A
```

### Fan-in (converge to one)
```bash
bd dep add D A    # {A, B, C} → D (D waits for all)
bd dep add D B
bd dep add D C
```

### Diamond
```bash
bd dep add B A    # A → {B, C} → D
bd dep add C A
bd dep add D B
bd dep add D C
```

## Quick Capture vs Full Create

| Scenario | Command |
|----------|---------|
| Thought during coding, flesh out later | `bd q "idea"` |
| Known task with details | `bd create "Title" -d "..." -p 1` |
| Bug found while working | `bd create "Bug: ..." -t bug -p 1` |
| Batch from markdown file | `bd create -f tasks.md` |

## Issue Types & When to Use

| Type | When |
|------|------|
| `task` | Default. Any unit of work. |
| `bug` | Defect or unexpected behavior |
| `feature` | New functionality |
| `chore` | Maintenance, cleanup, upgrades |
| `epic` | Parent grouping multiple tasks |

## Priority Guide

| Priority | Meaning | When |
|----------|---------|------|
| P0 | Critical | Blocking all progress, production down |
| P1 | High | Current sprint/phase, important |
| P2 | Medium | Default, normal work |
| P3 | Low | Nice to have, backlog |
| P4 | Backlog | Someday/maybe |

## Agent Anti-Patterns

| Don't | Do Instead |
|-------|-----------|
| `bd edit <id>` | `bd update <id> --description "..."` |
| Use TodoWrite/TaskCreate | Use `bd create` / `bd q` |
| Track tasks in markdown | Use beads for all task tracking |
| Forget to sync | Run `bd sync` at session end |
| Use "high"/"medium"/"low" priority | Use 0-4 or P0-P4 |
| Create issues without checking existing | `bd search` first |

## Integration with Git Worktrees

```bash
# Beads DB is at repo root — works across worktrees
# The .beads/ directory is shared via git

cd ../my-project-feature    # In a worktree
bd ready                    # Same issues visible
bd update <id> --status in_progress
# ... work ...
bd close <id>
bd sync
```

## Viewing Project Health

```bash
bd status        # Overview: open/closed/blocked counts
bd stale         # Issues not updated recently
bd blocked       # What's stuck and why
bd dep cycles    # Circular dependency check
bd graph --all   # Visual of all open work
```
