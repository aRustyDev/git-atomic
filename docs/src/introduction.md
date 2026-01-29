# Introduction

## What is git-atomic?

git-atomic is a git subcommand that splits multi-component commits into
isolated per-component branches. It reads a `.atomic.toml` configuration file
that maps glob patterns to named components, then creates branches like
`atomic/frontend` and `atomic/backend` where each branch contains only the
files belonging to that component.

## The Problem

Monorepos and multi-component repositories create friction in several areas:

- **CI/CD pipelines** run against the entire repository even when only one
  component changed. This wastes compute and slows feedback loops.

- **Code review** becomes harder when a single pull request touches unrelated
  parts of the codebase. Reviewers must mentally separate concerns that the
  tooling has mixed together.

- **Deployment** of independent components gets coupled. A backend change
  cannot ship without also deploying the frontend changes in the same commit,
  even if they are unrelated.

- **Blame and history** become noisy. When investigating a regression in the
  frontend, the git log includes backend changes that are irrelevant to the
  investigation.

These problems share a root cause: the commit is the wrong unit of
decomposition for multi-component work.

## The Solution: Component-Per-Branch

git-atomic introduces a component-per-branch model. Instead of one commit
touching many components, git-atomic creates a branch for each component and
places only the relevant files on that branch.

Given a commit that modifies files across `frontend` and `backend` components:

```
commit abc123
  src/ui/header.tsx        (frontend)
  src/api/routes.rs        (backend)
  src/components/nav.tsx   (frontend)
  src/db/migrations/001.sql (backend)
```

git-atomic produces:

```
atomic/frontend
  src/ui/header.tsx
  src/components/nav.tsx

atomic/backend
  src/api/routes.rs
  src/db/migrations/001.sql
```

Each branch can then trigger its own CI pipeline, receive its own code review,
and deploy independently.

## What Are Atomic Commits?

In software engineering, an atomic commit is a commit that represents a single,
indivisible unit of change. It should be self-contained: all the files in the
commit relate to a single logical change, and removing any file would break
the intent of the commit.

In practice, developers often create commits that bundle changes across
multiple components because that is how they work -- fixing a frontend bug
while also adjusting the API it calls. These commits are not atomic in the
component sense even though they may be atomic in the logical sense.

git-atomic bridges this gap. It takes a logically atomic commit and splits it
into component-atomic branches. The developer's workflow does not change; the
tooling handles the decomposition.

## How It Works

1. **Configuration**: You define components in `.atomic.toml`, mapping glob
   patterns to component names. Each component gets a branch name template.

2. **Analysis**: When you run `git-atomic commit`, the tool reads the specified
   commit (or HEAD by default) and classifies each changed file into a
   component based on glob matching.

3. **Splitting**: For each component that has changed files, git-atomic creates
   (or updates) a branch containing only those files. The branch is based on
   your configured base branch.

4. **Range support**: You can also pass a commit range like `main..feature`.
   git-atomic computes the net diff across the range, filters out net-zero
   files (files added then removed within the range), and splits the result.

## Design Principles

### Local-first

git-atomic operates entirely on the local repository. It does not push, does
not contact remotes, and does not require network access. Push operations are
opt-in via `--ci-mode` or manual `git push`.

### Non-destructive

git-atomic creates new branches; it never modifies existing branches or
rewrite history. If a component branch already exists and has diverged, the
tool reports the conflict and exits with a non-zero code rather than forcing
an update.

### Deterministic

Given the same commit and configuration, git-atomic always produces the same
result. There is no randomness, no timestamp-dependent behavior, and no
ordering ambiguity. Files are matched using first-match-wins semantics against
the component list.

### Transparent

Every operation can be previewed with `--dry-run`. The tool explains what it
would do without making any changes. Combined with `--json`, this makes
git-atomic scriptable and auditable.

## When to Use git-atomic

git-atomic is useful when:

- Your repository contains multiple independently deployable components.
- You want per-component CI pipelines without restructuring into separate repos.
- Code review quality suffers from large cross-cutting pull requests.
- You need fine-grained control over what gets deployed and when.

git-atomic is not needed when:

- Your repository contains a single component.
- All files in the repository always deploy together.
- You already use separate repositories per component.

## Next Steps

- [Installation](./guide/installation.md) -- get git-atomic on your system.
- [Quickstart](./guide/quickstart.md) -- split your first commit in 5 minutes.
- [Configuration Reference](./reference/configuration.md) -- all config options.
