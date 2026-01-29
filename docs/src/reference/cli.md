# CLI Reference

## Synopsis

```
git-atomic [OPTIONS] <COMMAND>
git atomic [OPTIONS] <COMMAND>
```

## Global Options

These options apply to all commands.

| Option | Description |
|--------|-------------|
| `--dry-run` | Preview operations without making changes |
| `--json` | Output in machine-readable JSON |
| `--quiet` | Suppress non-error output |
| `-v`, `--verbose` | Increase verbosity (use `-vv` for debug) |
| `--config <PATH>` | Path to config file (default: `.atomic.toml`) |
| `-h`, `--help` | Print help |
| `-V`, `--version` | Print version |

## Commands

### commit

Split commit(s) into per-component branches.

```
git-atomic commit [OPTIONS] [REF]
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `[REF]` | Commit or range to split (default: `HEAD`) |

**Options:**

| Option | Description |
|--------|-------------|
| `--ci-mode` | Split and push in one step |

**REF formats:**

| Format | Meaning |
|--------|---------|
| (empty) | Split HEAD |
| `abc123` | Split the specified commit |
| `main..feature` | Split the range with partial-squash semantics |

**Examples:**

```sh
# Split the latest commit
git-atomic commit

# Split a specific commit
git-atomic commit abc123

# Split a range
git-atomic commit main..feature

# Preview without changes
git-atomic commit --dry-run

# Split and push (CI)
git-atomic commit --ci-mode
```

### status

Show the state of component branches.

```
git-atomic status [OPTIONS]
```

Displays each component branch and whether it exists, is up-to-date, or has
diverged from the base branch.

**Examples:**

```sh
git-atomic status
git-atomic status --json
```

### validate

Check the configuration file for errors.

```
git-atomic validate [OPTIONS]
```

Validates TOML syntax, required fields, glob patterns, and duplicate names.
Exits with code 2 if the configuration is invalid.

**Examples:**

```sh
git-atomic validate
git-atomic validate --config custom.toml
```

### init

Generate a starter `.atomic.toml` configuration file.

```
git-atomic init [OPTIONS]
```

Creates a `.atomic.toml` in the current directory with example components.
Refuses to overwrite an existing file unless `--force` is specified.

**Options:**

| Option | Description |
|--------|-------------|
| `--force` | Overwrite existing `.atomic.toml` |

**Examples:**

```sh
git-atomic init
git-atomic init --force
```
