# Phase 2: Reference Files

## Scope

Create progressive disclosure reference files that SKILL.md links to for detailed content.

## Deliverables

### 1. `references/syntax-quick-ref.md` (~150 lines)

Comprehensive syntax reference table covering:

| Category | Content |
|----------|---------|
| Settings | `set shell`, `set dotenv-load`, `set positional-arguments` |
| Attributes | `[group]`, `[confirm]`, `[script]`, `[working-directory]`, `[no-cd]`, `[private]`, `[no-exit-message]` |
| Parameters | Positional, default, variadic (`+`/`*`), environment vars |
| Dependencies | Static, parameterized, conditional |
| Functions | `os()`, `arch()`, `env()`, `justfile_directory()`, `invocation_directory()` |
| Conditionals | `if/else`, `os()` checks, error handling |
| Shebang | Bash, Python, Node patterns |
| Variables | Assignment, `export`, interpolation, `{{` escaping |

### 2. `references/recipe-patterns.md` (~200 lines)

Common recipe patterns organized by purpose:

| Section | Recipes |
|---------|---------|
| Quality Gates | `check`, `check-quick`, `fmt`, `lint`, `test`, `coverage` |
| Development | `dev`, `build`, `watch`, `clean`, `setup`, `install` |
| Docker | `docker-build`, `docker-run`, `docker-push` |
| Release | `release`, `version-bump`, `changelog` |
| CI | `ci`, `ci-check`, `audit`, `sbom` |
| Docs | `docs-build`, `docs-serve`, `docs-open` |

Each with language-specific implementations (Rust, Go, TS, Python).

### 3. `references/module-system.md` (~100 lines)

Module system deep dive:

- `mod` vs `import` syntax
- Working directory behavior in modules
- Optional modules with `mod?`
- Module listing and navigation
- CDN modules from `just.arusty.dev`
- Local module patterns (`just/<name>.just`)
- Monorepo orchestration patterns

### 4. `references/maturity-model.md` (~120 lines)

Adapted maturity model (advisory, not prescriptive):

| Level | Name | When | Recipes Added |
|-------|------|------|---------------|
| 0 | Baseline | Every project | default, build, test, lint, fmt, clean |
| 1 | Quality | CI/CD needed | coverage, test-watch, check-all |
| 2 | Security | Deploying | audit, sbom, doctor |
| 3 | Production | Prod systems | deploy, migrate, logs, status |
| 4 | Polyglot | Multi-language | modules, orchestration |

- Assessment script (bash one-liner per level)
- Upgrade workflow: assess → add recipes → validate
- YAGNI guidance: "Don't add levels you don't need"

## Dependencies

- Phase 1 (SKILL.md must exist to link from)

## Acceptance Criteria

- [ ] Each reference file < 200 lines
- [ ] All files linked from SKILL.md
- [ ] No overlap between reference files
- [ ] Tables preferred over prose
- [ ] Code examples are copy-pasteable
- [ ] Language-specific content organized in tables, not scattered
