# git-atomic MVP Requirements

**Version:** 0.1.0  
**Status:** Draft  
**Last Updated:** 2025-01-28

---

## Executive Summary

`git-atomic` is a git subcommand that decomposes compound commits into atomic, component-specific branches. The MVP focuses on **local-first execution** with optional CI fallback, significantly simplifying the architecture compared to the original CI-centric design.

### Key Pivot from Original Design

| Aspect | Original (CI-centric) | MVP (Local-first) |
|--------|----------------------|-------------------|
| Primary execution | CI after merge | Developer's machine before push |
| Integration branch | Reset after atomization | Untouched |
| State persistence | GitHub artifacts + local | Local only |
| Race conditions | 3-layer defense needed | Eliminated (dev works locally) |
| Resume logic | Complex cross-run state | Simple re-run |
| PR description | Used for context | CI-only enhancement |

---

## 1. Product Definition

### 1.1 What is git-atomic?

A git subcommand (binary named `git-atomic`, invoked as `git atomic`) that:

1. Analyzes commits on the current branch to identify per-component file changes
2. Creates or updates atomic branches (`atomic/{component}`) forked from `main`
3. Generates conventional commits with appropriate changelog content
4. Provides clear feedback on what was created/updated

### 1.2 What git-atomic is NOT (MVP)

