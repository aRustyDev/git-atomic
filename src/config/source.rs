use std::fmt;

/// Tracks where a configuration value came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigSource {
    /// Hardcoded default value.
    Default,
    /// Git config (system < global < local < worktree, merged by gix).
    GitConfig,
    /// `.atomic.toml` file.
    File,
    /// `GIT_ATOMIC_*` environment variable.
    Env,
    /// CLI argument (reserved — unused this phase).
    Cli,
}

impl ConfigSource {
    /// Short label for status output.
    pub fn label(&self) -> &str {
        match self {
            ConfigSource::Default => "default",
            ConfigSource::GitConfig => "git config",
            ConfigSource::File => ".atomic.toml",
            ConfigSource::Env => "env",
            ConfigSource::Cli => "cli",
        }
    }
}

impl fmt::Display for ConfigSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

/// A configuration value with its provenance.
#[derive(Debug, Clone)]
pub struct Sourced<T> {
    pub value: T,
    pub source: ConfigSource,
}

impl<T> Sourced<T> {
    pub fn new(value: T, source: ConfigSource) -> Self {
        Self { value, source }
    }
}
