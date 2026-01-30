use std::path::PathBuf;
use std::process::ExitCode;

/// Top-level error type for git-atomic.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    General(String),

    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Git(#[from] GitError),

    #[error("unmatched files: {}", .paths.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", "))]
    UnmatchedFiles { paths: Vec<PathBuf> },

    #[error("branch {branch} has diverged from {base}")]
    DivergedBranch { branch: String, base: String },
}

/// Configuration errors (exit code 2).
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("config file not found: {path}")]
    NotFound { path: PathBuf },

    #[error("invalid config: {reason}")]
    Invalid { reason: String },

    #[error("invalid glob pattern in component {component:?}: {pattern:?}: {reason}")]
    InvalidGlob {
        component: String,
        pattern: String,
        reason: String,
    },

    #[error("failed to read config: {0}")]
    Io(#[from] std::io::Error),
}

/// Git operation errors (exit code 3).
#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("not a git repository: {path}")]
    NotARepo { path: PathBuf },

    #[error("failed to resolve reference {reference:?}: {reason}")]
    ResolveRef { reference: String, reason: String },

    #[error("git operation failed: {0}")]
    Operation(String),

    #[error("tree entry not found: {path}")]
    TreeEntryNotFound { path: String },

    #[error("ref update failed for {branch}: {reason}")]
    RefUpdate { branch: String, reason: String },

    #[error(transparent)]
    Gix(Box<gix::open::Error>),
}

impl Error {
    /// Map error to process exit code per ADR-004.
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Error::General(_) => ExitCode::from(1),
            Error::Config(_) => ExitCode::from(2),
            Error::Git(_) => ExitCode::from(3),
            Error::UnmatchedFiles { .. } => ExitCode::from(4),
            Error::DivergedBranch { .. } => ExitCode::from(5),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_codes_match_spec() {
        assert_eq!(Error::General("x".into()).exit_code(), ExitCode::from(1));
        assert_eq!(
            Error::Config(ConfigError::Invalid {
                reason: "bad".into()
            })
            .exit_code(),
            ExitCode::from(2)
        );
        assert_eq!(
            Error::Git(GitError::Operation("fail".into())).exit_code(),
            ExitCode::from(3)
        );
        assert_eq!(
            Error::UnmatchedFiles { paths: vec![] }.exit_code(),
            ExitCode::from(4)
        );
        assert_eq!(
            Error::DivergedBranch {
                branch: "a".into(),
                base: "b".into()
            }
            .exit_code(),
            ExitCode::from(5)
        );
    }
}
