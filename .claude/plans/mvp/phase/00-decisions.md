# Phase 0: Decisions & ADRs

**Status**: Not Started
**Issue**: TBD
**Parent**: [MVP Plan](../index.md)

## Skills

- `architecture-decision-records-dev`
- `lang-rust-dev`

## Scope

Resolve all open questions from the requirements and phase plans. Produce ADRs for significant architectural decisions. This phase gates all implementation work.

## Deliverables

1. ADR-001: Adopt local-first design over CI-centric architecture
2. ADR-002: Use gix for git operations
3. ADR-003: Use globset with first-match-wins for component matching
4. ADR-004: Use thiserror for error hierarchy
5. All open questions resolved (documented inline below)

## Dependencies

- None (first phase)

## Acceptance Criteria

- [ ] All 4 ADRs authored in `docs/src/adr/`
- [ ] All open questions from requirements and phases resolved
- [ ] Phase plans updated with resolutions (no remaining OQs)

---

## Resolved Open Questions

### From Requirements

**OQ-MVP-001: Commit Message Strategy**
→ **Decision: Option 1** — Copy source commit message, add `(component)` scope. Simplest approach, preserves developer intent.

**OQ-MVP-002: Multiple Commits to Same Component**
→ **Decision: Option 1** — One atomic commit per source commit. Preserves commit granularity and is easiest to reason about.

**OQ-MVP-003: Atomic Branch Divergence Definition**
→ **Decision: Option 1** — Reachability check (atomic branch tip is not an ancestor of proposed new commit). Simple, correct for MVP.

### From Phase 1

**Glob library choice**
→ **Decision: globset** — Compiled pattern sets, fast matching, supports standard glob syntax. From the `globset` crate.

**Overlap handling**
→ **Decision: First-match-wins** — Components are evaluated in config order; first matching glob claims the file. Documented in config reference.

**Initial commit handling**
→ **Decision: Empty tree diff** — Use git's empty tree SHA as the "parent" tree to diff against for initial commits.

### From Phase 2

**Tree manipulation approach**
→ **Decision: gix tree builder** — Manipulate trees directly via gix's tree builder API. No worktree checkout needed; faster and avoids filesystem side effects.

**Commit author**
→ **Decision: Preserve source author** — Atomic commits retain the original commit's author info. Committer is set to the current user.

**GPG signing**
→ **Decision: Defer to post-MVP** — No GPG signing in v0.1.0. Add as opt-in feature later.

---

## ADR Summary

| ADR | Title | Status |
|-----|-------|--------|
| ADR-001 | Adopt local-first design | To Author |
| ADR-002 | Use gix for git operations | To Author |
| ADR-003 | Use globset with first-match-wins | To Author |
| ADR-004 | Use thiserror for error hierarchy | To Author |

Each ADR follows the format in `docs/src/adr/` per project conventions.

## Review Gate

Before proceeding to Phase 1:

- [ ] All ADRs authored and accepted
- [ ] All open questions documented with decisions
- [ ] Phase plans updated to reference decisions

## References

- [Requirements: Section 10](../reference/requirements.md#10-technical-decisions)
- [Requirements: Section 13](../reference/requirements.md#13-open-questions-for-mvp)
