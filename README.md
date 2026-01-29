# git-atomic

Split multi-component commits into isolated per-component branches.

git-atomic reads your `.atomic.toml` configuration and creates atomic branches,
each containing only the files that belong to a single component. This enables
independent CI/CD pipelines, cleaner code review, and fine-grained deployment.

## Installation

```sh
cargo install git-atomic
```

## Quickstart

1. Create `.atomic.toml` in your repo root:

```toml
[settings]
base_branch = "main"

[[components]]
name = "frontend"
globs = ["src/ui/**", "src/components/**"]

[[components]]
name = "backend"
globs = ["src/api/**", "src/db/**"]
```

2. Make a commit that touches files across components, then split it:

```sh
git-atomic commit
```

This creates branches `atomic/frontend` and `atomic/backend`, each with only
the relevant files from your commit.

## Range Mode

Split a range of commits with partial-squash semantics:

```sh
git-atomic commit main..feature
```

Net-zero files (added then removed within the range) are automatically filtered out.

## Commands

| Command | Description |
|---------|-------------|
| `git-atomic commit [REF]` | Split commit(s) into per-component branches |
| `git-atomic status` | Show component branch states |
| `git-atomic validate` | Check configuration for errors |
| `git-atomic init` | Generate starter `.atomic.toml` |

## Global Flags

| Flag | Description |
|------|-------------|
| `--dry-run` | Preview without mutations |
| `--json` | Machine-readable JSON output |
| `--quiet` | Suppress non-error output |
| `-v` / `-vv` | Increase verbosity |
| `--config PATH` | Custom config path (default: `.atomic.toml`) |

## CI Integration

```sh
git-atomic commit --ci-mode
```

`--ci-mode` splits and pushes in one step, exiting non-zero on any failure.

## Documentation

Full documentation: [docs.arusty.dev/git-atomic](https://docs.arusty.dev/git-atomic)

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Configuration error |
| 3 | Git operation error |
| 4 | Unmatched files |
| 5 | Diverged branch |

## License

See [LICENSE](LICENSE) for details.
