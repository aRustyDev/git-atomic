# Quickstart

This guide walks you through splitting your first commit in about 5 minutes.

## 1. Initialize a Test Repository

```sh
mkdir demo && cd demo
git init
git commit --allow-empty -m "initial commit"
```

## 2. Create the Configuration

Create `.atomic.toml` in the repository root:

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

This tells git-atomic that files under `src/ui/` and `src/components/` belong
to the `frontend` component, and files under `src/api/` and `src/db/` belong
to `backend`.

Alternatively, generate a starter config:

```sh
git-atomic init
```

## 3. Create Files Across Components

```sh
mkdir -p src/ui src/api

echo '<h1>Hello</h1>' > src/ui/index.html
echo 'fn main() {}' > src/api/main.rs

git add .
git commit -m "add frontend and backend files"
```

This commit touches both components.

## 4. Split the Commit

```sh
git-atomic commit
```

git-atomic reads HEAD, classifies each file, and creates per-component
branches.

## 5. Inspect the Results

```sh
git branch
```

You should see:

```
  atomic/backend
  atomic/frontend
* main
```

Check what each branch contains:

```sh
git log --oneline atomic/frontend
git diff main..atomic/frontend --stat

git log --oneline atomic/backend
git diff main..atomic/backend --stat
```

The `atomic/frontend` branch contains only `src/ui/index.html`, and
`atomic/backend` contains only `src/api/main.rs`.

## 6. Preview Before Splitting

Use `--dry-run` to see what git-atomic would do without making changes:

```sh
git-atomic commit --dry-run
```

This prints the plan (which branches would be created, which files go where)
without modifying the repository.

## 7. Check Status

```sh
git-atomic status
```

This shows the state of all component branches: whether they exist, are
up-to-date, or have diverged.

## Next Steps

- [Configuration Reference](../reference/configuration.md) -- customize branch
  names, handle unmatched files, and more.
- [CI Integration](./ci-integration.md) -- automate splitting in your pipeline.
