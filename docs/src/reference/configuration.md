# Configuration

git-atomic is configured via `.atomic.toml` in the repository root. The config
path can be overridden with `--config PATH`.

## Minimal Example

```toml
[settings]
base_branch = "main"

[[components]]
name = "frontend"
globs = ["src/ui/**"]
```

## Settings

The `[settings]` table controls global behavior.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `base_branch` | string | `"main"` | Branch that component branches are based on |
| `branch_template` | string | `"atomic/{name}"` | Template for component branch names |
| `unmatched_files` | string | `"error"` | How to handle files matching no component |
| `default_commit_type` | string | `"feat"` | Default conventional commit type |

### base_branch

The branch from which component branches are created. Component branches
contain the base branch content plus only the files belonging to that
component.

```toml
[settings]
base_branch = "main"
```

### branch_template

Controls the naming of component branches. The `{name}` placeholder is
replaced with the component name.

```toml
[settings]
branch_template = "atomic/{name}"
```

With a component named `frontend`, this produces the branch `atomic/frontend`.

### unmatched_files

Determines what happens when a changed file does not match any component's
glob patterns.

| Value | Behavior |
|-------|----------|
| `"error"` | Exit with code 4 (default) |
| `"warn"` | Print a warning, continue processing |
| `"ignore"` | Silently skip unmatched files |

```toml
[settings]
unmatched_files = "warn"
```

### default_commit_type

The conventional commit type used when creating commits on component branches,
unless overridden per-component.

```toml
[settings]
default_commit_type = "feat"
```

## Components

Components are defined as an array of tables using `[[components]]`.

| Key | Type | Required | Description |
|-----|------|----------|-------------|
| `name` | string | yes | Component identifier (used in branch names) |
| `globs` | string[] | yes | Glob patterns matching files in this component |
| `commit_type` | string | no | Override `default_commit_type` for this component |
| `branch` | string | no | Override `branch_template` for this component |

### name

A unique identifier for the component. Used in branch naming and output.

```toml
[[components]]
name = "frontend"
```

### globs

An array of glob patterns. Files matching any pattern belong to this
component. Patterns use standard glob syntax:

| Pattern | Matches |
|---------|---------|
| `src/ui/**` | All files under `src/ui/` recursively |
| `*.rs` | All `.rs` files in the root |
| `src/api/*.rs` | `.rs` files directly in `src/api/` |
| `docs/**/*.md` | All `.md` files under `docs/` recursively |

```toml
[[components]]
name = "frontend"
globs = ["src/ui/**", "src/components/**", "public/**"]
```

### Match Semantics

git-atomic uses **first-match-wins** semantics. A file is assigned to the
first component whose glob patterns match it. If a file matches multiple
components, only the first match counts. Order your components accordingly.

### commit_type

Override the default commit type for this component's commits:

```toml
[[components]]
name = "docs"
globs = ["docs/**"]
commit_type = "docs"
```

### branch

Override the branch name for this component, ignoring `branch_template`:

```toml
[[components]]
name = "frontend"
globs = ["src/ui/**"]
branch = "deploy/frontend"
```

## Environment Variable Overrides

Settings can be overridden via environment variables prefixed with
`GIT_ATOMIC_`:

| Variable | Overrides |
|----------|-----------|
| `GIT_ATOMIC_BASE_BRANCH` | `settings.base_branch` |
| `GIT_ATOMIC_BRANCH_TEMPLATE` | `settings.branch_template` |
| `GIT_ATOMIC_UNMATCHED_FILES` | `settings.unmatched_files` |
| `GIT_ATOMIC_CONFIG` | Config file path (same as `--config`) |

Environment variables take precedence over the config file.

## Git Config

Some settings can also be set via git config:

```sh
git config atomic.baseBranch develop
git config atomic.branchTemplate "split/{name}"
```

Precedence order (highest to lowest):

1. Environment variables (`GIT_ATOMIC_*`)
2. CLI flags (`--config`)
3. Git config (`atomic.*`)
4. `.atomic.toml` file
5. Built-in defaults

## Validation

Run `git-atomic validate` to check your configuration for errors:

```sh
git-atomic validate
```

This checks for:

- Valid TOML syntax
- Required fields present
- No duplicate component names
- Valid glob patterns
- No overlapping globs (warning)
