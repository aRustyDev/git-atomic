# MVP Plan: git-atomic v0.1.0

**Status**: Planning
**Target**: v0.1.0 (MVP)
**Last Updated**: 2025-01-28

## Overview

`git-atomic` is a git subcommand that decomposes compound commits into atomic, component-specific branches. The MVP focuses on **local-first execution**.

See [reference/requirements.md](reference/requirements.md) for full requirements.

## Progress Tracker

| Phase | Status | Issue | Description |
|-------|--------|-------|-------------|
| Phase 1 | Not Started | - | Core parsing & component detection |
| Phase 2 | Not Started | - | Branch creation & commit generation |
| Phase 3 | Not Started | - | CLI interface & UX |
| Phase 4 | Not Started | - | Testing & documentation |

## Phase Plans

- [ ] [Phase 1: Core Parsing & Component Detection](phase/01-core-parsing.md)
- [ ] [Phase 2: Branch Creation & Commit Generation](phase/02-branch-operations.md)
- [ ] [Phase 3: CLI Interface & User Experience](phase/03-cli-interface.md)
- [ ] [Phase 4: Testing & Documentation](phase/04-testing-docs.md)

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                    git atomic                        │
├─────────────────────────────────────────────────────┤
│  CLI Layer (clap)                                   │
│    └── Commands: atomize, status, config            │
├─────────────────────────────────────────────────────┤
│  Core Engine                                        │
│    ├── CommitAnalyzer    - Parse commits            │
│    ├── ComponentMatcher  - Map files → components   │
│    ├── BranchManager     - Create/update branches   │
│    └── CommitGenerator   - Conventional commits     │
├─────────────────────────────────────────────────────┤
│  Git Interface (git2-rs)                            │
│    └── Repository operations                        │
└─────────────────────────────────────────────────────┘
```

## Sequence Diagram: Basic Flow

```
sequenceDiagram
    participant U as User
    participant CLI as git-atomic
    participant A as CommitAnalyzer
    participant M as ComponentMatcher
    participant B as BranchManager
    participant G as Git

    U->>CLI: git atomic
    CLI->>G: Get current branch commits
    G-->>CLI: Commits since main
    CLI->>A: Analyze commits
    A-->>CLI: File changes by commit
    CLI->>M: Match files to components
    M-->>CLI: Component groups
    loop For each component
        CLI->>B: Create/update atomic branch
        B->>G: Cherry-pick changes
        G-->>B: Success
        B-->>CLI: Branch updated
    end
    CLI-->>U: Summary + push commands
```

## Review Gates

1. **Phase Plan Review** - Before each phase becomes an issue
2. **GAP Review** - After all phases drafted, before issue creation
3. **Implementation Review** - PR review for each phase
4. **MVP Review** - Final review before v0.1.0 release

## References

- [requirements.md](reference/requirements.md) - Full requirements document
- [ROADMAP.md](ROADMAP.md) - Version timeline and milestones

## Open Questions

<!-- Track unresolved questions here -->

## GAP Review Notes

<!-- Document gaps, areas for refinement, potential extensions -->
