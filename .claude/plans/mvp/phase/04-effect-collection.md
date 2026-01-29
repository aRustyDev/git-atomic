# Phase 4: Effect Collection Pattern & Global Dry-Run

**Status**: Complete
**Issue**: TBD
**Parent**: [MVP Plan](../index.md)

## Scope

Replace inline mutations with an effect collection pattern so `--dry-run` works globally across all subcommands. Every side effect flows through a single `Effect` enum — commands produce effects, the executor runs or previews them.

## Deliverables

1. `src/core/effect.rs` — `Effect` enum + `PlannedRefEdit` + `execute()` function
2. Refactored `atomize()` → `plan_atomize()` returning `(Vec<AtomicResult>, Vec<Effect>)`
3. Global `--dry-run` flag on `Cli` (removed from `AtomizeArgs`)
4. `init` command returns `WriteFile` effect instead of direct `fs::write`
5. Effect preview output for human/JSON/quiet modes
6. ADR-005 documenting the decision
7. Blog post on the effect collection pattern
8. New test: `plan_atomize_returns_effects_without_mutating`

## Skills

- `lang-rust-dev`
- `lang-rust-docs-dev`
- `architecture-decision-records-dev`

## Dependencies

- Phase 1: Core Parsing (types used by effects)
- Phase 2: Branch Operations (atomize refactored)
- Phase 3: CLI Interface (global flag added)

## Architecture

```
Command logic → Vec<Effect> → execute(effects, dry_run)
                                 ├─ dry_run=false → perform mutations
                                 └─ dry_run=true  → print preview for each
```

### Transaction Preservation

`atomize()` used `repo.edit_references(edits)` for atomic all-or-nothing ref updates. The effect pattern preserves this by grouping ref updates into a single `Effect::RefTransaction { edits }` variant. The transaction boundary moves to the executor but remains atomic.

### Object Writes Stay Inline

Git tree/commit object writes remain inside `plan_atomize()`. They're immutable objects in `.git/objects/` — harmless without refs pointing to them. Only the ref transaction (which makes commits reachable) becomes an effect.

## Implementation Tasks

### 4.1 Create `src/core/effect.rs`

- [x] `Effect` enum: `RefTransaction`, `Push`, `WriteFile`
- [x] `PlannedRefEdit` struct for display-friendly ref edits
- [x] `execute()` function: run or preview effects
- [x] Update `src/core/mod.rs` to export the module

### 4.2 Refactor `src/git/atomize.rs`

- [x] Rename `atomize()` → `plan_atomize()`
- [x] Remove `dry_run` parameter
- [x] Return `(Vec<AtomicResult>, Vec<Effect>)` instead of applying refs inline
- [x] Remove inline `repo.edit_references(edits)` block
- [x] Build `Vec<PlannedRefEdit>` → `Effect::RefTransaction`

### 4.3 Move `--dry-run` to global CLI

- [x] Remove `dry_run` from `AtomizeArgs`
- [x] Add `#[arg(long, global = true)] pub dry_run: bool` to `Cli`

### 4.4 Refactor command handlers

- [x] `atomize.rs`: call `plan_atomize()` + `execute()`
- [x] `init.rs`: return `WriteFile` effect + `execute()`
- [x] `main.rs`: pass `cli.dry_run` through to each handler

### 4.5 Update output

- [x] Add `print_effect_preview()` to `Printer`
- [x] Human mode: describe what would happen
- [x] JSON mode: structured effect description
- [x] Quiet mode: no output

### 4.6 Update tests

- [x] Existing atomize tests call `plan_atomize()` then `execute()` separately
- [x] New test: `plan_atomize_returns_effects_without_mutating`

### 4.7 Documentation

- [x] ADR-005: Effect collection for side effects
- [x] Blog post: Effect collection pattern
- [x] Update `docs/src/SUMMARY.md` with ADR-005

## Acceptance Criteria

- [x] `cargo build` succeeds
- [x] `cargo test` — all existing + new tests pass
- [x] `--dry-run` is a global flag accepted by all subcommands
- [x] `--dry-run atomize` previews without mutations
- [x] `--dry-run init` prints "would create .atomic.toml" without writing
- [x] `--dry-run status` works normally (read-only, no effects)
- [x] `--dry-run validate` works normally (read-only, no effects)
- [x] `--json --dry-run atomize` JSON includes effect descriptions
- [x] Atomize tests create branches via `execute(effects, false)`
- [x] New test verifies `plan_atomize` returns effects without mutating refs

## Files Changed

| File | Action |
|------|--------|
| `src/core/mod.rs` | Add `pub mod effect` |
| `src/core/effect.rs` | **New**: `Effect` enum + `execute()` |
| `src/git/atomize.rs` | Rename fn, return effects instead of applying |
| `src/cli/mod.rs` | Move `--dry-run` to global `Cli` |
| `src/cli/commands/atomize.rs` | Use `plan_atomize` + `execute` |
| `src/cli/commands/init.rs` | Return `WriteFile` effect |
| `src/cli/output.rs` | Add effect preview methods |
| `src/main.rs` | Thread `cli.dry_run` to handlers |
| `docs/src/adr/adr-005-*.md` | **New**: ADR |
| `docs/src/SUMMARY.md` | Add ADR-005 link |
| `docs/blog/001-*.md` | **New**: Blog post |

## Review Gate

- [x] All tests pass
- [x] ADR reviewed
- [x] Effect enum covers all current mutations
