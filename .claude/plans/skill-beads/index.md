# Plan: Beads (bd) Skill for Claude Code

**Status**: Not Started

## Goal

Create a Claude Code skill (`beads-dev`) that teaches agents how to use the `bd` CLI for dependency-aware issue tracking within projects.

## Context

Beads (`bd`) is a distributed, git-backed issue tracker designed for AI agents. It replaces markdown-based task tracking with a dependency-aware graph stored in `.beads/`. The skill should cover initialization, issue lifecycle, dependency management, and session workflows.

## Scope

- Skill file: `.claude/skills/beads-dev/`
- Covers: `bd` CLI usage for single-agent workflows
- Deferred: multi-agent swarms, Dolt backend, federation, Jira/Linear integrations

## Reference Material

- `reference/cli-reference.md` — full command reference with flags and examples
- `reference/workflows.md` — common workflows and patterns
- `reference/prime-output.md` — output of `bd prime` (agent context injection)
- `reference/agents-detailed.md` — upstream AGENTS_DETAILED.md from beads repo (dev guidelines, session workflow, release process, CLI design principles)

## Progress

- [ ] Gather reference material
- [ ] Draft skill content
- [ ] Review against `meta-skill-authoring-dev` guidelines
- [ ] Validate with `meta-skill-validation-dev`
- [ ] Test in a real session
