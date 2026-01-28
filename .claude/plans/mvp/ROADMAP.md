# Roadmap

## MVP - v0.1.0

Core functionality for local-first atomic branch creation.

### Milestones

- [ ] Phase 1: Core parsing & component detection
- [ ] Phase 2: Branch creation & commit generation
- [ ] Phase 3: CLI interface & UX
- [ ] Phase 4: Testing & documentation

### MVP Scope

**In Scope:**
- `git atomic` command (default: atomize current branch)
- Component detection from directory structure
- Atomic branch creation (`atomic/{component}`)
- Conventional commit generation
- Clear output with push commands

**Out of Scope (Post-MVP):**
- GitHub PR integration
- CI workflow components
- Configuration file support
- Custom component patterns

---

## v0.2.0 (Minor)

Configuration and customization.

- [ ] `.git-atomic.toml` configuration file
- [ ] Custom component patterns
- [ ] Ignore patterns
- [ ] Dry-run mode

---

## v0.3.0 (Minor)

CI integration.

- [ ] GitHub Action for CI fallback
- [ ] PR description parsing
- [ ] Status comments on PRs

---

## v1.0.0 (Major)

Production-ready with full feature set.

- [ ] Stable API
- [ ] Comprehensive documentation
- [ ] Performance optimizations
- [ ] Plugin system (stretch)

---

## Links

<!-- Populate when issues/milestones are created -->
- Project Board: TBD
- Milestone: MVP: TBD
- Milestone: v0.2.0: TBD

---

## Version Bump Guidelines

| Change Type | Version Bump |
|-------------|--------------|
| Bug fixes | Patch (0.1.x) |
| New features (backward compatible) | Minor (0.x.0) |
| Breaking changes | Major (x.0.0) |
| Pre-1.0: Breaking changes | Minor (0.x.0) |
