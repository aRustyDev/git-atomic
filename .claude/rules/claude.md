---
paths:
  - "**/CLAUDE.md"
  - "**/.claude/CLAUDE.md"
  - "**/CLAUDE.local.md"
  - "**/.claude/CLAUDE.local.md"
---

Don't duplicate content in these files
  - WHY: If `~/.claude/CLAUDE.md` and `.claude/CLAUDE.md` both exist with identical content, you consume double the context.
  - They layer, not replace.
verify the more local files contain content that is more specific and ideally an extension of the more global files

Priority order (loaded first → last, higher = foundation):
  1. Enterprise policy (`/Library/Application Support/ClaudeCode/CLAUDE.md`)
  2. Project (`./.claude/CLAUDE.md` or `./CLAUDE.md`)
  3. Project rules (`./.claude/rules/*.md`)
  4. User (`~/.claude/CLAUDE.md`)
  5. Project local (`./CLAUDE.local.md`)

Use `@import` syntax in *project* `CLAUDE.md` to reference shared content rather than duplicating.
