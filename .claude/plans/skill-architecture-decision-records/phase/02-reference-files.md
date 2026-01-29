# Phase 2: Reference Files

## Scope

Create progressive disclosure reference files that SKILL.md links to for detailed content.

## Deliverables

### 1. `references/adr-template.md` (~100 lines)

Complete MADR template with aRustyDev frontmatter:

```markdown
---
id: <uuidv4>
project:
  id: <project-uuid>
title: ADR-NNN: <Title in Imperative Mood>
status: proposed
tags: [adr, <domain>]
related:
  supersedes: []
  depends-on: []
---

# ADR-NNN: <Title>

## Status

Proposed | Accepted | Deprecated | Superseded by [ADR-NNN](./adr-NNN-title.md)

## Date

YYYY-MM-DD

## Deciders

- <name/role>

## Context and Problem Statement

<What situation necessitates this decision? What constraints exist?>

## Decision Drivers

- <Driver 1>
- <Driver 2>

## Considered Options

### Option 1: <Name>
<Description, pros, cons>

### Option 2: <Name>
<Description, pros, cons>

## Decision Outcome

Chose **Option N: <Name>** because <rationale>.

## Diagram

\`\`\`mermaid
<at least one diagram showing the decision's architectural impact>
\`\`\`

## Consequences

### Positive
- <benefit>

### Negative
- <trade-off>

### Neutral
- <observation>

## References

- [Related ADR](./adr-NNN-title.md)
- [INVESTIGATE: <gap to fill later>]
```

### 2. `references/quality-checklist.md` (~120 lines)

Full quality framework:

| Section | Content |
|---------|---------|
| E.C.A.D.R. criteria | Detailed definitions of each criterion |
| Strategic lenses | Chesterton's Fence, Path Dependence, Core vs Context, Second-System Effect |
| Quality scoring | Rubric for ADR quality (from lyndonkl) |
| Anti-patterns | Common ADR mistakes and fixes |
| INVESTIGATE markers | How to mark and resolve gaps |

### 3. `references/code-traceability.md` (~80 lines)

Language-specific patterns for ADR references in code:

| Language | File Header | Inline Comment |
|----------|-------------|----------------|
| Rust | `//! ADR: NNN` | `// ADR: NNN - reason` |
| Go | `// Package ... ADR: NNN` | `// ADR: NNN - reason` |
| TypeScript | `/** @see ADR: NNN */` | `// ADR: NNN - reason` |
| Python | `"""...\nADR: NNN\n"""` | `# ADR: NNN - reason` |

Plus placement decision tree and "when NOT to add" guidance.

### 4. `references/decision-triggers.md` (~80 lines)

Detailed guidance on what warrants an ADR:

| Section | Content |
|---------|---------|
| Decision triggers | Expanded table with examples |
| Scope assessment | How to scope to one decision |
| Timing guidance | When to write (before, during, after implementation) |
| Backfill signals | How to identify un-documented decisions in git history |
| File classification | Tier 0-3 for architectural vs supportive files |

## Dependencies

- Phase 1 (SKILL.md must exist to link from)

## Acceptance Criteria

- [ ] Each reference file < 200 lines
- [ ] All files linked from SKILL.md
- [ ] No overlap between reference files
- [ ] Tables preferred over prose
- [ ] Template uses aRustyDev frontmatter schema
- [ ] Code traceability covers Rust, Go, TS, Python
