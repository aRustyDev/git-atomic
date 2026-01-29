# Phase 6: Git Config Layered Configuration

**Status**: Not Started
**Issue**: TBD
**Parent**: [MVP Plan](../index.md)

## Scope

Add git config as a configuration source with a well-defined priority chain, using **figment** for the entire config (settings + components) with a custom git config provider. Migrate `.atomic.toml` components from `[components.<name>]` map format to `[[components]]` array-of-tables format (ADR-007) so figment can own all config loading. Enhance `init` to only output settings not already covered by higher-priority sources. Enhance `status` to show the resolved configuration with provenance (which source each value came from). Validate the merged configuration for cross-source consistency.

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

Component definitions (`[[components]]`) are **not** supported in git config — they require ordered arrays with glob patterns, which git config's flat key-value format cannot represent faithfully. Components live exclusively in `.atomic.toml`.

### ENV Namespace

| ENV Variable | Maps To |
|--------------|---------|
| `GIT_ATOMIC_BASE_BRANCH` | `settings.base_branch` |
| `GIT_ATOMIC_BRANCH_TEMPLATE` | `settings.branch_template` |
| `GIT_ATOMIC_UNMATCHED_FILES` | `settings.unmatched_files` |
| `GIT_ATOMIC_DEFAULT_COMMIT_TYPE` | `settings.default_commit_type` |

## Deliverables

1. **`src/config/git_provider.rs`** — Custom figment `Provider` wrapping gix config snapshot
2. **`src/config/source.rs`** — `ConfigSource` enum tracking provenance per value
3. **`src/config/layered.rs`** — Figment-based layered config resolver + provenance extraction
4. **Updated `src/config/types.rs`** — Migrate `Config` from `IndexMap<String, Component>` to `Vec<Component>` with `name` field
5. **Updated `src/config/mod.rs`** — New `load_layered_config()` entry point, adopt figment for entire config
6. **Updated `init` command** — Only outputs settings not already set by git config or ENV; generates `[[components]]` format
7. **Updated `status` command** — Always shows resolved config with provenance
8. **Updated `validate` command** — Uses layered config, validates component name uniqueness
9. **ADR-006** — Updated for figment adoption
10. **ADR-007** — Documents TOML array-of-tables migration for components
11. **Updated `docs/src/SUMMARY.md`** — ADR-007 link

## Skills

- `lang-rust-dev`
- `architecture-decision-records-dev`

## Dependencies

- Phase 1: Core Parsing (config types)
- Phase 3: CLI Interface (args structure)
- Phase 4: Effect Collection (init uses effects)
- Phase 5: Structured Dry-Run Output (init structured output)

## Architecture

### Unified figment loading

figment is already a declared dependency (`Cargo.toml`) but was unused — it was bypassed in Phase 1 because its TOML provider doesn't guarantee `IndexMap` insertion order, which was critical for first-match-wins component matching (ADR-003).

By migrating components from `[components.<name>]` (map) to `[[components]]` (array of tables — ADR-007), ordering is guaranteed by the TOML spec itself. This eliminates the `IndexMap` dependency and allows figment to own the **entire** config — settings and components — through a single provider chain.

### `.atomic.toml` Format Change (Breaking)

Before:
```toml
[components.frontend]
globs = ["src/ui/**"]

[components.backend]
globs = ["src/api/**"]
```

After:
```toml
[[components]]
name = "frontend"
globs = ["src/ui/**"]

[[components]]
name = "backend"
globs = ["src/api/**"]
```

### Figment Provider Chain

```rust
use figment::{Figment, providers::{Serialized, Toml, Env}};

let figment = Figment::new()
    // 1. Defaults (lowest priority — settings only, no default components)
    .merge(Serialized::defaults(Settings::default()))
    // 2. Git config (custom provider — settings only)
    .merge(GitConfigProvider::new(repo)?)
    // 3. .atomic.toml (settings + components)
    .merge(Toml::file(config_path))
    // 4. ENV variables (highest non-CLI priority — settings only)
    .merge(Env::prefixed("GIT_ATOMIC_").split("_"));

let config: Config = figment.extract()?;
```

