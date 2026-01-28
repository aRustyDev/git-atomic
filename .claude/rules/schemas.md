---
globs:
  - *.schema.json
---
# aRustyDev Repository Ecosystem

## Source of Truth Repository for Schemas

- **arustydev/schemas** - Central JSON/YAML schema registry
  - Serves: `schemas.arusty.dev`
  - Reference schemas from here, don't duplicate




## Repository Relationships

```
dotfiles ─────────────────────────────────────────┐
│                                                 │
├── ai (submodule)                                │
│   ├── plugins/                                  │
│   ├── components/                               │
│   └── .claude-plugin/marketplace.json           │
│                                                 │
└── just (installs from just.arusty.dev)          │
                                                  │
gha ──────────── (GitHub Actions source)          │
schemas ──────── (JSON/YAML schemas)              │
homebrew-tap ─── (Homebrew formulas)              │
pre-commit-hooks (management repo) ───────────────│
├── pre-commit-hooks-rs                           │
├── pre-commit-hooks-py                           │
├── pre-commit-hooks-go                           │
└── pre-commit-hooks-js                           │
mcp ──────────── (MCP server management) ──────────┘
```

## Dotfiles Exceptions

The following are managed separately from dotfiles:

- `arustydev/just` - Justfiles (installed via https://just.arusty.dev)
- `arustydev/ai` - AI configs (lives in dotfiles as a git-submodule)
