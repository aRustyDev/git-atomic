# GitHub Actions

A complete CI setup that splits commits and runs per-component pipelines.

## Split Workflow

This workflow runs on every push to `main`, splits the commit into component
branches, and pushes them.

```yaml
# .github/workflows/split.yml
name: Split Components

on:
  push:
    branches: [main]

permissions:
  contents: write

jobs:
  split:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/
          key: cargo-git-atomic-${{ runner.os }}

      - name: Install git-atomic
        run: cargo install git-atomic

      - name: Split and push
        run: git-atomic commit --ci-mode
```

## Per-Component Pipelines

After the split workflow pushes component branches, these workflows trigger
independently.

### Frontend

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

      - uses: actions/setup-node@v4
        with:
          node-version: 20

      - run: npm ci
      - run: npm test
      - run: npm run build
```

### Backend

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

      - uses: dtolnay/rust-toolchain@stable

      - run: cargo test
      - run: cargo clippy -- -D warnings
```

## PR Validation

Validate that a PR can be split cleanly before merging:

```yaml
# .github/workflows/validate-split.yml
name: Validate Split

on:
  pull_request:
    branches: [main]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install git-atomic
        run: cargo install git-atomic

      - name: Validate config
        run: git-atomic validate

      - name: Dry run split
        run: git-atomic commit --dry-run --json
```

## Tips

- **fetch-depth: 0** is required. git-atomic needs full history to create
  component branches.
- **Cache the cargo install** to avoid recompiling on every run.
- **permissions: contents: write** is needed for pushing component branches.
- Use **--json** output in validation steps to parse results programmatically.
- Consider adding a **concurrency group** to the split workflow to prevent
  parallel runs from conflicting.
