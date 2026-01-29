# Skill Plan: justfile-dev

> A progressive Claude Code skill for justfile authoring, reviewing, updating, and planning.

## Overview

Create a comprehensive `justfile-dev` skill that covers the full lifecycle of justfile management — from creating new justfiles to reviewing existing ones, planning recipes for new projects, and upgrading justfiles as projects mature.

## Architecture

```
components/skills/justfile-dev/
├── SKILL.md              # < 500 lines — core patterns, quick reference, workflows
├── references/
│   ├── syntax-quick-ref.md    # Just syntax cheat sheet
│   ├── recipe-patterns.md     # Common recipe patterns by language/domain
│   ├── module-system.md       # Module system, imports, CDN modules
│   └── maturity-model.md      # Maturity levels and upgrade paths
├── examples/
│   ├── rust-project.just      # Rust project template
│   ├── monorepo-root.just     # Monorepo root router
│   ├── polyglot.just          # Multi-language project
│   └── arustydev.just         # aRustyDev ecosystem conventions
└── tables/
    ├── standard-groups.md     # Group naming conventions
    └── language-recipes.md    # Language-specific recipe matrix
```

## Differentiation from Existing Skills

| Existing Skill | Focus | Our Differentiation |
|----------------|-------|---------------------|
| lanej/just | Minimal preference ("prefer just over make") | Full authoring guide with progressive disclosure |
| bryonjacob/maturity-model | 5-level maturity assessment | Integrated maturity as ONE section, not sole focus |
| laurigates/justfile-expert | Syntax reference + REFERENCE.md | Workflow-oriented (author/review/plan/update) |
| derKlinke/justfile-authoring | Authoring syntax reference | Ecosystem-aware (aRustyDev modules, CDN, templates) |
| rbergman/just-pro | Project setup patterns | Adds review, maturity assessment, upgrade workflows |

### Our Unique Value

1. **Four workflows**: Author → Review → Plan → Update (not just syntax reference)
2. **aRustyDev ecosystem integration**: CDN modules, gist templates, just.arusty.dev
3. **Maturity model** adapted for progressive adoption (not prescriptive levels)
4. **Progressive disclosure** following meta-skill-authoring-dev guidelines
5. **Review checklists** for auditing existing justfiles

## Progress

| Phase | Status | Issue |
|-------|--------|-------|
| 1: Core SKILL.md | Pending | — |
| 2: Reference Files | Pending | — |
| 3: Examples & Templates | Pending | — |
| 4: Validation & Testing | Pending | — |

## Research Sources

### Analyzed Skills (6)

1. **lanej/just** (34★) — Minimal, preference-level
2. **bryonjacob/justfile-maturity-model** (0★, 1 fork) — 5-level maturity with YAGNI
3. **laurigates/justfile-expert** (5★) — Syntax + REFERENCE.md, 2 files
4. **derKlinke/justfile-authoring** (2★) — Codex CLI skill, authoring focused
5. **rbergman/just-pro** (1★) — Most comprehensive, 7 files with references/
6. **bryonjacob/justfile-quality-patterns** — Level 1 patterns companion

### Skill Authoring Requirements (from meta-skill-authoring-dev)

- Frontmatter: `name` (hyphen-case, ≤64 chars), `description` (≤1024 chars, "Use when..." trigger)
- Body: < 500 lines with progressive disclosure
- Quality: templates for structured output, examples, consistent terminology
- Iron Law: RED → GREEN → REFACTOR (test skill with pressure scenarios first)

## GAP Review Notes

### Gaps

- None of the existing skills address **reviewing** existing justfiles for quality
- No skill covers the **planning** phase (deciding what recipes a new project needs)
- No skill integrates with a **module ecosystem** (like aRustyDev/just)
- No skill provides **upgrade paths** between maturity levels with concrete recipes

### Areas for Refinement

- Maturity model should be advisory, not prescriptive (avoid bryonjacob's rigid levels)
- Language-specific recipes should be in reference tables, not SKILL.md body
- Module system docs are scattered; consolidate in one reference file

### Potential Extensions

- Slash command `/justfile-review` that triggers the review checklist
- Integration with `gist-templates.md` for `just apply-gist lang_rust` workflows
- Graph data pattern for analyzing recipe dependencies across projects
