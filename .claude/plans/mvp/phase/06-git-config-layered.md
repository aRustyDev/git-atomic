# Phase 6: Git Config Layered Configuration

**Status**: Not Started
**Issue**: TBD
**Parent**: [MVP Plan](../index.md)

## Scope

Add git config as a configuration source with a well-defined priority chain. Enhance `init` to only output settings not already covered by higher-priority sources. Enhance `status` to show the resolved configuration with provenance (which source each value came from).

## Priority Chain

```
CLI args > ENV > .atomic.toml > git config > defaults
         (highest)                         (lowest)
```

### Rationale

- **CLI args**: Explicit per-invocation overrides. Highest priority.
- **ENV**: Session/CI overrides (`GIT_ATOMIC_BASE_BRANCH`, etc).
- **`.atomic.toml`**: Committed to the repo — represents the team's shared intent. Overrides git config because it's version-controlled and reviewable.
- **git config**: Per-user or per-clone overrides (local > global > system, as git handles internally). Useful for fork workflows (`git-atomic.remote = upstream`) or personal preferences that don't belong in the repo.
- **defaults**: Hardcoded fallbacks (`base_branch = "main"`, etc).

### Git Config Namespace

All settings live under the `atomic.` section in git config:

| Git Config Key | Maps To | Type |
|----------------|---------|------|
| `atomic.baseBranch` | `settings.base_branch` | string |
| `atomic.branchTemplate` | `settings.branch_template` | string |
| `atomic.unmatchedFiles` | `settings.unmatched_files` | `error`/`warn`/`ignore` |
| `atomic.defaultCommitType` | `settings.default_commit_type` | string |

Component definitions (`components.*`) are **not** supported in git config — they require ordered maps with glob arrays, which git config's flat key-value format cannot represent faithfully. Components live exclusively in `.atomic.toml`.

### ENV Namespace

| ENV Variable | Maps To |
|--------------|---------|
| `GIT_ATOMIC_BASE_BRANCH` | `settings.base_branch` |
| `GIT_ATOMIC_BRANCH_TEMPLATE` | `settings.branch_template` |
| `GIT_ATOMIC_UNMATCHED_FILES` | `settings.unmatched_files` |
| `GIT_ATOMIC_DEFAULT_COMMIT_TYPE` | `settings.default_commit_type` |

## Deliverables

1. **`src/config/layered.rs`** — Layered config resolver that merges sources by priority
2. **`src/config/source.rs`** — `ConfigSource` enum tracking provenance per value
3. **Updated `src/config/mod.rs`** — New `load_layered_config()` entry point
4. **Updated `init` command** — Only outputs settings not already set by git config or ENV
5. **Updated `status` command** — Shows resolved config with provenance
6. **ADR-006** — Documents the layered configuration decision
7. **Updated `docs/src/SUMMARY.md`** — ADR-006 link

## Skills

- `lang-rust-dev`
- `architecture-decision-records-dev`

## Dependencies

- Phase 1: Core Parsing (config types)
- Phase 3: CLI Interface (args structure)
- Phase 4: Effect Collection (init uses effects)
- Phase 5: Structured Dry-Run Output (init structured output)

## Architecture

### Resolved Config Model

```rust
/// Tracks where each setting value came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigSource {
    Default,
    GitConfig,    // gix config (system < global < local < worktree merged)
    File,         // .atomic.toml
    Env,          // GIT_ATOMIC_* environment variables
    Cli,          // --base-branch, etc. (future)
}

/// A config value with its provenance.
#[derive(Debug, Clone)]
pub struct Sourced<T> {
    pub value: T,
    pub source: ConfigSource,
}

/// Fully resolved configuration with provenance tracking.
#[derive(Debug)]
pub struct ResolvedConfig {
    pub base_branch: Sourced<String>,
    pub branch_template: Sourced<String>,
    pub unmatched_files: Sourced<UnmatchedPolicy>,
    pub default_commit_type: Sourced<Option<String>>,
    /// Components always come from .atomic.toml (no other source).
    pub components: IndexMap<String, Component>,
}
```

### Resolution Flow

```
                  ┌──────────┐
                  │ Defaults │
                  └────┬─────┘
                       │
              ┌────────▼────────┐
              │ gix config      │  repo.config_snapshot()
              │ (system/global/ │  reads atomic.baseBranch, etc.
              │  local/worktree)│
              └────────┬────────┘
                       │
              ┌────────▼────────┐
              │ .atomic.toml    │  TOML file (settings + components)
              └────────┬────────┘
                       │
              ┌────────▼────────┐
              │ ENV variables   │  GIT_ATOMIC_BASE_BRANCH, etc.
              └────────┬────────┘
                       │
              ┌────────▼────────┐
              │ CLI args        │  --base-branch (future phase)
              └────────┬────────┘
                       │
              ┌────────▼────────┐
              │ ResolvedConfig  │  Each field tagged with source
              └─────────────────┘
```

### Using gix Config API

```rust
// Already have a gix::Repository from open_repo()
let snapshot = repo.config_snapshot();

// Read typed values from the atomic.* section
let base: Option<String> = snapshot
    .string("atomic.baseBranch")
    .map(|v| v.to_string());

let template: Option<String> = snapshot
    .string("atomic.branchTemplate")
    .map(|v| v.to_string());
```

gix's config snapshot automatically merges system < global < local < worktree scopes. We treat the merged result as a single `GitConfig` source — individual git config scope distinction is not needed for the priority chain.

## Implementation Tasks

