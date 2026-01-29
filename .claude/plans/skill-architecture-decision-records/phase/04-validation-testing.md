# Phase 4: Validation & Testing

## Scope

Validate the skill against meta-skill-authoring-dev criteria and test with pressure scenarios (Iron Law).

## Deliverables

### 1. Pressure Scenarios (RED phase)

Test the skill by simulating these scenarios WITHOUT the skill loaded:

| Scenario | Expected Failure Without Skill |
|----------|-------------------------------|
| "Create an ADR for choosing a database" | Missing frontmatter, no diagram, inconsistent format |
| "Review this ADR for quality" | No checklist, misses gaps, no strategic lenses |
| "What decisions should I document as ADRs?" | No trigger guidance, random suggestions |
| "Supersede ADR-001 with a new decision" | Missing cross-references, old ADR not updated |
| "Backfill ADRs from this repo's history" | No classification system, no clustering, invents rationale |
| "Where should I put ADR references in code?" | Inconsistent patterns, wrong placement |

### 2. Skill Validation Checklist

```markdown
## Structure Validation
- [ ] SKILL.md exists
- [ ] SKILL.md < 500 lines (body)
- [ ] Progressive disclosure used (references/, examples/, tables/)
- [ ] Navigation section present
- [ ] No orphan files

## Frontmatter Validation
- [ ] name: hyphen-case, ≤64 chars
- [ ] description: ≤1024 chars, starts with trigger phrase

## Quality Validation
- [ ] Templates provided (E.C.A.D.R. checklist)
- [ ] Examples included for complex tasks
- [ ] Consistent terminology throughout
- [ ] No time-sensitive information
- [ ] No Windows-style paths

## Content Validation
- [ ] Author workflow complete
- [ ] Review workflow with E.C.A.D.R. checklist
- [ ] Plan workflow with decision triggers
- [ ] Update workflow with supersession
- [ ] Backfill workflow with classification
- [ ] Quick reference table present
- [ ] aRustyDev conventions documented

## Rule Validation
- [ ] Rule references skill
- [ ] Rule defines directory and naming
- [ ] Rule defines status values
- [ ] Rule defines frontmatter requirements
- [ ] Rule and skill have no content duplication
```

### 3. GREEN Phase

Load the skill and re-run all pressure scenarios. Document improvements and remaining gaps.

### 4. REFACTOR Phase

- Fix remaining gaps from GREEN testing
- Remove unused content
- Optimize token usage (tables over prose)
- Ensure all cross-references resolve

## Dependencies

- Phases 1-3 (complete skill needed for testing)

## Acceptance Criteria

- [ ] All 6 pressure scenarios pass with skill loaded
- [ ] Validation checklist fully checked
- [ ] No regression from removing/editing content
- [ ] Rule and skill work together correctly
