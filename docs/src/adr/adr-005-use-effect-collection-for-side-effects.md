---
id: d3f1a8c2-7e4b-4a91-b6d3-9c2e5f8a1b04
project:
  id: b0ad8e03-e785-4d81-a998-8c8341976588
title: "ADR-005: Use effect collection for side effects"
status: active
tags: [adr, architecture, effects]
related:
  depends-on: [550e8400-e29b-41d4-a716-446655440000]
---

# ADR-005: Use effect collection for side effects

## Status

Accepted

## Context

`--dry-run` was only available on the `atomize` subcommand, implemented by threading a `dry_run: bool` parameter through to the point where `repo.edit_references()` was called. The `init` command had no dry-run support at all. Adding new commands would require remembering to thread `dry_run` through every mutating function — easy to miss and inconsistent.

Mutations were scattered across command handlers: `atomize` called `repo.edit_references()` directly, `init` called `std::fs::write()` directly, and push logic lived inline in the atomize command. There was no unified way to preview or audit all side effects a command would perform.

## Alternatives Considered

1. **Pass `dry_run: bool` to every mutating function** — The status quo approach. Doesn't scale: each new command or mutation point requires manually threading the flag. Easy to forget, leading to inconsistent dry-run coverage.

2. **Trait-based IO abstraction** — Define a trait like `trait Executor { fn write_ref(...); fn write_file(...); }` with real and mock implementations. Over-engineered for a CLI tool with a small number of effect types. Adds indirection without proportional benefit.

3. **Effect collection (chosen)** — Commands return `Vec<Effect>` describing what they want to do. A single `execute()` function either performs or previews the effects. Keeps commands as near-pure functions while centralizing all mutation logic.

## Decision

All mutations go through an `Effect` enum. Commands produce effects; a single executor handles run vs preview. The gix atomic ref transaction is preserved via a `RefTransaction` variant that batches all ref edits into one all-or-nothing operation.

The `Effect` enum has three variants:
- `RefTransaction` — atomic batch ref update
- `Push` — push branches to a remote
- `WriteFile` — write a file to disk

Git tree/commit object creation stays inline in `plan_atomize()` because these are immutable objects in `.git/objects/` that are harmless without refs pointing to them.

## Consequences

**Positive:**
- Global `--dry-run` works on any command for free
- Effects are auditable and testable — `plan_atomize()` can be tested without any ref mutations
- Commands become near-pure functions (compute results + collect effects)
- New commands get dry-run support by returning effects

**Negative:**
- One layer of indirection for simple writes (e.g., `init` creating a single file)
- Effect variants must be added when new mutation types are introduced
