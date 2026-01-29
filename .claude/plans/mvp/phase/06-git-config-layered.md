# Phase 6: Git Config Layered Configuration

**Status**: Not Started
**Issue**: TBD
**Parent**: [MVP Plan](../index.md)

## Scope

Add git config as a configuration source with a well-defined priority chain. Enhance `init` to only output settings not already covered by higher-priority sources. Enhance `status` to show the resolved configuration with provenance (which source each value came from). Validate the merged configuration for cross-source consistency.

## Priority Chain

```
CLI args > ENV > .atomic.toml > git config > defaults
         (highest)                         (lowest)
```

### Rationale

- **CLI args**: Explicit per-invocation overrides. Highest priority. **Note**: `ConfigSource::Cli` is defined but unused this phase — reserved for future `--base-branch` style args.
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
5. **Updated `status` command** — Always shows resolved config with provenance
6. **Updated `validate` command** — Uses layered config for validation
7. **ADR-006** — Documents the layered configuration decision (already authored)
8. **Updated `docs/src/SUMMARY.md`** — ADR-006 link (already added)

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
    Cli,          // --base-branch, etc. (reserved — unused this phase)
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

**`Sourced<Option<String>>` semantics**: Only populated when the source explicitly provides a value. A `Sourced { value: None, source: Default }` means no source set a value and the default is "no value". If ENV sets `GIT_ATOMIC_DEFAULT_COMMIT_TYPE=feat`, the result is `Sourced { value: Some("feat"), source: Env }`.

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
              │ Validation      │  Cross-source consistency checks
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

// IMPORTANT: Custom sections use string_by(), not string().
// string() takes AsKey which is designed for gix's built-in key trees.
// For custom sections like atomic.*, use string_by():
let base: Option<String> = snapshot
    .string_by("atomic", None, "baseBranch")
    .map(|v| v.to_string());

let template: Option<String> = snapshot
    .string_by("atomic", None, "branchTemplate")
    .map(|v| v.to_string());
