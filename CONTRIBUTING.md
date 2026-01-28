# Contributing to git-atomic

Thank you for your interest in contributing to git-atomic!

## Getting Started

### Prerequisites

- Rust stable (MSRV: 1.75)
- Git 2.30+
- [just](https://github.com/casey/just) (task runner)
- Docker (optional, for containerized builds)

### Setup

```bash
# Clone the repository
git clone https://github.com/aRustyDev/git-atomic.git
cd git-atomic

# Install dependencies and verify setup
just setup

# Run tests
just test

# Build
just build
```

## Workflow

### 1. Open an Issue First

All work starts with an issue. Before writing code:

1. Check existing issues for duplicates
2. Create a new issue describing the work
3. Wait for acknowledgment on non-trivial changes

### 2. Create a Feature Branch

We use git worktrees for isolation. Branch naming convention:

```
<type>/<issue-number>-<short-description>
```

**Types:** `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci`

**Examples:**
- `feat/42-atomic-commit-command`
- `fix/17-parse-error-handling`
- `docs/23-cli-usage-guide`

```bash
# Create worktree for issue #42
git worktree add ../git-atomic-42 -b feat/42-atomic-commit

# Work in the worktree
cd ../git-atomic-42

# When done
git worktree remove ../git-atomic-42
```

### 3. Make Your Changes

- Write tests for new functionality
- Follow existing code style (run `just fmt`)
- Keep commits atomic and conventional

### 4. Submit a Pull Request

#### Required Artifacts

| Artifact | When Required |
|----------|---------------|
| Tests | All code changes |
| Documentation | New features, API changes |
| CHANGELOG entry | User-facing changes |
| ADR | Architectural decisions |

#### PR Checklist

- [ ] Issue linked (`Closes #123`)
- [ ] Tests pass (`just test`)
- [ ] Lints pass (`just lint`)
- [ ] Documentation updated (if applicable)
- [ ] Commits are conventional format

## Development Commands

```bash
# Setup and verification
just setup          # Install dependencies, verify toolchain
just check          # Quick compile check

# Development
just build          # Build debug binary
just test           # Run all tests
just lint           # Run clippy + fmt check
just fmt            # Format code

# Release
just build-release  # Build optimized binary
just docker-build   # Build Docker image

# Documentation
just docs           # Build and open docs
just docs-serve     # Serve mdBook locally
```

## Code Style

- **Formatting**: `rustfmt` with default settings
- **Linting**: `clippy` with `-D warnings`
- **Commits**: [Conventional Commits](https://www.conventionalcommits.org/)
- **Documentation**: Rustdoc for public APIs

### Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `ci`, `chore`

**Examples:**
```
feat(cli): add --dry-run flag for preview mode
fix(branch): handle diverged branches correctly
docs(readme): add installation instructions
```

## Architecture

See `.claude/plans/mvp/index.md` for architecture overview and diagrams.

Key modules:
- `cli` - Command-line interface (clap)
- `config` - Configuration loading (figment)
- `core` - Atomization engine
- `git` - Git operations (gix)

## Testing

```bash
# Run all tests
just test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test '*'
```

### Test Categories

| Category | Location | Purpose |
|----------|----------|---------|
| Unit | `src/**/tests.rs` | Module-level tests |
| Integration | `tests/` | Cross-module tests |
| E2E | `tests/e2e/` | Full workflow tests |

## Getting Help

- **Questions**: Open a [Discussion](https://github.com/aRustyDev/git-atomic/discussions)
- **Bugs**: Open an [Issue](https://github.com/aRustyDev/git-atomic/issues)
- **Security**: See [SECURITY.md](SECURITY.md)

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.
