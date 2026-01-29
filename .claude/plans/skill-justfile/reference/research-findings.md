# Research Findings: Existing Justfile Skills

## Skills Analyzed

### 1. lanej/just (34★)
- **Approach**: Minimal — single-sentence preference
- **Description**: "Use just for command running and task automation. Prefer Justfiles over Makefiles. Keep recipes simple - delegate complex logic to scripts."
- **Takeaway**: Too minimal for our needs, but good description phrasing

### 2. bryonjacob/justfile-maturity-model (0★, 1 fork)
- **Approach**: 5-level maturity progression with YAGNI enforcement
- **Levels**: 0 (Baseline 9 cmds) → 1 (Quality +4) → 2 (Security +4) → 3 (Advanced +6) → 4 (Polyglot)
- **Strengths**: Assessment scripts, upgrade paths, anti-patterns, non-linear progression
- **Weaknesses**: Overly prescriptive (9 mandatory baseline commands), rigid level structure
- **Companion skills**: justfile-interface, justfile-quality-patterns, justfile-security-patterns, justfile-advanced-patterns, justfile-polyglot-patterns
- **Takeaway**: Adapt maturity concept as advisory, not mandatory. Use 3-question assessment.

### 3. laurigates/justfile-expert (5★)
- **Approach**: Full expertise skill with progressive disclosure (SKILL.md + REFERENCE.md)
- **Description**: "Just command runner expertise, Justfile syntax, recipe development, and cross-platform task automation."
- **Files**: 2 (SKILL.md 6.7KB, REFERENCE.md 12.4KB)
- **Takeaway**: Good progressive disclosure model. Description has good trigger phrases.

### 4. derKlinke/justfile-authoring (2★)
- **Approach**: Authoring-focused Codex CLI skill
- **Description**: "Create, edit, or review justfiles. Use when adding or modifying recipes, parameters, dependencies, settings, attributes, aliases, or shebang scripts."
- **Files**: 1 (SKILL.md only)
- **Takeaway**: Best description for authoring use case. Good trigger phrases to borrow.

### 5. rbergman/just-pro (1★)
- **Approach**: Most comprehensive single skill — 7 files with references/
- **Description**: "Patterns for setting up just in projects. Use PROACTIVELY when creating build systems, setting up new repos."
- **Content**: Installation, project patterns (simple/monorepo), recipe patterns (quality gates, coverage, clean, parallel CI), module system (mod, mod?, working dirs, listing), language integration (Go, TypeScript, Rust), mise integration (shell override, graceful degradation), security auditing, reference templates
- **Files**: SKILL.md (9.7KB) + references/ (5 template files)
- **Takeaway**: Best single skill. Borrow: recipe patterns, module system docs, language integration tables, mise integration. Our differentiation: add review, plan, update workflows.

### 6. bryonjacob/justfile-quality-patterns
- **Approach**: Level 1 companion to maturity model
- **Description**: "Level 1 patterns - test-watch, integration-test, complexity, loc, duplicates, slowtests"
- **Takeaway**: Quality recipes to include in our recipe-patterns reference

## Skill Authoring Requirements (from meta-skill-authoring-dev)

### Frontmatter
- `name`: hyphen-case, max 64 chars
- `description`: max 1024 chars, third person, "Use when..." triggers
- Description must describe WHEN to use, NOT summarize workflow

### Structure
- SKILL.md body < 500 lines
- Progressive disclosure: references/, examples/, tables/
- Keep references ONE level deep from SKILL.md
- Reference files > 100 lines should have TOC

### Quality
- Templates for structured output
- Input/output examples
- Consistent terminology
- No time-sensitive info
- No Windows paths
- MCP tools use `mcp__server__tool` format

### Iron Law (TDD for Skills)
- RED: Run pressure scenarios WITHOUT skill, document failures
- GREEN: Write minimal skill addressing failures
- REFACTOR: Close loopholes, re-test

### Output Patterns
- High freedom: text instructions (multiple valid approaches)
- Medium freedom: parameterized templates (preferred pattern, some variation)
- Low freedom: exact commands (fragile operations)