```

gix's config snapshot automatically merges system < global < local < worktree scopes. We treat the merged result as a single `GitConfig` source — individual git config scope distinction is not needed for the priority chain (see Post-MVP Extensions).

## Implementation Tasks

### 6.1 Create `src/config/source.rs`

- [ ] `ConfigSource` enum: `Default`, `GitConfig`, `File`, `Env`, `Cli`
- [ ] `Sourced<T>` wrapper struct
- [ ] `impl Display for ConfigSource` — human-readable labels
- [ ] `impl ConfigSource` — `fn label(&self) -> &str` for status output

### 6.2 Add `FromStr` for `UnmatchedPolicy`

- [ ] `impl FromStr for UnmatchedPolicy` — parses `"error"`, `"warn"`, `"ignore"`
- [ ] Returns descriptive error on invalid values
- [ ] Used by both ENV and git config parsing paths

### 6.3 Create `src/config/layered.rs`

- [ ] `fn load_layered_config(repo: Option<&gix::Repository>, config_path: &Path) -> Result<ResolvedConfig>`
- [ ] Start with defaults for all settings
- [ ] Layer git config values via `repo.config_snapshot()` (if repo provided)
  - Use `string_by("atomic", None, "<key>")` — **not** `string("atomic.<key>")`
- [ ] Layer `.atomic.toml` values (if file exists — not an error if missing when git config provides settings)
- [ ] Layer ENV values
- [ ] Components always loaded from `.atomic.toml` only
- [ ] Return `ResolvedConfig` with provenance on each setting
- [ ] **Error handling for missing `.atomic.toml` + no components**: Settings-only commands (`status`) succeed. Commands requiring components (`atomize`) fail with a clear message: "No components defined. Create .atomic.toml with [components] or run git-atomic init."

### 6.4 Add cross-source validation

- [ ] `fn validate_resolved(config: &ResolvedConfig) -> Result<(), Vec<ConfigWarning>>`
- [ ] Check `branch_template` contains `{component}` placeholder
- [ ] Check `base_branch` is non-empty
- [ ] Check `default_commit_type` (if set) is a valid conventional commit type
- [ ] Validation runs on the **merged** config, not individual sources
- [ ] Warnings are non-fatal but printed in human mode

### 6.5 Update `src/config/mod.rs`

- [ ] Add `pub mod layered; pub mod source;`
- [ ] Keep existing `load_config()` for backward compatibility (tests, simple paths)
- [ ] Export `ResolvedConfig`, `ConfigSource`, `Sourced`

### 6.6 Update `init` command

- [ ] Attempt to open repo via `gix::open(".")` for git config reading (non-fatal if outside a git repo)
- [ ] Call `load_layered_config()` to discover what's already set
- [ ] When generating `.atomic.toml`, omit `[settings]` keys that already have values from git config or ENV
- [ ] If a setting's value matches the default, still omit it (don't write redundant defaults)
- [ ] If all settings are already covered, only output the `[components]` section
- [ ] Human mode: print a note for each omitted setting ("base_branch already set via git config")
- [ ] JSON mode: include `"omitted"` array listing keys and their existing sources
- [ ] **Outside git repo**: Skip git config layer, proceed with ENV + defaults only

### 6.7 Update `status` command

- [ ] Always show resolved config with provenance (no `--show-config` flag needed)
- [ ] Add `--no-config` flag to suppress config output when only branch state is wanted
- [ ] Output resolved config with provenance, similar to `git config -l --show-origin`:
  ```
  Settings:
    base_branch      = main           (git config)
    branch_template  = atomic/{component}  (.atomic.toml)
    unmatched_files  = error          (default)
  ```
- [ ] JSON mode: include `"config"` object with `{ "key": { "value": ..., "source": "..." } }` structure
- [ ] If components exist, still show component branch state as before

### 6.8 Update command handlers

- [ ] `atomize.rs`: use `load_layered_config()` — extract `Config` equivalent for existing logic
- [ ] `validate.rs`: use `load_layered_config()` — validate against resolved config
- [ ] `ResolvedConfig` provides a `fn to_config(&self) -> Config` method for backward compat with `ComponentMatcher` and `BranchManager`
- [ ] Thread `repo` reference to config loading where available

### 6.9 Tests

- [ ] **Unit: layered resolution** — defaults < git config < file < ENV priority
- [ ] **Unit: git config reading** — `string_by()` reads `atomic.*` keys correctly
- [ ] **Unit: ENV override** — `GIT_ATOMIC_BASE_BRANCH` overrides file and git config
- [ ] **Unit: missing .atomic.toml** — settings resolve from other sources; components empty
- [ ] **Unit: cross-source validation** — invalid `branch_template` (missing `{component}`) produces warning
- [ ] **Unit: `FromStr` for `UnmatchedPolicy`** — valid and invalid inputs
- [ ] **Unit: `Sourced<Option<String>>`** — only populated when source provides value
- [ ] **Unit: `ResolvedConfig::to_config()`** — bridges correctly to `Config`
- [ ] **Integration: init omits covered settings** — git config sets `base_branch`, init output lacks `base_branch`
- [ ] **Integration: status shows provenance** — human and JSON modes

### 6.10 Documentation

- [x] ADR-006: Layered configuration with git config support
- [x] Update `docs/src/SUMMARY.md`

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
- [ ] Cross-source validation warns on invalid merged config
- [ ] `validate` command uses layered config

## Design Notes

### Why not components in git config?

Git config is flat key-value. Components need:
1. Ordered maps (first-match-wins per ADR-003)
2. Array values (globs)
3. Nested structure (component → {globs, commit_type, branch})

Git config can approximate this with multi-value keys (`atomic.components.frontend.globs = src/ui/**`) but ordering is unreliable and parsing is brittle. TOML is the right tool for structured component definitions.

### Why not use figment for layering?

The codebase already avoids figment for config loading (see `config/mod.rs` comment about TOML insertion order). Adding figment for layering would introduce a second config system. The layering logic is simple enough (4 sources, ~5 keys) to implement directly.

Additionally, figment has no built-in git config provider — it supports TOML, JSON, YAML, and ENV via its standard providers. A custom `Provider` implementation would be needed for git config regardless. Given the small config surface (~5 keys), direct layering is simpler than figment + custom provider.

If the config surface grows significantly in future versions (10+ keys, nested structures beyond components), migrating to figment with a custom git config provider becomes worthwhile.

### gix API: `string_by()` vs `string()`

`config_snapshot().string()` takes `impl AsKey`, designed for gix's built-in config key tree (e.g., `core.bare`, `user.name`). Custom sections like `atomic.*` are not in this tree. Use `string_by(section, subsection, key)` instead:

```rust
// Correct:
snapshot.string_by("atomic", None, "baseBranch")

// Wrong — will not find custom section keys:
snapshot.string("atomic.baseBranch")
```

### What about `git config --show-origin`?

gix's `config_snapshot()` merges all scopes into a single view. To show individual git config scope (local vs global), we'd need `config_snapshot().meta()` or scope-by-scope queries. For MVP, showing "git config" as a single source is sufficient. Scope-level detail is deferred to post-MVP.

### Backward Compatibility

- `load_config()` (existing) continues to work — used in tests and simple paths
- `load_layered_config()` is the new entry point for commands
- `ResolvedConfig::to_config()` bridges to the existing `Config` type
- No breaking changes to the `Config` struct

## Files Changed

| File | Action |
|------|--------|
| `src/config/source.rs` | **New**: `ConfigSource`, `Sourced<T>` |
| `src/config/layered.rs` | **New**: `load_layered_config()`, `ResolvedConfig`, `validate_resolved()` |
| `src/config/mod.rs` | Add modules, exports |
| `src/config/types.rs` | Add `FromStr` for `UnmatchedPolicy` |
| `src/cli/commands/init.rs` | Omit settings already covered, handle outside-repo |
| `src/cli/commands/status.rs` | Show config provenance, add `--no-config` |
| `src/cli/commands/atomize.rs` | Use layered config |
| `src/cli/commands/validate.rs` | Use layered config |
| `src/cli/output.rs` | Add config provenance printing |
| `src/cli/mod.rs` | Add `--no-config` to `StatusArgs` |

## Post-MVP Extensions

These items were identified during GAP review and deferred:

- **Per-scope git config display**: Show `(git config: local)` vs `(git config: global)` in status output. Requires scope-by-scope queries via gix.
- **`git-atomic config` subcommand**: Dedicated subcommand for reading/writing config values (similar to `git config`). Currently, `status` handles display and users set values directly via `git config` or editing `.atomic.toml`.

## GAP Review Notes

**Review Date**: 2026-01-29

### Gaps Addressed

1. **G1: gix API mismatch** — Fixed code examples to use `string_by("atomic", None, "baseBranch")` instead of `string("atomic.baseBranch")`. Added design note explaining the difference.
2. **G2: `validate` command not updated** — Added task 6.8 to update `validate.rs` to use layered config.
3. **G3: Missing `.atomic.toml` + no components error path** — Defined in task 6.3: settings-only commands succeed, component-requiring commands fail with clear message.
4. **G4: `init` needs repo for git config** — Added to task 6.6: attempt `gix::open(".")`, non-fatal if outside git repo.
5. **G5: No tests specified** — Added task 6.9 with specific unit and integration test cases.

### Areas Refined

1. **A1: `--show-config` flag** — Changed to always show config in status, with `--no-config` to suppress.
2. **A2: `init` edge cases** — Added handling for same-as-default values and outside-git-repo scenarios.
3. **A3: `ConfigSource::Cli` usage** — Documented as reserved/unused this phase in both code comment and rationale section.
4. **A4: `Sourced<Option<String>>` semantics** — Added explicit documentation of when/how this is populated.
5. **A5: `UnmatchedPolicy` `FromStr`** — Added task 6.2 for `FromStr` implementation.

### Extensions Included

- **P3: Cross-source validation** — Added as task 6.4. Validates the merged config, not individual sources.

### Extensions Deferred

- **P1: Per-scope git config display** → post-MVP
- **P2: `git-atomic config` subcommand** → post-MVP
- **P4: figment migration** → documented threshold in design notes. figment has no built-in git config provider; custom `Provider` needed regardless. Migrate when config surface exceeds ~10 keys.

## Review Gate

- [ ] All tests pass
- [ ] ADR reviewed
- [ ] `git config` values are correctly read via gix (`string_by`)
- [ ] Priority chain verified with overlapping sources
- [ ] Cross-source validation catches invalid merged config
- [ ] `init` output adapts to existing config
- [ ] `init` works outside a git repo (graceful degradation)
- [ ] `status` provenance output verified
- [ ] `validate` uses layered config
