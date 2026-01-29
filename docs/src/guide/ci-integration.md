# CI Integration

git-atomic works well in continuous integration pipelines. The `--ci-mode` flag
combines splitting and pushing into a single step.

## CI Mode

```sh
git-atomic commit --ci-mode
```

This performs the standard split and then pushes each component branch to the
remote. If any operation fails, the command exits with a non-zero code.

## GitHub Actions Example

```yaml
name: Split Components

on:
  push:
    branches: [main]

jobs:
  split:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0  # full history needed for branch operations

      - name: Install git-atomic
        run: cargo install git-atomic

      - name: Split and push component branches
        run: git-atomic commit --ci-mode
```

## Triggering Per-Component Workflows

Once git-atomic pushes component branches, you can trigger separate workflows
for each component:

```yaml
# .github/workflows/frontend.yml
name: Frontend CI

on:
  push:
    branches: [atomic/frontend]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: npm ci
      - run: npm test
```

```yaml
# .github/workflows/backend.yml
name: Backend CI

on:
  push:
    branches: [atomic/backend]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo test
```

## Dry Run in CI

You can validate the split without pushing by combining `--dry-run` with
`--json` for machine-readable output:

```yaml
- name: Validate split
  run: git-atomic commit --dry-run --json
```

This is useful in pull request checks to verify that the commit can be split
cleanly before merging.

## Exit Codes in CI

git-atomic uses specific exit codes so your pipeline can react appropriately:

| Code | Meaning | CI Action |
|------|---------|-----------|
| 0 | Success | Continue |
| 1 | General error | Fail the job |
| 2 | Configuration error | Fail -- fix `.atomic.toml` |
| 3 | Git operation error | Retry or investigate |
| 4 | Unmatched files | Fail -- update component globs |
| 5 | Diverged branch | Fail -- manual resolution needed |

## Tips

- Always use `fetch-depth: 0` in checkout so git-atomic has full history.
- Cache the cargo install step to speed up builds.
- Use `--quiet` to reduce log noise in CI output.
- Use `--json` when you need to parse results programmatically.
