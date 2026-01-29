# Phase 1: Core SKILL.md + Rule Update

## Scope

Create the main `SKILL.md` file (< 500 lines) and update `.claude/rules/architecture-decision-records.md` with proper structure.

## Deliverables

### 1. Rule Update: `.claude/rules/architecture-decision-records.md`

Replace current minimal content with:

```yaml
---
paths:
  - "docs/src/adr/*.md"
  - "adr-*.md"
---
```

Content:

| Section | Content |
|---------|---------|
| Skill directive | Load `architecture-decision-records-dev` |
| Directory | `docs/src/adr/` |
| Naming | `adr-NNN-title-in-kebab-case.md` |
| Required sections | Status, Context, Decision, Consequences, Alternatives |
| Frontmatter | Per `frontmatter.md` rule + `status` field |
| Status values | Proposed, Accepted, Deprecated, Superseded |
| Immutability | Never edit accepted ADRs — supersede instead |
| One decision per ADR | Keep scope focused |

### 2. SKILL.md Frontmatter

```yaml
---
name: architecture-decision-records-dev
description: >-
  Architecture Decision Record authoring, reviewing, and lifecycle management.
  Use when creating new ADRs, reviewing ADR quality, deciding what warrants
  an ADR, updating ADR status, superseding decisions, or backfilling ADRs
  from git history. Covers MADR template, E.C.A.D.R. quality criteria,
  status lifecycle, and code traceability patterns.
---
```

### 3. SKILL.md Structure (target: ~400 lines)

```
# Architecture Decision Records

## Overview (5 lines)
## When to Use (10 lines — trigger conditions)

## Workflows

### Author (~60 lines)
- ADR creation process
- Naming and numbering
- Required sections with guidance
- Mandatory diagram requirement
- INVESTIGATE markers for gaps
- Frontmatter requirements

### Review (~50 lines)
- E.C.A.D.R. quality checklist (copyable)
- Strategic lenses (simplified)
- Common issues table

### Plan (~40 lines)
- Decision triggers table
- "When to create" vs "When NOT to create"
- Scope assessment (one decision per ADR)

### Update (~30 lines)
- Status transitions
- Supersession workflow
- Cross-referencing related ADRs
- Deprecation process

### Backfill (~30 lines)
- Git archaeology process
- File classification tiers
- Clustering decisions
- Quality bar for reconstructed ADRs

## Quick Reference (~30 lines)
- Status lifecycle table
- ADR naming convention
- Required sections table
- Frontmatter fields

## aRustyDev Conventions (~15 lines)
- Directory: docs/src/adr/
- Frontmatter schema alignment
- Integration with plans and issues

## See Also (~10 lines)
- Navigation to reference files
```

### 4. Key Design Decisions

**E.C.A.D.R. Review Checklist** (copyable):

```markdown
## ADR Review (E.C.A.D.R.)

- [ ] **E**xplicit problem statement — context clearly states the problem
- [ ] **C**omprehensive options — 2+ alternatives with trade-offs
- [ ] **A**ctionable decision — specific, implementable choice stated
- [ ] **D**ocumented consequences — positive, negative, and neutral listed
- [ ] **R**eviewable — readable by someone without current context

### Structure
- [ ] Frontmatter with id, status, project.id
- [ ] Status is valid (Proposed/Accepted/Deprecated/Superseded)
- [ ] One decision per ADR
- [ ] Title in imperative mood
- [ ] At least one diagram (Mermaid)
- [ ] No placeholders or hand-waving
- [ ] Alternatives include honest trade-offs

### Strategic Lenses (for significant decisions)
- [ ] Chesterton's Fence: if changing existing, original purpose documented?
- [ ] Path Dependence: irreversibility assessed, exit strategy defined?
- [ ] Second-System Effect: scope bounded, not "everything we didn't do last time"?
```

**Decision Triggers Table:**

| Trigger | Create ADR? | Why |
|---------|-------------|-----|
| Technology choice (DB, framework, language) | Yes | Shapes system for years |
| Architectural pattern (microservices, event-driven) | Yes | Affects all future development |
| Infrastructure decision (cloud, deployment) | Yes | Lock-in implications |
| Security approach (auth, encryption) | Yes | Compliance and risk |
| API design (versioning, format) | Yes | External contract |
| Implementation detail (function names, variables) | No | Too granular |
| Temporary workaround | No | Not architectural |
| Minor tooling (linter config) | No | Low impact, easily reversible |

**Status Lifecycle:**

```
Proposed → Accepted → [Deprecated | Superseded by ADR-NNN]
```

## Dependencies

- None (first phase)

## Acceptance Criteria

- [ ] Rule file updated with complete ADR conventions
- [ ] Rule references the skill
- [ ] SKILL.md < 500 lines (body)
- [ ] All 5 workflows present (Author, Review, Plan, Update, Backfill)
- [ ] E.C.A.D.R. checklist is copyable markdown
- [ ] Progressive disclosure links to `references/` and `examples/`
- [ ] No duplication between rule and skill
- [ ] Consistent with `frontmatter.md` rule