Each provider has a `Metadata` name. After extraction, `figment.find_metadata(key)` returns which provider won for that key — this powers the provenance display. Components always come from the TOML provider (no other source provides them).

### Custom Git Config Provider

```rust
use figment::{Metadata, Profile, Provider, value::{Map, Dict}};

pub struct GitConfigProvider {
    dict: Dict,
    /// Pre-built metadata for provenance tracking
    metadata: Metadata,
}

impl GitConfigProvider {
    pub fn new(repo: Option<&gix::Repository>) -> Result<Self, Error> {
        let mut dict = Dict::new();
        if let Some(repo) = repo {
            let snapshot = repo.config_snapshot();
            // Use string_by() for custom sections — not string()
            if let Some(v) = snapshot.string_by("atomic", None, "baseBranch") {
                dict.insert("base_branch".into(), v.to_string().into());
            }
            if let Some(v) = snapshot.string_by("atomic", None, "branchTemplate") {
                dict.insert("branch_template".into(), v.to_string().into());
            }
            if let Some(v) = snapshot.string_by("atomic", None, "unmatchedFiles") {
                dict.insert("unmatched_files".into(), v.to_string().into());
            }
            if let Some(v) = snapshot.string_by("atomic", None, "defaultCommitType") {
                dict.insert("default_commit_type".into(), v.to_string().into());
            }
        }
        Ok(Self {
            dict,
            metadata: Metadata::named("git config"),
        })
    }
}

impl Provider for GitConfigProvider {
    fn metadata(&self) -> Metadata {
        self.metadata.clone()
    }

    fn data(&self) -> Result<Map<Profile, Dict>, figment::Error> {
        Ok(Profile::Default.collect(self.dict.clone()))
    }
}
```

gix's `config_snapshot()` automatically merges system < global < local < worktree scopes. We treat the merged result as a single "git config" provider. Per-scope distinction is deferred to post-MVP.

### Provenance Tracking

figment tracks which provider won for each key via `Metadata`. After extraction:

```rust
/// Map figment metadata names back to our ConfigSource enum.
fn source_for_key(figment: &Figment, key: &str) -> ConfigSource {
    match figment.find_metadata(key).map(|m| m.name.as_str()) {
        Some("git config") => ConfigSource::GitConfig,
        Some(n) if n.contains(".atomic.toml") => ConfigSource::File,
        Some(n) if n.starts_with("GIT_ATOMIC_") || n.contains("env") => ConfigSource::Env,
        _ => ConfigSource::Default,
    }
}
```

This replaces the manual `Sourced<T>` layering — figment handles the merge, we query provenance after the fact.

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
    /// Components from .atomic.toml. Order preserved by TOML array-of-tables spec.
    pub components: Vec<Component>,
}
```

`ResolvedConfig` is built by extracting `Config` from figment, then querying `figment.find_metadata()` per settings field to populate `Sourced<T>` wrappers. Components come through figment's TOML provider with document-order preservation guaranteed by the TOML spec.

### Updated Config Types

```rust
/// Root configuration loaded from `.atomic.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub settings: Settings,

    /// Ordered list of components. Order determines match priority
    /// (first-match-wins per ADR-003). Order guaranteed by TOML
    /// array-of-tables spec (ADR-007).
    #[serde(default)]
    pub components: Vec<Component>,
}

