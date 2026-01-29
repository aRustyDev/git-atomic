---
id: a7c4e912-3b58-4d0f-9a17-6e8f2d4c5b73
project:
  id: b0ad8e03-e785-4d81-a998-8c8341976588
title: "Effect collection pattern"
status: active
tags: [architecture, patterns, dry-run]
---

# Effect Collection Pattern

**Date**: 2026-01-29
**Tags**: `architecture`, `patterns`, `dry-run`

## Problem Context

`--dry-run` was bolted onto the `atomize` subcommand only. The `init` command had no dry-run support. Adding a new subcommand meant remembering to thread `dry_run: bool` through every function that performed a mutation — and it was easy to miss one.

The atomize command mixed planning logic (resolve commits, build trees, compute branch states) with execution logic (write refs, push branches) in a single function. This made it hard to test the planning logic in isolation.

## Investigation

Three approaches were considered:

1. **Per-function `dry_run` parameter** (status quo) — Every function that mutates state accepts `dry_run: bool` and conditionally skips the mutation. Scales poorly: easy to forget in new code, and the flag must be threaded through every call chain.

2. **Trait-based IO abstraction** — Define a trait with methods for each mutation type, with real and test implementations. Over-engineered for a CLI with few effect types. Adds a layer of abstraction that doesn't pay for itself.

3. **Effect collection** — Commands return a `Vec<Effect>` describing intended mutations. A single `execute()` function either performs them or prints a preview. Commands become near-pure functions.

## Solution

An `Effect` enum captures every kind of mutation the CLI can perform:

```rust
pub enum Effect {
    RefTransaction { repo_path: PathBuf, edits: Vec<PlannedRefEdit> },
    Push { remote: String, branches: Vec<String> },
    WriteFile { path: PathBuf, content: String },
}
```

The key insight: gix's atomic ref transaction (all-or-nothing batch update) is preserved by grouping all ref edits into a single `RefTransaction` variant rather than splitting them into per-branch effects. The transaction boundary moves to the executor but stays atomic.

Git tree and commit object writes remain inline in `plan_atomize()`. These are immutable objects in `.git/objects/` — they're harmless without refs pointing to them, so there's no need to abstract them as effects.

`--dry-run` moved from `AtomizeArgs` to the top-level `Cli` struct as a global flag. Every subcommand gets dry-run support automatically: read-only commands (status, validate) simply produce no effects, so the flag is a no-op.

## Reproduction Steps

1. Before: `cargo run -- atomize --dry-run` worked; `cargo run -- init --dry-run` did not exist
2. After: `cargo run -- --dry-run atomize` and `cargo run -- --dry-run init` both work
3. `plan_atomize()` returns `(Vec<AtomicResult>, Vec<Effect>)` — testable without mutations
4. New test `plan_atomize_returns_effects_without_mutating` verifies no refs are created until `execute()` is called

## Key Takeaways

- When every side effect is a data structure, dry-run/logging/testing come for free
- Preserving existing transaction semantics (gix batch ref update) requires thinking about effect granularity — one `RefTransaction` with many edits, not many individual ref effects
- Immutable object writes (git trees/commits) don't need to be effects — only the operation that makes them reachable (ref update) matters
