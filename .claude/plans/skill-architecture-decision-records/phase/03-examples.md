# Phase 3: Examples

## Scope

Create example ADRs and the status lifecycle lookup table.

## Deliverables

### 1. `examples/technology-selection.md` (~60 lines)

Complete ADR example for a technology choice (e.g., "Use clap for CLI argument parsing"):

- Full frontmatter with UUIDs
- Context: CLI needs argument parsing
- Decision Drivers: ergonomics, derive macros, ecosystem
- Considered Options: clap, argh, pico-args
- Decision Outcome: clap (with rationale)
- Mermaid diagram showing CLI architecture
- Consequences (positive, negative, neutral)

### 2. `examples/architectural-change.md` (~60 lines)

Complete ADR example for an architectural pattern change (e.g., "Use atomic commits as default strategy"):

- Full frontmatter
- Context: git commits need atomicity guarantees
- Considered Options: single commit, squash, atomic
- Supersession example (supersedes a prior ADR)
- Mermaid diagram showing commit flow
- Cross-references to related ADRs

### 3. `tables/status-lifecycle.md` (~40 lines)

| Status | Meaning | Transitions | Governance |
|--------|---------|-------------|------------|
| Proposed | Under discussion | → Accepted, → Withdrawn | Any contributor |
| Accepted | Approved and implemented | → Deprecated, → Superseded | Team/lead approval |
| Deprecated | No longer relevant | Terminal | Document why |
| Superseded | Replaced by newer ADR | Terminal | Link to replacement |
| Withdrawn | Rejected before acceptance | Terminal | Document why rejected |

Plus:
- Transition rules (who can change status)
- Supersession workflow (step by step)
- Cross-referencing format

## Dependencies

- Phase 1 (examples referenced from SKILL.md)
- Phase 2 (examples follow template from references)

## Acceptance Criteria

- [ ] Examples use valid frontmatter per `frontmatter.md` rule
- [ ] Examples include Mermaid diagrams
- [ ] Examples demonstrate different ADR types
- [ ] Status lifecycle covers all transitions
- [ ] No duplicate content across examples
