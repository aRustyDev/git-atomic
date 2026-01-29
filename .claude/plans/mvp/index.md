# MVP Plan: git-atomic v0.1.0

**Status**: Planning
**Target**: v0.1.0 (MVP)
**Last Updated**: 2026-01-29

## Overview

`git-atomic` is a git subcommand that decomposes compound commits into atomic, component-specific branches. The MVP focuses on **local-first execution**.

See [reference/requirements.md](reference/requirements.md) for full requirements.

## Progress Tracker

| Phase | Status | Issue | Description |
|-------|--------|-------|-------------|
| Phase 0 | Complete | - | Decisions & ADRs |
| Phase 1 | Complete | - | Core parsing & component detection |
| Phase 2 | Complete | - | Branch creation & commit generation |
| Phase 3 | Complete | - | CLI interface & UX |
| Phase 4 | Complete | - | Effect collection & global dry-run |
| Phase 5 | Complete | - | Structured dry-run output |
| Phase 6 | Not Started | - | Testing & documentation |

## Phase Plans

- [x] [Phase 0: Decisions & ADRs](phase/00-decisions.md)
- [x] [Phase 1: Core Parsing & Component Detection](phase/01-core-parsing.md)
- [x] [Phase 2: Branch Creation & Commit Generation](phase/02-branch-operations.md)
- [x] [Phase 3: CLI Interface & User Experience](phase/03-cli-interface.md)
- [x] [Phase 4: Effect Collection & Global Dry-Run](phase/04-effect-collection.md)
- [x] [Phase 5: Structured Dry-Run Output](phase/05-structured-dry-run-output.md)
- [ ] [Phase 6: Testing & Documentation](phase/06-testing-docs.md)

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
│  Git Interface (gix)                                 │
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

All open questions resolved in Phase 0. See [phase/00-decisions.md](phase/00-decisions.md).

## GAP Review Notes

**Review Date**: 2025-01-29

### Gaps Found

1. **No Phase 0 for decisions** — Open questions from requirements and phases had no resolution gate. Added Phase 0: Decisions & ADRs.
2. **Missing skill directives** — Phase plans lacked `Skills` sections to guide agent tooling. Added to all phases.
3. **ROADMAP/requirements scope conflict** — ROADMAP listed `.atomic.toml` config as post-MVP, but requirements have it as P0. Resolved: config is MVP scope.
4. **ROADMAP listed dry-run as post-MVP** — Requirements and Phase 3 include it as P0. Resolved: dry-run is MVP scope.
5. **MSRV mismatch** — Requirements said 1.75, but Cargo.toml uses `edition = "2024"` which requires 1.85+. Resolved: MSRV is 1.85+.
6. **git-cliff `link_parsers` URL** — Pointed to `orhun/git-cliff` instead of `aRustyDev/git-atomic`. Fixed in requirements and Cargo.toml.
7. **`reference/tests/` directory** — Contains archived Helm chart workflow material unrelated to git-atomic. Added NOTE.md explaining its status.
8. **Architecture diagram said git2-rs** — Requirements confirm gix. Fixed in index.md.

### Areas Refined

- All 9 open questions (3 requirements, 3 Phase 1, 3 Phase 2) resolved with concrete decisions
- 4 ADRs identified for authoring in Phase 0
- Benchmark acceptance criteria added to Phase 4 for NFR-001/002
- Color crate decision narrowed to owo-colors in Phase 3

### Potential Extensions (Not Blocking)

- Dependency additions (tracing, globset, serde_json, figment features) documented as Phase 1 implementation tasks, not plan changes
- ADR template integration via `architecture-decision-records-dev` skill