- Not a merge tool (doesn't merge atomic branches anywhere)
- Not a PR creator (doesn't interact with GitHub PRs)
- Not a CI-only tool (designed for local use, CI is fallback)
- Not a history rewriter (doesn't modify the source branch)

### 1.3 Installation

```bash
# From crates.io (future)
cargo install git-atomic

# From source
cargo install --path .

# Verify
git atomic --version
```

Git automatically discovers `git-atomic` binary in PATH and exposes it as `git atomic`.

---

## 2. User Workflows

### 2.1 Primary: Local Development

```bash
# Developer makes changes touching multiple components
git checkout -b feature/update-monitoring
vim charts/prometheus/values.yaml
vim charts/grafana/values.yaml
git add -A
git commit -m "feat: update monitoring stack dashboards"

# Atomize before pushing
git atomic

# Output:
# ✓ Analyzed commit abc1234
# ✓ Created atomic/charts-prometheus (1 file, +45/-12 lines)
# ✓ Updated atomic/charts-grafana (1 file, +23/-5 lines)
# 
# Push commands:
#   git push origin feature/update-monitoring
#   git push origin atomic/charts-prometheus atomic/charts-grafana

# Developer pushes feature branch + atomic branches
git push origin feature/update-monitoring
git push origin atomic/charts-prometheus atomic/charts-grafana
```

### 2.2 Secondary: CI Fallback

When a PR is opened without pre-atomization, CI runs `git atomic` as fallback:

```yaml
# .github/workflows/atomize.yml
name: Atomize
on:
  pull_request:
    branches: [main]

jobs:
  atomize:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      
      - name: Install git-atomic
        run: cargo install git-atomic
      
      - name: Check/Create atomic branches
        run: |
          git atomic --ci-mode
          # Pushes atomic branches if they don't exist or are outdated
```

### 2.3 Dry Run (Preview)

```bash
git atomic --dry-run

# Output:
# Dry run - no changes will be made
# 
# Would analyze: HEAD~1..HEAD (1 commit)
# 
# Component: charts-prometheus
#   Branch: atomic/charts-prometheus (would CREATE)
#   Files:
#     - charts/prometheus/values.yaml (+45/-12)
#   Commit: feat(charts-prometheus): update monitoring stack dashboards
# 
# Component: charts-grafana
#   Branch: atomic/charts-grafana (would UPDATE, fast-forward)
#   Files:
#     - charts/grafana/values.yaml (+23/-5)
#   Commit: feat(charts-grafana): update monitoring stack dashboards
```

---

## 3. MVP Functional Requirements

### 3.1 Core Atomization

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-001 | Analyze HEAD commit by default | P0 |
| FR-002 | Accept `--commit <sha>` for specific commit | P0 |
| FR-003 | Accept `--range <start>..<end>` for multiple commits | P1 |
| FR-004 | Map files to components via glob patterns | P0 |
| FR-005 | Create atomic branches from `main` | P0 |
| FR-006 | Update existing atomic branches (fast-forward) | P0 |
| FR-007 | Generate conventional commit messages | P0 |
| FR-008 | Report unmatched files (DLQ) | P0 |

### 3.2 Branch Management

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-010 | Branch naming: `atomic/{component-name}` | P0 |
| FR-011 | Detect existing atomic branches | P0 |
| FR-012 | Fast-forward when possible | P0 |
| FR-013 | Error on diverged branches (require `--force`) | P0 |
| FR-014 | Support `--force` to overwrite diverged branches | P1 |

### 3.3 Configuration

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-020 | Load config from `.atomic.toml` | P0 |
| FR-021 | Support `--config <path>` override | P0 |
| FR-022 | Define components via glob patterns | P0 |
| FR-023 | Support component-specific commit type override | P1 |
| FR-024 | Support custom branch template | P1 |

### 3.4 Output & Feedback

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-030 | Human-readable terminal output (default) | P0 |
| FR-031 | JSON output via `--json` | P0 |
| FR-032 | Dry-run mode via `--dry-run` | P0 |
| FR-033 | Verbose mode via `-v` / `--verbose` | P0 |
| FR-034 | Quiet mode via `-q` / `--quiet` | P1 |

### 3.5 CI Mode

| ID | Requirement | Priority |
|----|-------------|----------|
| FR-040 | `--ci-mode` flag for CI execution | P0 |
| FR-041 | Auto-push atomic branches in CI mode | P0 |
| FR-042 | Fetch PR description for enhanced context (CI only) | P1 |
| FR-043 | Exit code 0 if atomic branches already current | P0 |

---

## 4. MVP Non-Functional Requirements

### 4.1 Performance

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Single commit atomization | < 2 seconds |
| NFR-002 | 10-component split | < 10 seconds |
| NFR-003 | Memory usage | < 100 MB |

### 4.2 Compatibility

| ID | Requirement |
|----|-------------|
| NFR-010 | Git 2.30+ |
| NFR-011 | Linux (primary), macOS, Windows (best-effort) |
| NFR-012 | Rust stable (MSRV: 1.85+, edition 2024) |

### 4.3 Reliability

| ID | Requirement |
|----|-------------|
| NFR-020 | No partial state on failure (atomic operation) |
| NFR-021 | Clear error messages with remediation hints |
| NFR-022 | Non-destructive to source branch |

---

## 5. Configuration Schema

### 5.1 Minimal Configuration

```toml
# .atomic.toml

[components.charts-prometheus]
globs = ["charts/prometheus/**"]

[components.charts-grafana]
globs = ["charts/grafana/**"]
```

### 5.2 Full Configuration

```toml
# .atomic.toml

[settings]
# Base branch for atomic branches (default: "main")
base_branch = "main"

# Branch naming template (default: "atomic/{component}")
branch_template = "atomic/{component}"

# How to handle unmatched files: "error" | "warn" | "ignore"
unmatched_files = "error"

# Default conventional commit type (default: inferred from changes)
default_commit_type = "feat"

[components.charts-prometheus]
globs = ["charts/prometheus/**"]
# Override commit type for this component
commit_type = "fix"
# Override branch name
branch = "atomic/monitoring/prometheus"

[components.charts-grafana]
globs = ["charts/grafana/**"]

[components.shared-libs]
globs = ["libs/**", "shared/**"]
commit_type = "chore"

# Catch-all for unmatched (optional)
[components._other]
globs = ["**"]
commit_type = "chore"
```

### 5.3 Configuration Precedence

1. CLI arguments (highest)
2. Environment variables (`GIT_ATOMIC_*`)
3. Repository config (`.atomic.toml`)
4. Built-in defaults (lowest)

---

## 6. CLI Interface

### 6.1 Commands

```
git atomic [OPTIONS] [COMMAND]

Commands:
  (default)    Atomize commits (same as 'atomize')
  atomize      Atomize commits into component branches
  status       Show current atomization state
  validate     Validate configuration file
  help         Print help information

Options:
  -c, --config <PATH>    Configuration file path [default: .atomic.toml]
  -v, --verbose          Increase verbosity (-v, -vv, -vvv)
  -q, --quiet            Suppress non-error output
      --json             Output in JSON format
  -h, --help             Print help
  -V, --version          Print version
```

### 6.2 Atomize Command

```
git atomic atomize [OPTIONS]

Options:
      --commit <SHA>       Atomize specific commit [default: HEAD]
      --range <RANGE>      Atomize commit range (e.g., HEAD~3..HEAD)
      --dry-run            Preview without making changes
      --force              Overwrite diverged atomic branches
      --ci-mode            CI mode: auto-push, fetch PR context
      --push               Push atomic branches after creation
  -h, --help               Print help
```

### 6.3 Status Command

```
git atomic status [OPTIONS]

Shows:
  - Current branch and HEAD commit
  - Detected components and their files
  - Atomic branch states (exists, up-to-date, diverged, missing)

Options:
      --commit <SHA>       Check specific commit [default: HEAD]
  -h, --help               Print help
```

### 6.4 Validate Command

```
git atomic validate [OPTIONS]

Validates:
  - Configuration file syntax
  - Glob pattern validity
  - Component overlap detection
  - Branch template validity

Options:
  -c, --config <PATH>    Configuration file [default: .atomic.toml]
  -h, --help             Print help
```

---

## 7. Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success (or already up-to-date) |
| 1 | General error |
| 2 | Configuration error |
| 3 | Git operation failed |
| 4 | Unmatched files (when `unmatched_files = "error"`) |
| 5 | Diverged branches (without `--force`) |

---

## 8. Output Examples

### 8.1 Successful Atomization

```
$ git atomic
Analyzing commit abc1234: feat: update monitoring stack

Components detected:
  charts-prometheus: 2 files
  charts-grafana: 1 file

Creating atomic branches from main (def5678)...
  ✓ atomic/charts-prometheus [created]
    └─ feat(charts-prometheus): update monitoring stack
       2 files changed, 67 insertions(+), 15 deletions(-)
  
  ✓ atomic/charts-grafana [updated, fast-forward]
    └─ feat(charts-grafana): update monitoring stack
       1 file changed, 23 insertions(+), 5 deletions(-)

Done! To push atomic branches:
  git push origin atomic/charts-prometheus atomic/charts-grafana
```

### 8.2 Dry Run

```
$ git atomic --dry-run
Dry run - no changes will be made

Analyzing commit abc1234: feat: update monitoring stack

Would create/update:
  atomic/charts-prometheus [would create]
    Files: charts/prometheus/values.yaml, charts/prometheus/Chart.yaml
    Commit: feat(charts-prometheus): update monitoring stack
  
  atomic/charts-grafana [would update, fast-forward]
    Files: charts/grafana/values.yaml
    Commit: feat(charts-grafana): update monitoring stack
```

### 8.3 Unmatched Files Error

```
$ git atomic
Analyzing commit abc1234: feat: update monitoring stack

Error: 2 files do not match any component:
  - scripts/deploy.sh
  - README.md

Hint: Add glob patterns to .atomic.toml or set unmatched_files = "warn"
```

### 8.4 Diverged Branch Error

```
$ git atomic
Analyzing commit abc1234: feat: update monitoring stack

Error: atomic/charts-prometheus has diverged from expected state
  Expected base: def5678 (main)
  Actual base: 111aaa2
  
Hint: Use --force to overwrite, or manually reconcile the branch
```

### 8.5 JSON Output

```json
{
  "success": true,
  "analyzed_commits": ["abc1234"],
  "base_branch": "main",
  "base_commit": "def5678",
  "components": [
    {
      "name": "charts-prometheus",
      "branch": "atomic/charts-prometheus",
      "action": "created",
      "commit": "fedcba9",
      "files": [
        {"path": "charts/prometheus/values.yaml", "insertions": 45, "deletions": 12},
        {"path": "charts/prometheus/Chart.yaml", "insertions": 22, "deletions": 3}
      ],
      "message": "feat(charts-prometheus): update monitoring stack"
    },
    {
      "name": "charts-grafana",
      "branch": "atomic/charts-grafana", 
      "action": "updated",
      "commit": "abcdef1",
      "files": [
        {"path": "charts/grafana/values.yaml", "insertions": 23, "deletions": 5}
      ],
      "message": "feat(charts-grafana): update monitoring stack"
    }
  ],
  "unmatched_files": []
}
```

---

## 9. MVP Scope Boundaries

### 9.1 In Scope

- Single commit atomization (HEAD default)
- Commit range atomization
- Glob-based component mapping
- Atomic branch creation from `main`
- Atomic branch updates (fast-forward)
- Conventional commit generation (basic)
- DLQ reporting (error/warn/ignore modes)
- Dry-run mode
- JSON output
- CI mode with auto-push
- Configuration validation
- Status command

### 9.2 Out of Scope (Post-MVP)

- git-cliff integration (enhanced changelogs)
- Difftastic integration (AST-level diffs)
- PR description parsing (beyond CI mode fetch)
- Branch rebase strategy (only fast-forward in MVP)
- Content hash manifest (idempotency optimization)
- Resume from partial failure (just re-run)
- GitHub Actions published action
- Multi-repo support
- Merge queue integration

### 9.3 Explicitly Removed from Original Design

| Original Feature | Reason for Removal |
|-----------------|-------------------|
| Integration branch reset | Not needed - local-first doesn't modify source |
| GitHub artifact state | Not needed - local-only, re-run on failure |
| Cross-run resume | Not needed - operations are fast, just re-run |
| Race condition defense | Not needed - local execution is isolated |
| Deployment locks | Not needed - no CI-centric coordination |
| Surgical history rewriting | Not needed - source branch untouched |

---

## 10. Technical Decisions

### 10.1 Confirmed from Original Design

| Decision | Rationale |
|----------|-----------|
| Atomic branches fork from `main` | Clean history, predictable state |
| gix as primary git library | Pure Rust, no native deps |
| figment for configuration | Layered config, git config support |
| clap for CLI | Derive-based, git subcommand friendly |
| thiserror for errors | Clean error types with exit codes |
| tracing for logging | Structured logging, verbosity levels |

### 10.2 Changed from Original Design

| Original | MVP | Rationale |
|----------|-----|-----------|
| Binary: `atomic` | Binary: `git-atomic` | Git subcommand discovery |
| CI-primary | Local-primary | Simpler architecture |
| Complex state machine | Stateless operation | Re-run is cheap |
| git2 fallback | gix only (MVP) | Reduce complexity |

### 10.3 Deferred Decisions

| Decision | Defer Until |
|----------|-------------|
| ~~Workdir vs tree-manipulation for partial apply~~ | ~~Implementation spike~~ → **Resolved: gix tree builder** |
| Content hash format | Post-MVP optimization |
| Rebase strategy implementation | User feedback on fast-forward limitations |

---

## 11. Testing Strategy

### 11.1 Unit Tests

- Glob pattern matching
- Configuration parsing and validation
- Conventional commit message generation
- Component file assignment

### 11.2 Integration Tests

- Git repository setup/teardown
- Branch creation from main
- Branch fast-forward updates
- Diverged branch detection

### 11.3 End-to-End Tests

- Full atomization workflow (single commit)
- Multi-commit range processing
- Dry-run accuracy
- CI mode behavior

### 11.4 Test Repository

Create a test fixture repository with:
- Multiple components (charts/, libs/, services/)
- Various file change scenarios
- Pre-existing atomic branches (for update tests)

---

## 12. Success Criteria

### 12.1 Functional

- [ ] `git atomic` successfully atomizes a compound commit
- [ ] Atomic branches are created from `main`
- [ ] Existing branches are fast-forwarded correctly
- [ ] Unmatched files are reported accurately
- [ ] Dry-run matches actual execution
- [ ] JSON output is parseable and complete
- [ ] CI mode pushes branches successfully

### 12.2 Quality

- [ ] < 2 second execution for single commit
- [ ] Zero data loss (source branch unchanged)
- [ ] Clear error messages with actionable hints
- [ ] Works on Linux and macOS

### 12.3 Documentation

- [ ] README with quickstart
- [ ] Configuration reference
- [ ] CI integration example
- [ ] Troubleshooting guide

---

## 13. Open Questions for MVP

### OQ-MVP-001: Commit Message Strategy ✅ RESOLVED

**Decision: Option 1** — Copy source commit message, add `(component)` scope. Simplest approach, preserves developer intent.

### OQ-MVP-002: Multiple Commits to Same Component ✅ RESOLVED

**Decision: Option 1** — One atomic commit per source commit. Preserves commit granularity.

### OQ-MVP-003: Atomic Branch Divergence Definition ✅ RESOLVED

**Decision: Option 1** — Reachability check (atomic branch tip is not an ancestor of proposed new commit). Simple, correct for MVP.

---

## 14. Milestones

### M1: Foundation (Week 1-2)

- [ ] Project setup (Cargo, CI, linting)
- [ ] CLI skeleton with clap
- [ ] Configuration loading with figment
- [ ] Basic git operations with gix

### M2: Core Atomization (Week 3-4)

- [ ] Glob pattern matching
- [ ] File → component mapping
- [ ] Atomic branch creation
- [ ] Partial path application

### M3: Polish (Week 5-6)

- [ ] Branch update (fast-forward)
- [ ] Conventional commit generation
- [ ] Output formatting (human + JSON)
- [ ] Dry-run mode

### M4: CI & Release (Week 7-8)

- [ ] CI mode implementation
- [ ] Status and validate commands
- [ ] Documentation
- [ ] v0.1.0 release

---

## Appendix A: Comparison with Similar Tools

| Tool | Purpose | Difference from git-atomic |
|------|---------|---------------------------|
| git-cliff | Changelog generation | Complementary - git-atomic could use git-cliff output |
| git-absorb | Absorb staged changes into past commits | Different purpose - modifies history |
| git-branchless | Stacked diffs workflow | Different model - not component-based |
| git-split | Split commits by file | Similar but manual, not config-driven |

---

## Appendix B: Migration from CI-Centric Design

If users have existing CI-centric workflows:

1. **Keep CI as fallback**: CI still runs `git atomic --ci-mode`
2. **Encourage local adoption**: Document benefits of pre-push atomization
3. **Remove old integration reset logic**: Not needed with local-first
4. **Simplify CI workflow**: Validation + fallback only

The local-first approach is backwards-compatible with CI-centric usage but eliminates the most complex and risky operations.