/// A single component definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// Component name (was previously the map key).
    pub name: String,
    pub globs: Vec<String>,
    pub commit_type: Option<String>,
    pub branch: Option<String>,
}
```

### Resolution Flow

```
              ┌──────────────────┐
              │ Figment::new()   │
              └────────┬─────────┘
                       │
              ┌────────▼────────┐
              │ Serialized      │  Settings::default()
              │ (defaults)      │
              └────────┬────────┘
                       │ .merge()
              ┌────────▼────────┐
              │ GitConfigProvider│  custom Provider (gix snapshot)
              │ (git config)    │  settings only
              └────────┬────────┘
                       │ .merge()
              ┌────────▼────────┐
              │ Toml::file()    │  .atomic.toml
              │ (.atomic.toml)  │  settings + components
              └────────┬────────┘
                       │ .merge()
              ┌────────▼────────┐
              │ Env::prefixed() │  GIT_ATOMIC_* variables
              │                 │  settings only
              └────────┬────────┘
                       │ .extract::<Config>()
              ┌────────▼────────┐
              │ Config          │  merged settings + components
              └────────┬────────┘
                       │ find_metadata() per field
              ┌────────▼────────┐
              │ ResolvedConfig  │  each setting tagged with source
              │                 │  components always from .atomic.toml
              └─────────────────┘
