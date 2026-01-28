---
paths:
  - "**/.claude/rules/**.md"
---
Rules Loading Mechanics

Fully loaded at startup. Application is conditional, not loading.

- ALL .md files in `.claude/rules/` are discovered and loaded into context at session start
- *Rules WITH paths*: frontmatter are loaded but only applied when working with matching files
- *Rules WITHOUT paths*: apply unconditionally

*Implication*: A rule with paths: ["src/api/**/*.ts"] consumes context tokens even if you're working in tests/. The filtering happens at application time, not load time.
