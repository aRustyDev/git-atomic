---
id: 4f361530-f576-4a8e-bfac-8fa25407c979
project:
  id: b0ad8e03-e785-4d81-a998-8c8341976588
title: "ADR-001: Adopt local-first design over CI-centric architecture"
status: accepted
tags: [adr, architecture]
related:
  supersedes: []
  depends-on: []
---

# ADR-001: Adopt local-first design over CI-centric architecture

## Status

Accepted

## Date

2025-01-29

## Deciders

- Adam (project lead)

## Context and Problem Statement

The original git-atomic design was CI-centric: atomization ran after merge via GitHub Actions, requiring integration branch resets, cross-run state persistence via GitHub artifacts, and a 3-layer race condition defense. This created significant architectural complexity:

- State had to survive across CI runs (artifact storage, local cache, GitHub API)
- Race conditions between concurrent PRs required locks and conflict resolution
- Integration branch reset after atomization required careful coordination
- Resume logic for partial failures was complex and error-prone

The tool's core value proposition — decomposing compound commits into atomic, component-specific branches — does not inherently require CI execution. A developer can atomize locally before pushing, eliminating the entire class of distributed coordination problems.

## Decision Drivers

- Architectural simplicity (fewer moving parts, fewer failure modes)
- Developer experience (fast local feedback loop)
- Reliability (no distributed state to corrupt or lose)
- Incremental adoption (works without any CI setup)
- CI remains viable as a fallback, not the primary path

## Considered Options

### Option 1: CI-centric architecture (original design)

Atomization runs as a GitHub Actions workflow after PR merge. Integration branch is reset after atomization. State persists across runs via GitHub artifacts.

| Pros | Cons |
|------|------|
| Automatic — no developer action needed | Complex state management across CI runs |
| Centralized — single source of truth | Race conditions between concurrent PRs |
| Enforced — can't skip atomization | Integration branch reset is risky |
| | Resume logic for partial failures is complex |
| | Requires GitHub-specific infrastructure |

### Option 2: Local-first architecture

Atomization runs on the developer's machine before push. CI acts as optional fallback for PRs that weren't pre-atomized. No persistent state needed — re-run is cheap.

| Pros | Cons |
|------|------|
| Simple — no distributed state | Requires developer to remember to run |
| Fast — local execution, instant feedback | Not enforced by default |
| Reliable — no race conditions or partial state | CI fallback still needed for enforcement |
| Portable — no GitHub-specific dependencies | |
| Stateless — re-run instead of resume | |

## Decision Outcome

Chose **Option 2: Local-first architecture** because it eliminates the most complex and risky parts of the system (distributed state, race conditions, integration branch reset) while preserving the core value proposition. The CI-centric model can be layered on later as a fallback without the original complexity since it only needs to run `git atomic --ci-mode` on branches that weren't pre-atomized.

### Confirmation

- MVP ships without any CI-specific state management code
- Single commit atomization completes in < 2 seconds locally
- CI fallback works as a simple workflow calling the same binary

## Diagram

```mermaid
flowchart TD
    subgraph "Local-First (Chosen)"
        A1[Developer commits] --> A2[git atomic]
        A2 --> A3[Atomic branches created locally]
        A3 --> A4[git push origin atomic/*]
    end

    subgraph "CI Fallback (Optional)"
        B1[PR opened without atomic branches] --> B2[CI runs git atomic --ci-mode]
        B2 --> B3[Atomic branches pushed by CI]
    end

    subgraph "Original CI-Centric (Rejected)"
        C1[PR merged] --> C2[CI atomize workflow]
        C2 --> C3[Load state from artifacts]
        C3 --> C4[Acquire locks]
        C4 --> C5[Reset integration branch]
        C5 --> C6[Create atomic branches]
        C6 --> C7[Persist state to artifacts]
        C7 --> C8[Release locks]
    end

    style A1 fill:#2d6,stroke:#1a4,color:#fff
    style A2 fill:#2d6,stroke:#1a4,color:#fff
    style A3 fill:#2d6,stroke:#1a4,color:#fff
    style A4 fill:#2d6,stroke:#1a4,color:#fff
    style C1 fill:#d44,stroke:#a22,color:#fff
    style C2 fill:#d44,stroke:#a22,color:#fff
    style C3 fill:#d44,stroke:#a22,color:#fff
    style C4 fill:#d44,stroke:#a22,color:#fff
    style C5 fill:#d44,stroke:#a22,color:#fff
    style C6 fill:#d44,stroke:#a22,color:#fff
    style C7 fill:#d44,stroke:#a22,color:#fff
    style C8 fill:#d44,stroke:#a22,color:#fff
```

## Consequences

### Positive

- Eliminates all distributed state management (artifacts, locks, cross-run resume)
- Eliminates race conditions between concurrent PRs
- Eliminates integration branch reset risk
- Faster feedback loop for developers
- Simpler codebase — fewer modules, fewer error paths
- No GitHub-specific dependencies in core logic

### Negative

- Atomization is not enforced by default — developers must run `git atomic`
- CI fallback is still needed for teams that want enforcement
- Two execution paths (local + CI) must be tested

### Neutral

- CI mode becomes a thin wrapper around the same local logic
- The original design's race condition defenses become unnecessary rather than wrong

## References

- [Requirements: Key Pivot from Original Design](../../.claude/plans/mvp/reference/requirements.md#key-pivot-from-original-design)
- [Requirements: Section 9.3 — Explicitly Removed](../../.claude/plans/mvp/reference/requirements.md#93-explicitly-removed-from-original-design)