```

## Implementation Tasks

### 6.1 Create `src/config/source.rs`

- [ ] `ConfigSource` enum: `Default`, `GitConfig`, `File`, `Env`, `Cli`
- [ ] `Sourced<T>` wrapper struct
- [ ] `impl Display for ConfigSource` — human-readable labels
- [ ] `impl ConfigSource` — `fn label(&self) -> &str` for status output

### 6.2 Add `FromStr` for `UnmatchedPolicy`

- [ ] `impl FromStr for UnmatchedPolicy` — parses `"error"`, `"warn"`, `"ignore"`
- [ ] Returns descriptive error on invalid values
- [ ] Used by figment's type coercion and display paths

### 6.3 Migrate `src/config/types.rs` (Breaking — ADR-007)

- [ ] Change `Config.components` from `IndexMap<String, Component>` to `Vec<Component>`
- [ ] Add `name: String` field to `Component`
- [ ] Remove `indexmap` dependency (if no longer used elsewhere)
- [ ] Update `Config::sample()` to use `Vec<Component>` with `name` field
- [ ] Update `schemars` derive if needed
- [ ] Update all code referencing `config.components` (map access → vec iteration)
  - `ComponentMatcher` — iterate `Vec` instead of `IndexMap`
  - `BranchManager` — component name from `component.name` instead of map key
  - Test fixtures — new format

### 6.4 Create `src/config/git_provider.rs`

- [ ] `GitConfigProvider` struct wrapping gix config snapshot data
- [ ] `impl figment::Provider` — `metadata()` returns `Metadata::named("git config")`
- [ ] `data()` reads `atomic.*` keys via `string_by()` and returns as `Dict`
- [ ] Constructor takes `Option<&gix::Repository>` — returns empty provider if no repo
- [ ] Key mapping: git config camelCase → Settings snake_case (`baseBranch` → `base_branch`)
- [ ] Only provides settings keys — components come from TOML provider

### 6.5 Create `src/config/layered.rs`

- [ ] `fn load_layered_config(repo: Option<&gix::Repository>, config_path: &Path) -> Result<ResolvedConfig>`
- [ ] Build figment chain: `Serialized::defaults` → `GitConfigProvider` → `Toml::file()` → `Env::prefixed("GIT_ATOMIC_")`
- [ ] Extract `Config` from figment (settings + components in one pass)
- [ ] Query `figment.find_metadata()` per settings field to determine `ConfigSource`
- [ ] Validate component name uniqueness (was free with map keys, now manual)
- [ ] Components empty if `.atomic.toml` missing (not an error for settings-only commands)
- [ ] Return `ResolvedConfig` with provenance on each setting
- [ ] **Error handling for missing `.atomic.toml` + no components**: Settings-only commands (`status`) succeed. Commands requiring components (`atomize`) fail with a clear message: "No components defined. Create .atomic.toml with [[components]] or run git-atomic init."

### 6.5 Add cross-source validation

- [ ] `fn validate_resolved(config: &ResolvedConfig) -> Result<(), Vec<ConfigWarning>>`
- [ ] Check `branch_template` contains `{component}` placeholder
- [ ] Check `base_branch` is non-empty
- [ ] Check `default_commit_type` (if set) is a valid conventional commit type
- [ ] Validation runs on the **merged** config, not individual sources
- [ ] Warnings are non-fatal but printed in human mode

### 6.7 Update `src/config/mod.rs`

- [ ] Add `pub mod layered; pub mod source; pub mod git_provider;`
- [ ] Migrate existing `load_config()` to use figment internally (or keep as thin wrapper)
- [ ] Remove comment about avoiding figment — it now owns all config loading
- [ ] Add component name uniqueness validation
- [ ] Export `ResolvedConfig`, `ConfigSource`, `Sourced`

### 6.8 Update `init` command

- [ ] Attempt to open repo via `gix::open(".")` for git config reading (non-fatal if outside a git repo)
- [ ] Call `load_layered_config()` to discover what's already set
- [ ] When generating `.atomic.toml`, omit `[settings]` keys that already have values from git config or ENV
- [ ] If a setting's value matches the default, still omit it (don't write redundant defaults)
- [ ] If all settings are already covered, only output the `[components]` section
- [ ] Human mode: print a note for each omitted setting ("base_branch already set via git config")
- [ ] JSON mode: include `"omitted"` array listing keys and their existing sources
- [ ] **Outside git repo**: Skip git config layer, proceed with ENV + defaults only

### 6.9 Update `status` command

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

### 6.10 Update command handlers

- [ ] `atomize.rs`: use `load_layered_config()` — update component iteration for `Vec<Component>`
- [ ] `validate.rs`: use `load_layered_config()` — validate against resolved config, check name uniqueness
- [ ] `ResolvedConfig` provides a `fn to_config(&self) -> Config` method for backward compat with `ComponentMatcher` and `BranchManager`
- [ ] Thread `repo` reference to config loading where available
- [ ] Update `ComponentMatcher` to accept `&[Component]` instead of `&IndexMap<String, Component>`

### 6.11 Tests

- [ ] **Unit: figment provider chain** — defaults < git config < file < ENV priority
- [ ] **Unit: GitConfigProvider** — returns correct Dict from gix snapshot
- [ ] **Unit: provenance extraction** — `find_metadata()` maps to correct `ConfigSource`
- [ ] **Unit: ENV override** — `GIT_ATOMIC_BASE_BRANCH` overrides file and git config
- [ ] **Unit: missing .atomic.toml** — settings resolve from other sources; components empty
- [ ] **Unit: cross-source validation** — invalid `branch_template` (missing `{component}`) produces warning
- [ ] **Unit: `FromStr` for `UnmatchedPolicy`** — valid and invalid inputs
- [ ] **Unit: `ResolvedConfig::to_config()`** — bridges correctly to `Config`
- [ ] **Unit: component name uniqueness** — duplicate names rejected with clear error
- [ ] **Unit: `[[components]]` order preserved** — components deserialized in document order
- [ ] **Integration: init omits covered settings** — git config sets `base_branch`, init output lacks `base_branch`
- [ ] **Integration: init generates `[[components]]` format** — output uses array-of-tables
- [ ] **Integration: status shows provenance** — human and JSON modes
- [ ] **Integration: old `[components.name]` format** — clear error message pointing to new format

### 6.12 Update ADR-006

- [ ] Update decision to reference figment-based layering with custom git config provider
- [ ] Note figment now owns entire config (settings + components)
- [ ] Reference ADR-007 for the format change enabling this

### 6.13 Documentation

- [x] ADR-006: Updated for figment adoption
- [x] ADR-007: TOML array-of-tables for components
- [x] Update `docs/src/SUMMARY.md`

## Acceptance Criteria

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes — all existing + new tests
- [ ] `git config atomic.baseBranch develop && git-atomic status` shows `base_branch = develop (git config)`
- [ ] `.atomic.toml` with `base_branch = "release"` overrides git config value
- [ ] `GIT_ATOMIC_BASE_BRANCH=staging git-atomic status` shows ENV as source
- [ ] `git-atomic init` omits settings already present in git config
- [ ] `git-atomic --json status` includes config provenance in JSON output
- [ ] Components loaded from `.atomic.toml` via figment's TOML provider in `[[components]]` format
- [ ] Component document order preserved (first-match-wins per ADR-003)
- [ ] Duplicate component names rejected with clear error
- [ ] Old `[components.name]` format produces clear migration error
- [ ] Missing `.atomic.toml` is not fatal if git config provides necessary settings (settings only — components still required from file for atomize)
- [ ] Cross-source validation warns on invalid merged config
- [ ] `validate` command uses layered config
- [ ] figment owns entire config loading (settings + components, single provider chain)

## Design Notes

### Why `[[components]]` instead of `[components.<name>]`?

See ADR-007. The original map-of-tables format (`[components.<name>]`) required `IndexMap` to preserve insertion order, which figment's TOML provider couldn't guarantee. By switching to TOML array-of-tables (`[[components]]`), ordering is guaranteed by the TOML spec itself, and figment can load the entire config through a single provider chain.

Trade-offs:
- **Gained**: Unified config loading, explicit ordering in type system (`Vec`), `name` is a visible field
- **Lost**: Automatic name deduplication (now validated manually), slightly more verbose format
- **Acceptable**: Pre-1.0 breaking change

### Why not components in git config?

Git config is flat key-value. Components need:
1. Ordered maps (first-match-wins per ADR-003)
2. Array values (globs)
3. Nested structure (component → {globs, commit_type, branch})

Git config can approximate this with multi-value keys (`atomic.components.frontend.globs = src/ui/**`) but ordering is unreliable and parsing is brittle. TOML is the right tool for structured component definitions.

### gix API: `string_by()` vs `string()`

`config_snapshot().string()` takes `impl AsKey`, designed for gix's built-in config key tree (e.g., `core.bare`, `user.name`). Custom sections like `atomic.*` are not in this tree. Use `string_by(section, subsection, key)` instead:

```rust
// Correct:
snapshot.string_by("atomic", None, "baseBranch")

// Wrong — will not find custom section keys:
snapshot.string("atomic.baseBranch")
```

### figment provenance via `find_metadata()`

figment tracks which provider won for each key. After `extract()`, call `figment.find_metadata("key")` to get the `Metadata` (which includes the provider name). Map provider names to `ConfigSource`:

- `"git config"` → `ConfigSource::GitConfig`
- Name containing `.atomic.toml` → `ConfigSource::File`
- Name containing `GIT_ATOMIC_` or `env` → `ConfigSource::Env`
- Fallback → `ConfigSource::Default`

This is cleaner than manual `Sourced<T>` layering — figment handles the merge, we query provenance post-hoc.

### What about `git config --show-origin`?

gix's `config_snapshot()` merges all scopes into a single view. To show individual git config scope (local vs global), we'd need scope-by-scope queries or separate `GitConfigProvider` instances per scope. For MVP, showing "git config" as a single source is sufficient. Scope-level detail is deferred to post-MVP.

### Backward Compatibility

- `load_config()` migrated to use figment internally
- `load_layered_config()` is the new entry point for commands with provenance
- `ResolvedConfig::to_config()` bridges to the updated `Config` type
- **Breaking**: `Config.components` changes from `IndexMap<String, Component>` to `Vec<Component>` — all consumers must update
- **Breaking**: `.atomic.toml` format changes — old format produces a clear error with migration instructions

## Files Changed

| File | Action |
|------|--------|
| `src/config/source.rs` | **New**: `ConfigSource`, `Sourced<T>` |
| `src/config/git_provider.rs` | **New**: `GitConfigProvider` implementing `figment::Provider` |
| `src/config/layered.rs` | **New**: `load_layered_config()`, `ResolvedConfig`, `validate_resolved()` |
| `src/config/types.rs` | **Breaking**: `IndexMap` → `Vec<Component>` with `name` field, add `FromStr` for `UnmatchedPolicy` |
| `src/config/mod.rs` | Add modules, exports, migrate to figment |
| `src/core/matcher.rs` | Update `ComponentMatcher` for `Vec<Component>` |
| `src/git/atomize.rs` | Update component iteration for `Vec<Component>` |
| `src/cli/commands/init.rs` | Omit settings already covered, generate `[[components]]` format |
| `src/cli/commands/status.rs` | Show config provenance, add `--no-config` |
| `src/cli/commands/atomize.rs` | Use layered config |
| `src/cli/commands/validate.rs` | Use layered config, check name uniqueness |
| `src/cli/output.rs` | Add config provenance printing |
| `src/cli/mod.rs` | Add `--no-config` to `StatusArgs` |
| `docs/src/adr/adr-006-*.md` | Update for figment adoption |
| `docs/src/adr/adr-007-*.md` | **New**: TOML array-of-tables for components |
| `docs/src/SUMMARY.md` | Add ADR-007 link |
| `tests/fixtures/*.toml` | Migrate to `[[components]]` format |

## Post-MVP Extensions

These items were identified during GAP review and deferred:

- **Per-scope git config display**: Show `(git config: local)` vs `(git config: global)` in status output. Could be implemented by creating three separate `GitConfigProvider` instances (system, global, local) with distinct `Metadata` names, letting figment track which scope won per field.
- **`git-atomic config` subcommand**: Dedicated subcommand for reading/writing config values (similar to `git config`). Currently, `status` handles display and users set values directly via `git config` or editing `.atomic.toml`.

## GAP Review Notes

**Review Date**: 2026-01-29

### Gaps Addressed

1. **G1: gix API mismatch** — Fixed code examples to use `string_by("atomic", None, "baseBranch")` instead of `string("atomic.baseBranch")`. Added design note explaining the difference.
2. **G2: `validate` command not updated** — Added task 6.9 to update `validate.rs` to use layered config.
3. **G3: Missing `.atomic.toml` + no components error path** — Defined in task 6.4: settings-only commands succeed, component-requiring commands fail with clear message.
4. **G4: `init` needs repo for git config** — Added to task 6.7: attempt `gix::open(".")`, non-fatal if outside git repo.
5. **G5: No tests specified** — Added task 6.10 with specific unit and integration test cases.

### Areas Refined

1. **A1: `--show-config` flag** — Changed to always show config in status, with `--no-config` to suppress.
2. **A2: `init` edge cases** — Added handling for same-as-default values and outside-git-repo scenarios.
3. **A3: `ConfigSource::Cli` usage** — Documented as reserved/unused this phase in both code comment and rationale section.
4. **A4: `Sourced<Option<String>>` semantics** — figment handles this via its merge logic; provenance queried post-extraction.
5. **A5: `UnmatchedPolicy` `FromStr`** — Added task 6.2 for `FromStr` implementation.

### Extensions Included

- **P3: Cross-source validation** — Added as task 6.5. Validates the merged config, not individual sources.

### Extensions Deferred

- **P1: Per-scope git config display** → post-MVP (solvable via multiple `GitConfigProvider` instances)
- **P2: `git-atomic config` subcommand** → post-MVP

### Decisions Changed

- **P4: figment adoption** — Originally deferred. Now adopted for entire config (settings + components) after confirming figment supports per-field provenance via `find_metadata()`. figment was already a dependency but unused.
- **Component format** — Migrated from `[components.<name>]` (IndexMap) to `[[components]]` (Vec) per ADR-007. This eliminated the insertion-order concern that blocked figment adoption, allowing a single unified config loading path.

## Review Gate

- [ ] All tests pass
- [ ] ADR-006 updated for figment adoption
- [ ] ADR-007 reviewed
- [ ] `git config` values are correctly read via gix (`string_by`) through custom provider
- [ ] Priority chain verified with overlapping sources
- [ ] Cross-source validation catches invalid merged config
- [ ] `init` output adapts to existing config and generates `[[components]]` format
- [ ] `init` works outside a git repo (graceful degradation)
- [ ] `status` provenance output verified
- [ ] `validate` uses layered config and checks name uniqueness
- [ ] Components in `[[components]]` format with document-order preservation
- [ ] Old `[components.name]` format produces clear migration error
- [ ] figment owns entire config (single provider chain, no split paths)
- [ ] Test fixtures migrated to new format
