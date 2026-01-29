---
id: 8b2f4e71-a3d9-4c82-b517-6d1e9f3a8c05
project:
  id: b0ad8e03-e785-4d81-a998-8c8341976588
title: "ADR-006: Layered configuration with git config support"
status: accepted
tags: [adr, configuration, git-config]
related:
  depends-on:
    - c5a7acfb-67cb-4854-8139-3e0bc5dd9bf1
---

# ADR-006: Layered configuration with git config support

## Status

Accepted

## Date

2026-01-29

## Deciders

- Adam (project lead)

## Context and Problem Statement

git-atomic reads all configuration from `.atomic.toml`. This works for team-shared settings but provides no mechanism for per-user overrides (e.g., a different remote for fork workflows) or per-session overrides without modifying a committed file. Users expect git subcommands to respect `git config` conventions — tools like `git-lfs`, `git-absorb`, and `git-branchless` all read from git config.

Additionally, `git-atomic init` generates a full `.atomic.toml` with all defaults, even when some settings are already configured elsewhere. And `git-atomic status` shows branch state but not the resolved configuration, making it hard to debug why a setting has a particular value.

## Decision Drivers

- Users expect `git config` support from git subcommands
- Per-user overrides shouldn't require changing committed files
- CI environments need ENV-based overrides
- Users need to understand which source a configuration value came from
- Must preserve TOML insertion order for first-match-wins (ADR-003)

## Considered Options

### Option 1: figment-based layering

Use the figment crate to merge providers (TOML, ENV, git config via custom provider).

| Pros | Cons |
|------|------|
| Battle-tested merging logic | Already avoided for config loading (insertion order concern) |
| Supports many formats | Adds complexity for ~5 settings keys |
| Provider abstraction | Custom provider needed for git config anyway |
| | Doesn't track provenance per-field natively |

### Option 2: Direct layered resolution (chosen)

Implement layering manually: iterate sources in priority order, track which source provided each value using a `Sourced<T>` wrapper.

| Pros | Cons |
|------|------|
| Full control over merge semantics | More code to maintain than figment |
| Per-field provenance tracking built in | Must add new sources manually |
| No new dependencies | |
| Preserves TOML insertion order for components | |
| Simple — only ~5 settings to layer | |

### Option 3: git config only (no `.atomic.toml`)

Drop `.atomic.toml` and move everything to git config.

| Pros | Cons |
|------|------|
| Single config source | Git config can't represent ordered maps (ADR-003) |
| Familiar to git users | Glob arrays are awkward in flat key-value |
| | Loses version-controlled component definitions |
| | Breaking change |

## Decision

Use figment-based layered resolution with a custom git config provider and the following priority chain:

```
CLI args > ENV > .atomic.toml > git config > defaults
```

### Priority Rationale

- **`.atomic.toml` over git config**: The TOML file is committed to the repo and represents the team's shared intent. It's reviewable in PRs and versioned. Git config is per-clone and invisible to teammates. Team settings should win over individual config.
- **ENV over `.atomic.toml`**: ENV variables are explicit per-session overrides, commonly used in CI. They should override file-based config.
- **CLI args over everything**: The most explicit, most intentional override.

### Scope Boundaries

Only `[settings]` keys are supported in git config and ENV. Component definitions (`[[components]]`) remain `.atomic.toml`-only because:
1. Components use array values (glob patterns)
2. Git config's flat key-value format cannot faithfully represent this structure
3. Components are team-shared definitions that belong in version control

Note: Components use TOML array-of-tables format (`[[components]]`) per ADR-007, which guarantees document order by spec and allows figment to load the entire config through a single provider chain.

### Git Config Namespace

Settings use the `atomic.*` section with camelCase keys (matching git conventions):

| Git Config Key | Setting |
|----------------|---------|
| `atomic.baseBranch` | `settings.base_branch` |
| `atomic.branchTemplate` | `settings.branch_template` |
| `atomic.unmatchedFiles` | `settings.unmatched_files` |
| `atomic.defaultCommitType` | `settings.default_commit_type` |

### gix Config API

Configuration is read via `repo.config_snapshot()`, which automatically merges system, global, local, and worktree scopes in git's standard order. The merged result is treated as a single "git config" source for provenance tracking.

## Consequences

**Positive:**
- Users can set per-user defaults via `git config --global atomic.baseBranch develop`
- CI can override via `GIT_ATOMIC_BASE_BRANCH=release`
- `git-atomic status` shows where each setting comes from — debugging config issues becomes straightforward
- `git-atomic init` generates minimal `.atomic.toml` — only settings not already covered elsewhere
- Consistent with git ecosystem conventions

**Negative:**
- Settings exist in up to 4 places — potential confusion about which source wins
- Must document the priority chain clearly
- Component definitions are asymmetric — TOML only while settings support multiple sources

**Implementation Note:**
figment handles the entire config via its provider chain (defaults → git config custom provider → TOML → ENV). Components use TOML array-of-tables format (ADR-007) which preserves document order by spec, allowing figment to load them directly without a separate code path.
