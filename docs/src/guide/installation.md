# Installation

## Prerequisites

- **Rust toolchain** (1.70 or later) -- required for `cargo install`
- **Git** (2.30 or later) -- git-atomic shells out to git for some operations

## Install via Cargo

The recommended installation method:

```sh
cargo install git-atomic
```

This downloads, compiles, and installs the `git-atomic` binary to
`~/.cargo/bin/`. Make sure this directory is in your `PATH`.

Verify the installation:

```sh
git-atomic --version
```

## Build from Source

Clone the repository and build:

```sh
git clone https://github.com/aRustyDev/git-atomic.git
cd git-atomic
cargo build --release
```

The binary is at `target/release/git-atomic`. Copy it to a directory in your
`PATH`:

```sh
cp target/release/git-atomic ~/.local/bin/
```

## Git Subcommand Integration

Because the binary is named `git-atomic`, git automatically discovers it as a
subcommand. Both of these are equivalent:

```sh
git-atomic commit
git atomic commit
```

No additional configuration is needed for subcommand discovery. Git searches
your `PATH` for any executable named `git-<name>` and exposes it as
`git <name>`.

## Updating

```sh
cargo install git-atomic --force
```

## Uninstalling

```sh
cargo uninstall git-atomic
```
