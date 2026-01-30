# git-atomic justfile
# Run `just --list` for available recipes

set shell := ["bash", "-cu"]

# Default recipe
default:
    @just --list

# =============================================================================
# Development
# =============================================================================

# Install dependencies and verify toolchain
[group('dev')]
setup:
    rustup show
    cargo fetch

# Quick compile check (no codegen)
[group('dev')]
check:
    cargo check --all-targets

# Build debug binary
[group('dev')]
build:
    cargo build

# Build optimized release binary
[group('dev')]
build-release:
    cargo build --release

# Format code
[group('dev')]
fmt:
    cargo fmt

# Format check (CI-friendly)
[group('dev')]
fmt-check:
    cargo fmt -- --check

# =============================================================================
# Testing
# =============================================================================

# Run all tests
[group('test')]
test:
    cargo test

# Run tests with output
[group('test')]
test-verbose:
    cargo test -- --nocapture

# Run specific test
[group('test')]
test-one name:
    cargo test {{name}} -- --nocapture

# =============================================================================
# Linting
# =============================================================================

# Run clippy
[group('lint')]
clippy:
    cargo clippy --all-targets -- -D warnings

# Run all lints (clippy + fmt check)
[group('lint')]
lint: clippy fmt-check

# =============================================================================
# Documentation
# =============================================================================

# Build and open rustdoc
[group('docs')]
docs:
    cargo doc --open --no-deps

# Build mdBook documentation
[group('docs')]
docs-build:
    mdbook build docs

# Serve mdBook locally
[group('docs')]
docs-serve:
    mdbook serve docs --open

# =============================================================================
# Docker
# =============================================================================

# Build Docker image
[group('docker')]
docker-build:
    docker build -t git-atomic:dev -f docker/Dockerfile .

# Run in Docker container
[group('docker')]
docker-run *args:
    docker run --rm -v "$(pwd):/workspace" -w /workspace git-atomic:dev {{args}}

# =============================================================================
# Release
# =============================================================================

# Generate changelog (dry-run)
[group('release')]
changelog-preview:
    git cliff --unreleased

# Full CI check (what CI runs)
[group('release')]
ci: lint test build

# =============================================================================
# Utilities
# =============================================================================

# Clean build artifacts
[group('util')]
clean:
    cargo clean

[group('util')]
# Update dependencies
update:
    cargo update

[group('util')]
[macos]
# Install dependencies
install:
    cargo install tdd-guard-rust
    brew bundle install .claude/reference/Brewfile
    mdbook-mermaid install docs/
    # mdbook-pagetoc install docs/
    # mdbook-svgbob install docs/
    # mdbook-cmdrun install docs/
    # mdbook-admonish install docs/
    # mdbook-linkcheck install docs/

# Show outdated dependencies
[group('util')]
outdated:
    cargo outdated -R

[group('user')]
notes:
    ekphos "{{ justfile_directory() }}/docs" || error("ekphos not installed -> run 'just install'")

# jira-sync:
#     curl https://github.com/steveyegge/beads/raw/main/examples/jira-import/jira2jsonl.py
#     curl https://github.com/steveyegge/beads/raw/main/examples/jira-import/jsonl2jira.py
