# Phase 8: Testing & Documentation

**Status**: Not Started
**Issue**: TBD
**Parent**: [MVP Plan](../index.md)

## Scope

Comprehensive testing, documentation, and release preparation.

## Deliverables

1. Shared test fixture module
2. Complete test suite (unit, integration, E2E)
3. Property-based tests for RefSpec parsing
4. Snapshot tests for CLI output (human + JSON)
5. README with quickstart
6. Configuration reference documentation
7. CI integration examples
8. Troubleshooting guide
9. v0.1.0 release

## Skills

- `lang-rust-dev`
- `lang-rust-docs-dev`
- `lang-rust-benchmarking-eng`
- `architecture-decision-records-dev`

## Dependencies

- Phase 1: Core Parsing
- Phase 2: Branch Operations
- Phase 3: CLI Interface
- Phase 4: Effect Collection
- Phase 5: Structured Dry-Run Output
- Phase 6: Git Config Layered Configuration
- Phase 7: Unified Ref Argument

## Acceptance Criteria

- [ ] Test coverage > 80% for core modules
- [ ] All E2E scenarios pass
- [ ] README enables 5-minute quickstart
- [ ] Configuration options fully documented
- [ ] GitHub Actions example works
- [ ] Common errors have troubleshooting entries
- [ ] v0.1.0 tagged and released

## Implementation Tasks

### 8.1 Test Fixture Module

Extract duplicated test setup into a shared module.

- [ ] Create `tests/support/mod.rs` (or `src/test_support.rs` behind `#[cfg(test)]`)
- [ ] `init_test_repo(dir) -> Repository` — git init, config user, initial empty commit
- [ ] `commit_file(dir, path, content, message)` — write file, add, commit
- [ ] `test_config(components)` — generate Config from a list of component specs
- [ ] Migrate existing tests in `atomize.rs`, `walk.rs`, `diff.rs`, `branch.rs` to use shared helpers

### 8.2 Unit Tests

- [ ] Config parsing tests (valid, invalid, edge cases)
- [ ] Glob matching tests (exact, wildcard, overlap)
- [ ] Commit message generation tests
- [ ] Error type tests
- [ ] RefSpec parsing (single, range, empty sides, `...` error, unicode, special chars)
- [ ] Layered config (git config overrides, env var overrides, defaults)
- [ ] Config provenance tracking

### 8.3 Integration Tests

- [ ] Git repository setup/teardown via fixture module
- [ ] Branch creation from main
- [ ] Branch fast-forward updates
- [ ] Diverged branch detection
- [ ] Multi-component split (single commit)
- [ ] Range mode: partial-squash with net-zero filtering
- [ ] Range mode: incremental cumulative trees
- [ ] Range mode: empty range (A..A) produces no results
- [ ] Range mode: all changes net-zero produces no results
- [ ] Range mode: multiple components with independent branches
- [ ] Init subcommand creates `.atomic.toml`
- [ ] Init dry-run previews without writing
- [ ] Validate subcommand detects bad globs, missing fields

### 8.4 End-to-End Tests

Use `assert_cmd` to invoke the compiled binary.

- [ ] Full workflow: `git-atomic commit` → verify branches created
- [ ] `git-atomic commit main..feature` → verify partial-squash result
- [ ] Dry-run accuracy: `--dry-run` output matches actual execution
- [ ] Range dry-run: `--dry-run main..feature` previews all effects
- [ ] JSON output completeness (`--json`)
- [ ] CI mode push simulation (`--ci-mode`)
- [ ] Error scenarios and exit codes
- [ ] `git-atomic init` creates config
- [ ] `git-atomic init --dry-run` previews without writing
- [ ] `git-atomic validate` reports errors for bad config
- [ ] `git-atomic status --ref HEAD` shows component state
- [ ] Default (no subcommand) splits HEAD

### 8.5 Property-Based Tests

Use `proptest` or `quickcheck` for fuzz-style coverage.

- [ ] `RefSpec::parse` with arbitrary strings — never panics
- [ ] `RefSpec::parse` round-trip: valid inputs parse and reconstruct correctly
- [ ] Branch name generation from arbitrary component names — always produces valid ref names
- [ ] Glob pattern matching — consistent with globset semantics

### 8.6 Snapshot Tests

Use `insta` for output regression testing.

- [ ] Human-readable output for single-commit split
- [ ] Human-readable output for range split
- [ ] JSON output for single-commit split
- [ ] JSON output for range split
- [ ] Dry-run output format (human + JSON)
- [ ] Status output format
- [ ] Error message formatting
- [ ] Config provenance display

### 8.7 Documentation: README

- [ ] Project description and value prop
- [ ] Installation instructions (cargo, binary)
- [ ] Quickstart (5-minute guide)
- [ ] Basic usage examples (single commit, range)
- [ ] Links to detailed docs

### 8.8 Documentation: mdBook

Update `docs/src/`:

- [ ] Introduction and concepts
- [ ] Installation guide
- [ ] Configuration reference (all options including git config and env vars)
- [ ] CLI reference (all commands/flags including positional ref/range argument)
- [ ] CI integration guide (GitHub Actions `git-atomic commit --ci-mode` in PR workflow)
- [ ] Troubleshooting guide
- [ ] Update ADR listing (ADRs 001–005 exist)

