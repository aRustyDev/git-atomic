# Plan: architecture-decision-records-dev Skill

## Overview

Create a unified ADR skill covering authoring, reviewing, planning, updating, and backfilling Architecture Decision Records. Integrates with the existing `.claude/rules/architecture-decision-records.md` rule and aRustyDev conventions (frontmatter schema, `docs/src/adr/` directory).

## Architecture

```
.claude/skills/architecture-decision-records-dev/
├── SKILL.md                          # < 500 lines — 5 workflows
├── references/
│   ├── adr-template.md               # MADR template with frontmatter
│   ├── quality-checklist.md          # E.C.A.D.R. + strategic lenses
│   ├── code-traceability.md          # Language-specific ADR references
│   └── decision-triggers.md          # When to create/not create ADRs
├── examples/
│   ├── technology-selection.md       # Database/framework choice
│   └── architectural-change.md       # Migration/pattern change
└── tables/
    └── status-lifecycle.md           # Status transitions + governance
```

## Workflows

| Workflow | Purpose | Key Feature |
|----------|---------|-------------|
| **Author** | Create new ADR | Template + frontmatter + mandatory diagram |
| **Review** | Validate ADR quality | E.C.A.D.R. checklist + strategic lenses |
| **Plan** | Decide what needs an ADR | Decision triggers + "when NOT to create" |
| **Update** | Supersede/deprecate | Lifecycle management + cross-references |
| **Backfill** | Reconstruct from git history | File classification + clustering |

## Differentiation from Existing Skills

| Feature | Existing Skills | Our Skill |
|---------|----------------|-----------|
| Unified workflows | Separate skills per workflow | Single skill, 5 workflows |
| aRustyDev conventions | Generic paths | `docs/src/adr/`, frontmatter schema |
| Rule integration | None | Works with `.claude/rules/architecture-decision-records.md` |
| Quality framework | Various | E.C.A.D.R. + strategic lenses |
| Backfill | liza-mas standalone | Integrated workflow |
| Code traceability | terrylica standalone | Reference file |

## Rule Update Plan

The existing rule at `.claude/rules/architecture-decision-records.md` is minimal (3 bullet points). It needs to be updated to:
- Define ADR directory (`docs/src/adr/`)
- Define naming convention (`adr-NNN-title.md`)
- Define required frontmatter fields
- Reference the skill for authoring workflows
- Define status values and transitions

## GAP Review

### Gaps
- Current rule has no structure — needs complete rewrite
- No ADR template exists in the project
- No examples of ADRs exist yet

### Areas for Refinement
- Frontmatter schema needs alignment with `frontmatter.md` rule
- ADR numbering: 3-digit (`NNN`) vs 4-digit (`NNNN`) — use 3-digit per CLAUDE.md convention

### Potential Extensions
- v1.1: Slash command `/adr` for quick ADR creation
- v1.1: ADR index generation recipe in justfile
- v1.2: Graph data integration (ADR relationships in KuzuDB)

## Progress Tracker

- [ ] Phase 1: Core SKILL.md + Rule Update
- [ ] Phase 2: Reference Files
- [ ] Phase 3: Examples & Templates
- [ ] Phase 4: Validation & Testing
