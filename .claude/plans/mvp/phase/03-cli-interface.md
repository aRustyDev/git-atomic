# Phase 3: CLI Interface & User Experience

**Status**: Not Started
**Issue**: TBD
**Parent**: [MVP Plan](../index.md)

## Scope

Complete CLI with all commands, output formatting, and user-facing features.

## Deliverables

1. Full CLI with clap (atomize, status, validate)
2. Human-readable output formatting
3. JSON output mode
4. Dry-run mode
5. Verbosity levels
6. CI mode with auto-push

## Dependencies

- Phase 1: Core Parsing (configuration)
- Phase 2: Branch Operations (atomization engine)

## Acceptance Criteria

- [ ] `git atomic` works as git subcommand
- [ ] All commands from spec implemented
- [ ] `--dry-run` shows accurate preview
- [ ] `--json` outputs valid, parseable JSON
- [ ] `-v/-vv/-vvv` controls verbosity
- [ ] `--ci-mode` pushes branches automatically
- [ ] Exit codes match specification
- [ ] Error messages include remediation hints

## Implementation Tasks

### 3.1 CLI Structure

- [ ] Define `Cli` struct with clap derive
- [ ] Implement subcommands:
  - `atomize` (default)
  - `status`
  - `validate`
- [ ] Global options: `--config`, `--verbose`, `--quiet`, `--json`
- [ ] Git subcommand compatibility (binary named `git-atomic`)

### 3.2 Atomize Command

- [ ] Options: `--commit`, `--range`, `--dry-run`, `--force`, `--ci-mode`, `--push`
- [ ] Default to HEAD commit
- [ ] Parse commit range syntax
- [ ] Wire to core atomization engine

### 3.3 Status Command

- [ ] Show current branch and HEAD
- [ ] List detected components with file counts
- [ ] Show atomic branch states (exists, current, diverged, missing)
- [ ] Support `--commit` to check specific commit

### 3.4 Validate Command

- [ ] Load and validate configuration
- [ ] Check glob pattern syntax
- [ ] Detect component overlap
- [ ] Report validation results

### 3.5 Output Formatting

- [ ] Define output structs for each command
- [ ] Human-readable formatter with colors (termcolor/owo-colors)
- [ ] JSON formatter with serde_json
- [ ] Progress indicators for long operations
- [ ] Summary statistics (files, branches, lines changed)

### 3.6 Dry-Run Mode

- [ ] Execute full analysis without mutations
- [ ] Show "would create/update" for each branch
- [ ] List files that would be included
- [ ] Preview generated commit messages

### 3.7 CI Mode

- [ ] `--ci-mode` flag behavior:
  - Auto-push atomic branches
  - Non-interactive (no prompts)
  - Exit 0 if already up-to-date
- [ ] Configure remote for push (default: origin)
- [ ] Handle push failures gracefully

### 3.8 Error Handling

- [ ] Map errors to exit codes (see spec)
- [ ] User-friendly error messages
- [ ] Include hints for common issues
- [ ] Structured errors in JSON mode

## Sequence Diagram

```
sequenceDiagram
    participant User
    participant CLI
    participant Core
    participant Output

    User->>CLI: git atomic --dry-run
    CLI->>CLI: parse_args()
    CLI->>Core: load_config()
    Core-->>CLI: Config
    CLI->>Core: analyze_commits()
    Core-->>CLI: AnalysisResult

    alt Dry Run
        CLI->>Output: format_dry_run(result)
    else Execute
        CLI->>Core: atomize(result)
        Core-->>CLI: AtomizationResult
        CLI->>Output: format_result(result)
    end

    Output-->>CLI: formatted_string
    CLI-->>User: print + exit_code
```

## Test Cases

| Test | Description |
|------|-------------|
| `cli_default_is_atomize` | `git atomic` = `git atomic atomize` |
| `cli_config_override` | `--config custom.toml` loads custom |
| `cli_dry_run_no_mutations` | Dry run doesn't change anything |
| `cli_json_valid` | JSON output parses correctly |
| `cli_verbose_levels` | -v, -vv, -vvv increase detail |
| `cli_exit_codes` | Each error type has correct code |
| `cli_ci_mode_pushes` | CI mode pushes branches |
| `cli_error_hints` | Errors include helpful hints |

## Review Gate

Before proceeding to Phase 4:

- [ ] All acceptance criteria met
- [ ] Manual testing of all commands
- [ ] Output matches specification examples
- [ ] Code reviewed

## UI/UX Examples

### Success Output

```
$ git atomic
Analyzing commit abc1234: feat: update monitoring stack

Components detected:
  charts-prometheus: 2 files
  charts-grafana: 1 file

Creating atomic branches from main (def5678)...
  ✓ atomic/charts-prometheus [created]
  ✓ atomic/charts-grafana [updated]

Done! To push atomic branches:
  git push origin atomic/charts-prometheus atomic/charts-grafana
```

### Error Output

```
$ git atomic
Error: 2 files do not match any component:
  - scripts/deploy.sh
  - README.md

Hint: Add glob patterns to .atomic.toml or set unmatched_files = "warn"
```

## References

- [Requirements: Section 6](../reference/requirements.md#6-cli-interface)
- [Requirements: Section 7](../reference/requirements.md#7-exit-codes)
- [Requirements: Section 8](../reference/requirements.md#8-output-examples)
- [clap documentation](https://docs.rs/clap)