### 8.9 Documentation: Examples

- [ ] Basic `.atomic.toml` example
- [ ] Full configuration example (all options)
- [ ] GitHub Actions workflow example (`git-atomic commit --ci-mode` with branch protection)
- [ ] Range mode example (partial-squash workflow)

### 8.10 Release Preparation

- [ ] CHANGELOG generation (git-cliff)
- [ ] Version bump in Cargo.toml
- [ ] Release workflow (GitHub Actions)
- [ ] Crates.io publishing setup
- [ ] Binary releases (cross-compile)

## Test Matrix

| Scenario | Unit | Integration | E2E |
|----------|------|-------------|-----|
| Config loading | ✓ | | |
| Invalid config | ✓ | | |
| Layered config (git config, env) | ✓ | ✓ | |
| Config provenance | ✓ | | |
| Glob matching | ✓ | | |
| RefSpec parsing | ✓ | | |
| Branch creation | | ✓ | |
| Branch update | | ✓ | |
| Diverged branch | | ✓ | |
| Single-commit split | | ✓ | ✓ |
| Range split (partial-squash) | | ✓ | ✓ |
| Net-zero file filtering | | ✓ | |
| Incremental trees | | ✓ | |
| Init subcommand | | ✓ | ✓ |
| Validate subcommand | | ✓ | ✓ |
| Status subcommand | | | ✓ |
| Dry-run (single) | | | ✓ |
| Dry-run (range) | | | ✓ |
| JSON output | | | ✓ |
| CI mode | | | ✓ |
| Error exit codes | | | ✓ |
| Default (no subcommand) | | | ✓ |

## Benchmark Acceptance Criteria

Per NFR-001 and NFR-002:

| Benchmark | Target | Measurement |
|-----------|--------|-------------|
| Single-commit split (`plan_atomize`) | < 2 seconds | `cargo bench` with criterion |
| Range split (`plan_atomize_range`, 10 commits) | < 5 seconds | `cargo bench` with criterion |
| 10-component split | < 10 seconds | `cargo bench` with criterion |
| Memory usage (peak) | < 100 MB | Profiling via `lang-rust-benchmarking-eng` |

Benchmarks must be automated and run in CI to detect regressions.

## Note on `reference/tests/`

The `reference/tests/` directory contains archived material from a prior Helm chart workflow design. It is not related to git-atomic MVP testing. See `reference/tests/NOTE.md`.

## Documentation Structure

```
docs/src/
├── SUMMARY.md
├── introduction.md
├── guide/
│   ├── installation.md
│   ├── quickstart.md
│   └── ci-integration.md
├── reference/
│   ├── configuration.md
│   ├── cli.md
│   └── exit-codes.md
├── troubleshooting.md
└── adr/
    ├── adr-001-use-local-first-design.md
    ├── adr-002-use-partial-trees-for-isolation.md
    ├── adr-003-use-gix-for-git-operations.md
    ├── adr-004-use-globset-for-pattern-matching.md
    └── adr-005-use-effect-collection-for-side-effects.md
```

## Review Gate

Before v0.1.0 release:

- [ ] All tests pass
- [ ] Documentation reviewed
- [ ] README verified with fresh clone
- [ ] CI workflow tested
- [ ] CHANGELOG accurate
- [ ] Release notes drafted

## Release Checklist

```markdown
## v0.1.0 Release Checklist

### Pre-release
- [ ] All tests pass on main
- [ ] Version bumped in Cargo.toml
- [ ] CHANGELOG updated
- [ ] Documentation complete
- [ ] README quickstart verified

### Release
- [ ] Create release tag
- [ ] GitHub release with notes
- [ ] Publish to crates.io
- [ ] Build and attach binaries
- [ ] Announce (if applicable)

### Post-release
- [ ] Verify crates.io install works
- [ ] Verify binary releases work
- [ ] Update documentation links
```

## GAP Review Notes

Reviewed after Phase 7 completion. Findings addressed:

- **G1** (task numbering): Fixed `4.x` → `8.x`
- **G2** (missing Phase 6/7 coverage): Added layered config, RefSpec, range mode, net-zero, incremental trees to test matrix and tasks
- **G3** (missing init tests): Added to integration and E2E tasks
- **G4** (missing validate tests): Added to integration and E2E tasks
- **G5** (stale terminology): Replaced "atomize"/"atomization" with "commit"/"split"
- **G6** (ADR listing): Updated documentation structure to list all ADRs 001–005
- **G7** (range dry-run): Added range dry-run to E2E tests
- **A1** (E2E approach): Specified `assert_cmd` for binary invocation
- **A2** (benchmark targets): Added range split benchmark, clarified function names
- **A3** (CI workflow): Specified `--ci-mode` with branch protection as example content
- **A4** (scope): Kept as single phase — tasks are ordered so testing/docs can ship independently of release engineering
- **P1** (property-based tests): Added as Task 8.5 using `proptest`/`quickcheck`
- **P2** (snapshot tests): Added as Task 8.6 using `insta`
- **P3** (test fixture module): Added as Task 8.1, first task to reduce duplication

## References

- [Requirements: Section 11](../reference/requirements.md#11-testing-strategy)
- [Requirements: Section 12](../reference/requirements.md#12-success-criteria)
- [Requirements: Section 14](../reference/requirements.md#14-milestones)
