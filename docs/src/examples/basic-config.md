# Basic Configuration

A minimal `.atomic.toml` for a repository with two components.

## The Configuration

```toml
[settings]
base_branch = "main"

[[components]]
name = "frontend"
globs = ["src/ui/**", "src/components/**", "public/**"]

[[components]]
name = "backend"
globs = ["src/api/**", "src/db/**", "migrations/**"]
```

## What This Does

### Settings

- `base_branch = "main"` -- Component branches are created from `main`. Each
  component branch contains the full contents of `main` plus only the files
  belonging to that component.

### Components

**frontend**: Matches any file under `src/ui/`, `src/components/`, or
`public/`, at any depth. Examples:

- `src/ui/header.tsx` -- matched
- `src/ui/styles/theme.css` -- matched
- `src/components/nav/index.tsx` -- matched
- `public/favicon.ico` -- matched
- `src/api/routes.rs` -- not matched

**backend**: Matches any file under `src/api/`, `src/db/`, or `migrations/`.

### Branch Output

When you run `git-atomic commit` on a commit that touches files in both
components, git-atomic creates:

- `atomic/frontend` -- containing only frontend file changes
- `atomic/backend` -- containing only backend file changes

If a commit only touches frontend files, only `atomic/frontend` is created or
updated.

## Extending This Config

Add more components as your repository grows:

```toml
[[components]]
name = "docs"
globs = ["docs/**", "*.md"]
commit_type = "docs"

[[components]]
name = "infra"
globs = [".github/**", "terraform/**", "docker/**"]
branch = "deploy/infra"
```

## Match Order

Components are matched in the order they appear in the file. If a file matches
multiple components, it is assigned to the first match. Place more specific
components before more general ones:

```toml
# Specific first
[[components]]
name = "api-tests"
globs = ["tests/api/**"]

# General second
[[components]]
name = "backend"
globs = ["src/api/**", "tests/**"]
```

In this example, `tests/api/test_routes.rs` matches `api-tests` (first match),
not `backend`.
