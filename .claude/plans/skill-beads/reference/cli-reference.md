# Beads CLI Reference

Version: 0.49.1

## Overview

`bd` (beads) is a distributed, git-backed issue tracker with first-class dependency support. Issues are stored as JSONL in `.beads/` with SQLite for local querying. Git hooks auto-sync changes.

## Initialization

```bash
bd init                    # Initialize in current directory (prefix = dir name)
bd init --prefix api       # Custom prefix (issues: api-<hash>)
bd init --stealth          # Invisible mode (gitignore, no repo tracking)
bd init --backend dolt     # Dolt backend for version-controlled DB
```

## Issue Lifecycle

### Create

```bash
bd create "Title"                               # Basic task
bd create "Title" -d "Description" -p 1         # With description, priority
bd create "Title" -t feature -l "ui,auth"        # Type + labels
bd create "Title" --parent <id>                  # Child of parent
bd create "Title" --deps "blocks:<id>"           # With dependency
bd create "Title" --acceptance "Criteria here"   # With acceptance criteria
bd create "Title" --design "Design notes"        # With design notes
bd create "Title" --due "+2w"                    # Due in 2 weeks
bd create "Title" --defer "+1w"                  # Hidden from bd ready for 1 week
bd create "Title" --ephemeral                    # Not exported to JSONL
bd q "Title"                                     # Quick capture (ID only output)
bd q "Title" -p 0 -l "critical"                  # Quick with priority/labels
```

**Types**: `task` (default), `bug`, `feature`, `chore`, `epic`, `merge-request`, `molecule`, `gate`, `agent`

**Priority**: 0-4 or P0-P4 (0=critical, 4=backlog). Default: 2. NOT "high"/"medium"/"low".

**Batch create from markdown**:
```bash
bd create -f tasks.md      # Create multiple from markdown file
```

### Read

```bash
bd show <id>                          # Full details with dependencies
bd list                               # All issues
bd list --status open                 # Filter by status
bd list --status in_progress          # Active work
bd list --priority 0                  # Critical issues
bd list --label testing               # Filter by label
bd ready                              # Unblocked work (no blocking deps, open/in_progress)
bd blocked                            # Issues waiting on dependencies
bd search "query"                     # Text search
bd children <parent-id>               # List children of parent
bd count --status open                # Count matching issues
bd stale                              # Issues not updated recently
```

### Update

```bash
bd update <id> --status in_progress   # Claim work
bd update <id> --status open          # Release claim
bd update <id> --assignee alice       # Assign
bd update <id> --priority 0           # Escalate
bd update <id> --title "New title"    # Retitle
bd update <id> --description "..."    # Update description
bd update <id> --notes "..."          # Update notes
bd update <id> --design "..."         # Update design notes
```

**WARNING**: Do NOT use `bd edit` — it opens $EDITOR which blocks agents.

### Close

```bash
bd close <id>                         # Close single
bd close <id1> <id2> <id3>            # Close multiple (efficient)
bd close <id> --reason "explanation"  # Close with reason
bd reopen <id>                        # Reopen closed issue
bd defer <id>                         # Defer (hidden from bd ready)
bd undefer <id>                       # Undefer
```

## Dependencies

```bash
bd dep add <child> <parent>           # child depends on parent (parent blocks child)
bd dep <blocker> --blocks <blocked>   # Equivalent shorthand
bd dep remove <child> <parent>        # Remove dependency
bd dep list <id>                      # List deps for issue
bd dep tree <id>                      # Visualize dependency tree
bd dep relate <a> <b>                 # Soft bidirectional link
bd dep unrelate <a> <b>               # Remove soft link
bd dep cycles                         # Detect circular dependencies
```

**Dependency types**:
- `blocks` — Task B must complete before task A
- `related` — Soft connection, doesn't block
- `parent-child` — Epic/subtask hierarchy (via `--parent` on create)
- `discovered-from` — Auto-created when AI discovers related work

## Epics & Hierarchy

```bash
bd create "Epic title" -t epic                    # Create epic
bd create "Subtask" --parent <epic-id>            # Add child
bd children <epic-id>                             # List children
bd epic status <epic-id>                          # Completion status
bd epic close-eligible                            # Close epics where all children done
```

## Visualization

```bash
bd graph <id>              # Dependency graph for issue
bd graph <id> --compact    # Tree format (one line per issue)
bd graph --all             # All open issues grouped by component
bd status                  # Project overview with counts
```

**Graph icons**: `○` open, `◐` in_progress, `●` blocked, `✓` closed, `❄` deferred

## Labels

```bash
bd label list                          # List all labels
bd label add <id> "label1,label2"      # Add labels
bd label remove <id> "label"           # Remove label
```

## Comments

```bash
bd comments <id>                       # View comments
bd comments <id> --add "Comment text"  # Add comment
```

## Sync & Git

```bash
bd sync                    # Export DB to JSONL, commit to git
bd sync --status           # Check sync status without syncing
bd hooks install           # Install git hooks for auto-sync
```

Auto-sync behavior (enabled by default):
- CRUD operations auto-export to JSONL (5s debounce)
- Import from JSONL when newer than DB (after git pull)
- Disable with `--no-auto-flush` or `--no-auto-import`

## Molecules (Work Templates)

```bash
bd formula list                        # List available formulas
bd mol show <id>                       # Show proto/molecule structure
bd mol pour <proto-id>                 # Instantiate persistent molecule
bd mol wisp <proto-id>                 # Instantiate ephemeral molecule
bd mol progress <mol-id>              # Show molecule progress
bd mol distill <epic-id>              # Extract proto from ad-hoc epic
bd mol squash <mol-id>                # Condense to digest
bd mol burn <mol-id>                  # Discard molecule
bd cook <formula>                      # Compile formula into proto
```

## Swarms (Parallel Work)

```bash
bd swarm create <epic-id>              # Create swarm from epic
bd swarm validate <epic-id>            # Validate epic DAG for swarming
bd swarm status <swarm-id>             # Current swarm status
bd swarm list                          # List all swarms
```

## Agent Integration

```bash
bd prime                               # Output full workflow context (for LLM injection)
bd onboard                             # Minimal AGENTS.md snippet
bd setup                               # Configure editor integration
bd slot list                           # List agent slots
bd agent list                          # List agent beads
```

## Configuration

```bash
bd config list                         # Show all config
bd config set <key> <value>            # Set config value
bd config get <key>                    # Get config value
bd info                                # Database and daemon info
bd where                               # Show active .beads/ location
```

## Maintenance

```bash
bd doctor                              # Check installation health
bd repair                              # Fix corrupted database
bd resolve-conflicts                   # Resolve git merge conflicts in JSONL
bd rename-prefix <new>                 # Rename issue prefix
bd migrate                             # Database migrations
```

## Global Flags

| Flag | Description |
|------|-------------|
| `--json` | JSON output |
| `--quiet` / `-q` | Errors only |
| `--verbose` / `-v` | Debug output |
| `--dry-run` | Preview without executing (on `create`) |
| `--actor <name>` | Audit trail identity |
| `--db <path>` | Explicit database path |
| `--readonly` | Block write operations |
| `--sandbox` | Disable daemon and auto-sync |
| `--no-db` | JSONL-only mode (no SQLite) |

## Database Discovery Order

1. `--db /path/to/db.db` flag
2. `$BEADS_DB` environment variable
3. `.beads/*.db` in current directory or ancestors
4. `~/.beads/default.db` as fallback
