---
globs:
  - .pre-commit-config.yaml
---

### Pre-commit Hooks

Multi-repo ecosystem at `arustydev/pre-commit-hooks`

- `arustydev/pre-commit-hooks` - Index/management repo only
- Language-specific registries (each is a standalone pre-commit source):
  - `arustydev/pre-commit-hooks-rs` (Rust)
  - `arustydev/pre-commit-hooks-py` (Python)
  - `arustydev/pre-commit-hooks-go` (Go)
  - `arustydev/pre-commit-hooks-js` (JavaScript)
- Same hooks available in each language; swap `repo:` URL to compare performance
