# Phase 1: Core SKILL.md

## Scope

Create the main `SKILL.md` file (< 500 lines) covering all four workflows with progressive disclosure to reference files.

## Deliverables

### 1. Frontmatter

```yaml
---
name: justfile-dev
description: >-
  Justfile authoring, reviewing, and planning for the just command runner.
  Use when creating new justfiles, adding recipes, reviewing existing justfiles
  for quality, planning recipe sets for new projects, or upgrading justfiles
  as projects mature. Covers syntax patterns, module system, recipe groups,
  language-specific templates, and maturity assessment.
---
```

### 2. SKILL.md Structure (target: ~400 lines)

```
# Justfile Development

## Overview (5 lines)
## When to Use (10 lines — trigger conditions)

## Workflows

### Author (40 lines)
- New justfile scaffold
- Recipe writing patterns
- Group conventions
- Documentation comments

### Review (50 lines)
- Review checklist (copyable)
- Common issues table
- Quality criteria
- Anti-patterns to flag

### Plan (40 lines)
- Project type → recipe set mapping
- Maturity assessment (quick version)
- When to add recipes vs. defer

### Update (30 lines)
- Adding recipes to existing justfiles
- Upgrading maturity level
- Module migration

## Quick Reference (30 lines)
- Syntax table (essential subset)
- Standard groups table
- Common recipe shapes

## aRustyDev Conventions (20 lines)
- Module library (just.arusty.dev)
- Gist templates
- Shell setting, confirm attribute

## Language Patterns (20 lines — summary table)
- Rust, Go, TypeScript, Python (one-liner each)
- Link to references/recipe-patterns.md

## See Also (10 lines)
- Related skills
- Navigation to reference files
```

### 3. Key Design Decisions

**Review Checklist** — Provide a copyable markdown checklist Claude can use:

```markdown
## Justfile Review Checklist

- [ ] `set shell` declared at top
- [ ] All recipes grouped with `[group('name')]`
- [ ] Every recipe has a doc comment
- [ ] No ungrouped recipes (except `default`)
- [ ] Private helpers prefixed with `_`
- [ ] Destructive recipes use `[confirm]`
- [ ] No `cd` usage (use `[working-directory]`)
- [ ] No `&&` chaining (use dependencies or separate lines)
- [ ] No secrets in justfile (env vars or `op://`)
- [ ] `default` recipe shows `just --list`
```

**Maturity Assessment** — Quick 3-question assessment (not bryonjacob's rigid 5-level):

| Question | Yes → | No → |
|----------|-------|------|
| Has CI? | Add quality gates (test-watch, coverage) | Skip quality recipes |
| Deploys to prod? | Add security + deploy recipes | Skip deploy/security |
| Multiple languages? | Add module structure | Keep single justfile |

**Recipe Planning** — Decision table for "what recipes does this project need?":

| Project Type | Baseline | Quality | Deploy | Modules |
|-------------|----------|---------|--------|---------|
| Rust CLI | dev, test, lint, build, release | coverage, bench | docker, release | — |
| Rust lib | dev, test, lint, docs | coverage, bench | release | — |
| Web app | dev, test, lint, build | coverage | docker, deploy | if polyglot |
| Monorepo | orchestrate | per-package | per-service | yes |

## Dependencies

- None (first phase)

## Acceptance Criteria

- [ ] SKILL.md < 500 lines (body, excluding frontmatter)
- [ ] Frontmatter has valid `name` and `description`
- [ ] All four workflows present (Author, Review, Plan, Update)
- [ ] Review checklist is copyable markdown
- [ ] Progressive disclosure links to `references/` and `examples/`
- [ ] No duplication of content from `.claude/rules/justfile.md`
- [ ] Consistent terminology throughout

## Review Gate

- Validate against meta-skill-validation-dev checklist
- Count lines (`wc -l`)
- Verify all internal links resolve