### 6.1 Create `src/config/source.rs`

- [ ] `ConfigSource` enum: `Default`, `GitConfig`, `File`, `Env`, `Cli`
- [ ] `Sourced<T>` wrapper struct
- [ ] `impl Display for ConfigSource` — human-readable labels
- [ ] `impl ConfigSource` — `fn label(&self) -> &str` for status output

### 6.2 Create `src/config/layered.rs`

- [ ] `fn load_layered_config(repo: Option<&gix::Repository>, config_path: &Path) -> Result<ResolvedConfig>`
- [ ] Start with defaults for all settings
- [ ] Layer git config values via `repo.config_snapshot()` (if repo provided)
- [ ] Layer `.atomic.toml` values (if file exists — not an error if missing when git config provides settings)
- [ ] Layer ENV values
- [ ] Components always loaded from `.atomic.toml` only
- [ ] Return `ResolvedConfig` with provenance on each setting

### 6.3 Update `src/config/mod.rs`

- [ ] Add `pub mod layered; pub mod source;`
- [ ] Keep existing `load_config()` for backward compatibility (tests, simple paths)
- [ ] Export `ResolvedConfig`, `ConfigSource`, `Sourced`

### 6.4 Update `init` command

- [ ] Call `load_layered_config()` to discover what's already set
- [ ] When generating `.atomic.toml`, omit `[settings]` keys that already have values from git config or ENV
- [ ] If all settings are already covered, only output the `[components]` section
- [ ] Human mode: print a note for each omitted setting ("base_branch already set via git config")
- [ ] JSON mode: include `"omitted"` array listing keys and their existing sources

### 6.5 Update `status` command

- [ ] Add a `--show-config` flag (or always show config when no components changed)
- [ ] Output resolved config with provenance, similar to `git config -l --show-origin`:
  ```
  Settings:
    base_branch      = main           (git config)
    branch_template  = atomic/{component}  (.atomic.toml)
    unmatched_files  = error          (default)
  ```
- [ ] JSON mode: include `"config"` object with `{ "key": { "value": ..., "source": "..." } }` structure
- [ ] If components exist, still show component branch state as before

### 6.6 Update command handlers

- [ ] `atomize.rs`: use `load_layered_config()` — extract `Config` equivalent for existing logic
- [ ] `ResolvedConfig` provides a `fn to_config(&self) -> Config` method for backward compat with `ComponentMatcher` and `BranchManager`
- [ ] Thread `repo` reference to config loading where available

### 6.7 Documentation

- [ ] ADR-006: Layered configuration with git config support
- [ ] Update `docs/src/SUMMARY.md`

## Acceptance Criteria

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes — all existing + new tests
- [ ] `git config atomic.baseBranch develop && git-atomic status` shows `base_branch = develop (git config)`
- [ ] `.atomic.toml` with `base_branch = "release"` overrides git config value
- [ ] `GIT_ATOMIC_BASE_BRANCH=staging git-atomic status` shows ENV as source
- [ ] `git-atomic init` omits settings already present in git config
- [ ] `git-atomic --json status` includes config provenance in JSON output
- [ ] Components are always loaded from `.atomic.toml` only
- [ ] Missing `.atomic.toml` is not fatal if git config provides necessary settings (settings only — components still required from file for atomize)

## Design Notes

### Why not components in git config?

Git config is flat key-value. Components need:
1. Ordered maps (first-match-wins per ADR-003)
2. Array values (globs)
3. Nested structure (component → {globs, commit_type, branch})

Git config can approximate this with multi-value keys (`atomic.components.frontend.globs = src/ui/**`) but ordering is unreliable and parsing is brittle. TOML is the right tool for structured component definitions.

### Why not use figment for layering?

The codebase already avoids figment for config loading (see `config/mod.rs` comment about TOML insertion order). Adding figment for layering would introduce a second config system. The layering logic is simple enough (4 sources, ~5 keys) to implement directly. If the config surface grows significantly in future versions, migrating to figment becomes worthwhile.

### What about `git config --show-origin`?

gix's `config_snapshot()` merges all scopes into a single view. To show individual git config scope (local vs global), we'd need `config_snapshot().meta()` or scope-by-scope queries. For MVP, showing "git config" as a single source is sufficient. Scope-level detail can be added later.

### Backward Compatibility

- `load_config()` (existing) continues to work — used in tests and simple paths
- `load_layered_config()` is the new entry point for commands
- `ResolvedConfig::to_config()` bridges to the existing `Config` type
- No breaking changes to the `Config` struct

## Files Changed

| File | Action |
|------|--------|
| `src/config/source.rs` | **New**: `ConfigSource`, `Sourced<T>` |
| `src/config/layered.rs` | **New**: `load_layered_config()`, `ResolvedConfig` |
| `src/config/mod.rs` | Add modules, exports |
| `src/cli/commands/init.rs` | Omit settings already covered |
| `src/cli/commands/status.rs` | Show config provenance |
| `src/cli/commands/atomize.rs` | Use layered config |
| `src/cli/output.rs` | Add config provenance printing |
| `src/cli/mod.rs` | Possibly add `--show-config` to StatusArgs |
| `docs/src/adr/adr-006-*.md` | **New**: ADR |
| `docs/src/SUMMARY.md` | Add ADR-006 link |

## Review Gate

- [ ] All tests pass
- [ ] ADR reviewed
- [ ] `git config` values are correctly read via gix
- [ ] Priority chain verified with overlapping sources
- [ ] `init` output adapts to existing config
- [ ] `status` provenance output verified
