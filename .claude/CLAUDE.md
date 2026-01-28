# git-atomic Project Guidelines

> git subcommand compatible tool for creating atomic commits & branches

## Quick Reference

- **Labels**: [.claude/reference/labels.md](reference/labels.md)
- **Plans**: `.claude/plans/<plan-name>/`
- **Docs**: `docs/src/` (mdBook)
- **Blog/Lessons**: `docs/blog/`

---

## Directory Structure

```
.claude/
├── CLAUDE.md              # This file
├── plans/                 # Implementation plans
│   └── <plan-name>/
│       ├── index.md       # Plan overview + progress tracker
│       ├── ROADMAP.md     # Milestones, versions, MVP timeline
│       ├── phase/         # Phase-specific plans
│       │   ├── 01-<phase>.md
│       │   └── 02-<phase>.md
│       └── reference/     # Plan-specific reference materials
├── reference/             # Global context references
│   └── labels.md          # Label definitions for issues
├── rules/                 # Project-specific rules
└── skills/                # Project-specific skills

docs/
├── book.toml              # mdBook configuration
├── src/                   # Documentation source
│   ├── SUMMARY.md         # mdBook table of contents
│   └── *.md               # Feature docs, ADRs, guides
└── blog/                  # Lessons learned (blog format)
    └── <lesson>.md        # Problem context for future agents
```

---

## Work Patterns

### Starting New Work

**Always open an issue first.**

1. Create issue describing the work
2. Assign labels from [labels.md](reference/labels.md)
3. Link to milestone/project if applicable
4. Create worktree + feature branch

### Feature Branch Pattern

```
<type>/<issue-number>-<short-description>

# Examples:
feat/42-atomic-commit-command
fix/17-parse-error-handling
docs/23-cli-usage-guide
refactor/31-config-module
```

**Types**: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci`

### Git Worktrees

All work happens in worktrees, not the main working directory.

```bash
# Create worktree for issue #42
git worktree add ../git-atomic-42 -b feat/42-atomic-commit

# Work in the worktree
cd ../git-atomic-42

# When done, remove worktree
git worktree remove ../git-atomic-42
```

See skill: `method-git-worktrees-dev`

---

## Planning Process

### Overview

```
Local Plan → Phase Plans → GAP Review → Issues → Implementation
```

### Creating Plans

Plans live in `.claude/plans/<plan-name>/`:

| File/Dir     | Purpose                                             |
| ------------ | --------------------------------------------------- |
| `index.md`   | Overarching plan, progress tracker, links to phases |
| `ROADMAP.md` | Milestone links, version bumps, MVP timeline        |
| `phase/`     | Individual phase plans                              |
| `reference/` | Supporting materials, research, diagrams            |

### Plan Requirements

1. **Use sequence diagrams** for workflows and interactions
2. **Use skills** (`method-writing-plans-dev`, `method-executing-plans-dev`)
3. **Include external review gates** - plans must have approval checkpoints
4. **Decompose into phases** - each phase gets its own plan file

### Phase Plans

Each phase plan (`phase/01-<name>.md`) should include:

- Clear scope and deliverables
- Dependencies on other phases
- Acceptance criteria
- Review gate before proceeding

### GAP Review

Before converting plans to issues:

1. Review for **G**aps - missing pieces
2. Review for **A**reas needing refinement
3. Review for **P**otential extensions/improvements

Document findings and address before creating issues.

### Plan → Issues Migration

1. Create parent issue from `index.md`
2. Create child issues from each `phase/*.md`
3. Link child issues to parent
4. Set dependency ordering via issue links
5. Update issues as work progresses
6. Update `index.md` progress tracker

### ROADMAP.md Structure

```markdown
# Roadmap

## MVP - v0.1.0

- [ ] Phase 1: Core parsing (#12)
- [ ] Phase 2: CLI interface (#13)

## v0.2.0 (Minor)

- [ ] Feature X (#14)

## v1.0.0 (Major)

- [ ] Breaking change Y (#15)

## Links

- [Project Board](https://github.com/...)
- [Milestone: MVP](https://github.com/...)
```

---

## Documentation

### When to Update `docs/src/`

- New feature implemented → add feature docs
- API/CLI changes → update usage guides
- Architecture decisions → add ADR
- Breaking changes → update migration guide

### ADR Format

Store in `docs/src/adr/`:

```markdown
# ADR-NNN: Title

## Status

Proposed | Accepted | Deprecated | Superseded

## Context

What prompted this decision?

## Decision

What was decided?

## Consequences

What are the trade-offs?
```

### Lessons Learned (`docs/blog/`)

Write blog-style posts capturing:

- **Problem context** - what happened, symptoms
- **Investigation** - what was tried, dead ends
- **Solution** - what worked
- **Reproduction steps** - enough detail for another agent to recreate

Goal: Future agents can learn from past problems.

---

## Automation

### Justfile Recipes

Prefer justfile recipes for repo automations. Keep recipes **clean and focused**.

```just
# Good: Single responsibility
test:
    cargo test

# Good: Composed from focused recipes
ci: lint test build

# Bad: Kitchen sink recipe
do-everything:
    cargo fmt && cargo clippy && cargo test && cargo build && ...
```

### Recipe Categories

| Group     | Purpose                  |
| --------- | ------------------------ |
| `dev`     | Development workflows    |
| `test`    | Testing commands         |
| `build`   | Build processes          |
| `release` | Release automation       |
| `docs`    | Documentation generation |

---

## Tracking Progress

### GitHub Integration

| Tool           | Purpose                                           |
| -------------- | ------------------------------------------------- |
| **Issues**     | Work items, bugs, features                        |
| **Milestones** | Version targets, MVP                              |
| **Projects**   | Kanban-style tracking                             |
| **Labels**     | Categorization ([labels.md](reference/labels.md)) |

### Issue Lifecycle

```
New → Explore → Research:Planned → Research:Done →
Plan:Rough → Plan:Phased → PhasePlan:Refined →
PhasePlan:Approved → PhasePlan:InProgress → Done
```

### Linking Issues

```markdown
<!-- In child issue -->

Parent: #42

<!-- In parent issue -->

Child issues:

- [ ] #43 - Phase 1
- [ ] #44 - Phase 2

<!-- Dependencies -->

Blocked by: #40
Blocks: #45
```

---

## Reference Links

- **Labels**: [.claude/reference/labels.md](reference/labels.md)
- **Skills**: `.claude/skills/` (see `method-*` for workflows)
- **Plans**: `.claude/plans/`

---

## CONTRIBUTING.md

The `CONTRIBUTING.md` file (at repo root) documents:

1. **Workflow patterns** - how to contribute
2. **Feature submission** - issue → plan → PR flow
3. **Required PR artifacts** - tests, docs, changelog
4. **Repo setup** - how to get started locally

Keep CLAUDE.md for agent guidance; CONTRIBUTING.md for human contributors.
