# Phase 1: Core Parsing & Component Detection

**Status**: Not Started
**Issue**: TBD
**Parent**: [MVP Plan](../index.md)

## Scope

Foundation work: project setup, configuration loading, and commit analysis.

## Deliverables

1. Project scaffolding with CI
2. Configuration schema and loading
3. Glob pattern matching for components
4. Commit analysis (file changes extraction)

## Dependencies

- None (first phase)

## Acceptance Criteria

- [ ] `cargo build` produces working binary
- [ ] CI runs lint + test on PR
- [ ] `.atomic.toml` loads correctly with figment
- [ ] Configuration validation catches invalid globs
- [ ] Can extract file changes from a commit via gix
- [ ] Files correctly map to components via glob patterns

## Implementation Tasks

### 1.1 Project Setup

- [ ] Cargo.toml with dependencies (clap, figment, gix, thiserror, serde)
- [ ] Basic module structure (`cli/`, `config/`, `core/`, `git/`)
- [ ] GitHub Actions CI workflow (lint, test, build)
- [ ] Pre-commit hooks configuration

### 1.2 Configuration

- [ ] Define `Config` struct with serde
- [ ] Define `Component` struct (name, globs, optional overrides)
- [ ] Load from `.atomic.toml` via figment
- [ ] Support `--config` CLI override
- [ ] Validate glob patterns at load time
- [ ] Error types for config issues

### 1.3 Glob Matching

- [ ] Evaluate glob crate options (glob, globset, ignore)
- [ ] Implement `ComponentMatcher` struct
- [ ] Match file path → component name
- [ ] Handle overlapping patterns (first match wins? error?)
- [ ] Track unmatched files (DLQ)

### 1.4 Commit Analysis

- [ ] Initialize gix repository handle
- [ ] Get HEAD commit (or specified `--commit`)
- [ ] Extract tree diff (parent → commit)
- [ ] List changed files with change type (add/modify/delete)
- [ ] Group files by component using matcher

## Sequence Diagram

```
sequenceDiagram
    participant CLI
    participant Config
    participant Matcher as ComponentMatcher
    participant Git as gix

    CLI->>Config: load(".atomic.toml")
    Config-->>CLI: Config { components: [...] }
    CLI->>Matcher: new(config.components)
    Matcher-->>CLI: ComponentMatcher
    CLI->>Git: open_repository(".")
    Git-->>CLI: Repository
    CLI->>Git: get_commit(HEAD)
    Git-->>CLI: Commit { tree, parent }
    CLI->>Git: diff_trees(parent.tree, commit.tree)
    Git-->>CLI: Vec<FileDiff>
    loop For each file
        CLI->>Matcher: match_file(path)
        Matcher-->>CLI: Option<ComponentName>
    end
    CLI-->>CLI: HashMap<Component, Vec<File>>
```

## Test Cases

| Test | Description |
|------|-------------|
| `config_loads_valid_toml` | Parse valid .atomic.toml |
| `config_rejects_invalid_glob` | Detect malformed patterns |
| `matcher_exact_path` | `charts/foo/x.yaml` matches `charts/foo/**` |
| `matcher_no_match` | Unmatched files go to DLQ |
| `matcher_first_wins` | Overlapping patterns use first match |
| `git_extract_changes` | Get file list from commit |
| `git_handles_initial_commit` | No parent commit case |

## Review Gate

Before proceeding to Phase 2:

- [ ] All acceptance criteria met
- [ ] Tests pass
- [ ] Code reviewed
- [ ] Documentation for config format exists

## Open Questions

1. **Glob library choice**: globset (fast, compiled) vs glob (simple)?
2. **Overlap handling**: Error or first-match-wins?
3. **Initial commit**: How to handle commits with no parent?

## References

- [Requirements: Section 3.1](../reference/requirements.md#31-core-atomization)
- [Requirements: Section 5](../reference/requirements.md#5-configuration-schema)
- [gix documentation](https://docs.rs/gix)
- [figment documentation](https://docs.rs/figment)
