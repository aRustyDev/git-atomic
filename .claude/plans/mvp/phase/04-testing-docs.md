# Phase 4: Testing & Documentation

**Status**: Not Started
**Issue**: TBD
**Parent**: [MVP Plan](../index.md)

## Scope

Comprehensive testing, documentation, and release preparation.

## Deliverables

1. Complete test suite (unit, integration, E2E)
2. README with quickstart
3. Configuration reference documentation
4. CI integration examples
5. Troubleshooting guide
6. v0.1.0 release

## Skills

- `lang-rust-dev`
- `lang-rust-docs-dev`
- `lang-rust-benchmarking-eng`
- `architecture-decision-records-dev`

## Dependencies

- Phase 1: Core Parsing
- Phase 2: Branch Operations
- Phase 3: CLI Interface

## Acceptance Criteria

- [ ] Test coverage > 80% for core modules
- [ ] All E2E scenarios pass
- [ ] README enables 5-minute quickstart
- [ ] Configuration options fully documented
- [ ] GitHub Actions example works
- [ ] Common errors have troubleshooting entries
- [ ] v0.1.0 tagged and released

## Implementation Tasks

### 4.1 Unit Tests

- [ ] Config parsing tests (valid, invalid, edge cases)
- [ ] Glob matching tests (exact, wildcard, overlap)
- [ ] Commit message generation tests
- [ ] Error type tests

### 4.2 Integration Tests

- [ ] Git repository setup/teardown fixtures
- [ ] Branch creation from main
- [ ] Branch fast-forward updates
- [ ] Diverged branch detection
- [ ] Multi-component atomization

### 4.3 End-to-End Tests

- [ ] Full workflow: commit → atomize → verify
- [ ] Dry-run accuracy (matches execution)
- [ ] JSON output completeness
- [ ] CI mode push simulation
- [ ] Error scenarios and exit codes

### 4.4 Test Fixtures

- [ ] Create test repository generator
- [ ] Multi-component structure (charts/, libs/, services/)
- [ ] Pre-existing atomic branches for update tests
- [ ] Various change scenarios (add, modify, delete)

### 4.5 Documentation: README

- [ ] Project description and value prop
- [ ] Installation instructions (cargo, binary, docker)
- [ ] Quickstart (5-minute guide)
- [ ] Basic usage examples
- [ ] Links to detailed docs

### 4.6 Documentation: mdBook

Update `docs/src/`:

- [ ] Introduction and concepts
- [ ] Installation guide
- [ ] Configuration reference (all options)
- [ ] CLI reference (all commands/flags)
- [ ] CI integration guide
- [ ] Troubleshooting guide

### 4.7 Documentation: Examples

- [ ] Basic `.atomic.toml` example
- [ ] Full configuration example
- [ ] GitHub Actions workflow example
- [ ] Multi-repo setup example

### 4.8 Release Preparation

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
| Glob matching | ✓ | | |
| Branch creation | | ✓ | |
| Branch update | | ✓ | |
| Diverged branch | | ✓ | |
| Full atomize | | | ✓ |
| Dry run | | | ✓ |
| JSON output | | | ✓ |
| CI mode | | | ✓ |
| Error exit codes | | | ✓ |

## Benchmark Acceptance Criteria

Per NFR-001 and NFR-002:

| Benchmark | Target | Measurement |
|-----------|--------|-------------|
| Single commit atomization | < 2 seconds | `cargo bench` with criterion |
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
    └── 001-local-first-design.md
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

## References

- [Requirements: Section 11](../reference/requirements.md#11-testing-strategy)
- [Requirements: Section 12](../reference/requirements.md#12-success-criteria)
- [Requirements: Section 14](../reference/requirements.md#14-milestones)
