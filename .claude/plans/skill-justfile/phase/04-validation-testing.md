# Phase 4: Validation & Testing

## Scope

Validate the skill against meta-skill-authoring-dev criteria and test with pressure scenarios (Iron Law).

## Deliverables

### 1. Pressure Scenarios (RED phase)

Test the skill by simulating these scenarios WITHOUT the skill loaded, documenting failures:

| Scenario | Expected Failure Without Skill |
|----------|-------------------------------|
| "Create a justfile for a new Rust project" | Missing groups, no shell setting, no confirm on destructive |
| "Review this justfile for quality issues" | No checklist, misses anti-patterns |
| "What recipes should I add for CI?" | No maturity guidance, random suggestions |
| "Convert this Makefile to a justfile" | Syntax errors, missing just-specific patterns |
| "Set up monorepo justfile structure" | Missing module system knowledge |
| "Add deployment recipes" | No security/maturity context |

### 2. Skill Validation Checklist

Run meta-skill-validation-dev against the completed skill:

```markdown
## Structure Validation
- [ ] SKILL.md exists
- [ ] SKILL.md < 500 lines (body)
- [ ] Progressive disclosure used (references/, examples/, tables/)
- [ ] Navigation section present
- [ ] No orphan files

## Frontmatter Validation
- [ ] name: hyphen-case, ≤64 chars
- [ ] description: ≤1024 chars, starts with action/trigger phrase

## Quality Validation
- [ ] Templates provided for structured output (review checklist)
- [ ] Examples included for complex tasks
- [ ] Consistent terminology throughout
- [ ] No time-sensitive information
- [ ] No Windows-style paths
- [ ] Prerequisites documented

## Content Validation
- [ ] Author workflow complete
- [ ] Review workflow with checklist
- [ ] Plan workflow with decision tables
- [ ] Update workflow with maturity guidance
- [ ] Quick reference table present
- [ ] Language patterns summarized (detailed in references)
- [ ] aRustyDev conventions documented
```

### 3. GREEN Phase

Load the skill and re-run all pressure scenarios. Document:
- What improved
- What still fails
- Required adjustments

### 4. REFACTOR Phase

- Fix any remaining gaps from GREEN testing
- Remove unused content
- Optimize token usage (tables over prose)
- Ensure all cross-references resolve

## Dependencies

- Phases 1-3 (complete skill needed for testing)

## Acceptance Criteria

- [ ] All 6 pressure scenarios pass with skill loaded
- [ ] Validation checklist fully checked
- [ ] No regression from removing/editing content
- [ ] Skill ready for promotion to aRustyDev/ai
