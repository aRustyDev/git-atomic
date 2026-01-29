# Phase 5: Structured Dry-Run Output

**Status**: Not Started
**Issue**: TBD
**Parent**: [MVP Plan](../index.md)

## Scope

Enhance the `Effect::WriteFile` variant so that `--json --dry-run` outputs structured content (parsed from the file format) rather than a raw string. The initial implementation covers TOML files (`.atomic.toml` from `init`). Other formats fall back to the raw string.

## Deliverables

1. `Effect::WriteFile` gains an optional `structured: Option<serde_json::Value>` field
2. `init` command populates `structured` by serializing the `Config` directly to JSON
3. `print_effect_preview` uses `structured` when present, falls back to raw `content`
4. TOML header comments are intentionally dropped in JSON output (machine consumption, not human)

## Skills

- `lang-rust-dev`

## Dependencies

- Phase 4: Effect Collection (the `Effect` enum and `execute()` function)

## Architecture

```
Producer (init)                         Printer (JSON mode)
  │                                       │
  ├─ content: String (rendered TOML)      │
  ├─ structured: Some(json!({             ├─ uses structured if present
  │    "settings": {...},                  │
  │    "components": {...}                 └─ falls back to content string
  │  }))
  └─ path: PathBuf
```

The `execute()` function always uses `content: String` for actual file writes. The `structured` field is only used by the printer for JSON dry-run output.

## Implementation Tasks

### 5.1 Update `Effect::WriteFile`

- [ ] Add `structured: Option<serde_json::Value>` field to `WriteFile`
- [ ] Update all existing `WriteFile` constructions to include `structured: None` (or populate it)
- [ ] `execute()` ignores `structured` — always writes `content`

### 5.2 Update `init` command

- [ ] Serialize `Config::sample()` to `serde_json::Value` via `serde_json::to_value()`
- [ ] Pass as `structured: Some(value)` in the `WriteFile` effect

### 5.3 Update `print_effect_preview`

- [ ] JSON mode: if `structured.is_some()`, emit it as `"content"` instead of the raw string
- [ ] Human mode: no change (still prints the raw content lines)

### 5.4 Update tests

- [ ] Test that `init` dry-run JSON output has structured `content` with `settings` and `components` keys
- [ ] Test that `WriteFile` without `structured` falls back to raw string in JSON output

## Acceptance Criteria

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes
- [ ] `git-atomic --json --dry-run init` outputs `"content": {"settings": {...}, "components": {...}}`
- [ ] `git-atomic --dry-run init` (human mode) still shows the raw TOML content
- [ ] TOML header comments are not present in JSON output (expected, documented)
- [ ] Any future `WriteFile` effect without `structured` falls back to raw string

## Files Changed

| File | Action |
|------|--------|
| `src/core/effect.rs` | Add `structured` field to `WriteFile` |
| `src/cli/commands/init.rs` | Populate `structured` from `Config` |
| `src/cli/output.rs` | Use `structured` in JSON preview |
| `src/git/atomize.rs` | No change (no `WriteFile` effects) |

## Design Notes

### Why not parse content at print time?

Parsing TOML → JSON at print time would work but couples the printer to file format detection. Having the producer supply structured data keeps the printer generic.

### Why `Option<serde_json::Value>` instead of a trait?

A trait like `StructuredContent` would be more type-safe but over-engineered for this use case. `serde_json::Value` is the output format anyway, and the field is optional for effects that don't need structured output.

### TOML header comments

TOML parsers discard comments. Since JSON output targets machine consumption, losing comments is acceptable and documented.

## Review Gate

- [ ] All tests pass
- [ ] JSON output verified manually with `git-atomic --json --dry-run init`
