# Phase 3: Examples & Templates

## Scope

Create example justfiles and lookup tables that Claude can reference when authoring.

## Deliverables

### 1. `examples/rust-project.just` (~60 lines)

Complete justfile for a Rust CLI/lib project:

```just
set shell := ["bash", "-cu"]

default:
    @just --list

[group('dev')]
build:
    cargo build

[group('test')]
test:
    cargo test

# ... full template with all standard groups
```

### 2. `examples/monorepo-root.just` (~40 lines)

Root router for monorepo with module delegation.

### 3. `examples/polyglot.just` (~50 lines)

Multi-language project with modules per language.

### 4. `examples/arustydev.just` (~50 lines)

aRustyDev-specific conventions:
- CDN module imports
- `set shell := ["bash", "-cu"]`
- Template application recipes
- KuzuDB integration recipes (if applicable)

### 5. `tables/standard-groups.md` (~40 lines)

| Group | Purpose | Typical Recipes |
|-------|---------|-----------------|
| `dev` | Development | build, setup, install, watch |
| `test` | Testing | test, coverage, bench, test-watch |
| `lint` | Code quality | fmt, lint, clippy, check |
| `docs` | Documentation | docs-build, docs-serve |
| `docker` | Containers | docker-build, docker-run |
| `release` | Publishing | release, version-bump |
| `util` | Maintenance | clean, update, doctor |

### 6. `tables/language-recipes.md` (~80 lines)

Matrix of language × recipe with exact commands:

| Recipe | Rust | Go | TypeScript | Python |
|--------|------|----|------------|--------|
| `fmt` | `cargo fmt` | `gofmt -w .` | `prettier --write .` | `ruff format .` |
| `lint` | `cargo clippy` | `golangci-lint run` | `eslint .` | `ruff check .` |
| `test` | `cargo test` | `go test ./...` | `vitest` | `pytest` |
| `build` | `cargo build` | `go build ./...` | `tsc` | `python -m build` |
| `coverage` | `cargo tarpaulin` | `go test -cover` | `vitest --coverage` | `pytest --cov` |

## Dependencies

- Phase 1 (examples referenced from SKILL.md)
- Phase 2 (examples complement reference files)

## Acceptance Criteria

- [ ] Example files are valid justfile syntax
- [ ] Tables are comprehensive for covered languages
- [ ] aRustyDev example follows all rules from `.claude/rules/justfile.md`
- [ ] Examples use `.just` extension (reference only; actual files are `justfile`)
- [ ] No duplicate content across examples
