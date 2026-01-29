pub mod types;

pub use types::{Component, Config, Settings, UnmatchedPolicy};

use crate::core::ConfigError;
use std::path::Path;

/// Load configuration from `.atomic.toml` (or a custom path).
///
/// Uses the `toml` crate directly (not figment) to preserve TOML insertion
/// order in `IndexMap`, which is critical for first-match-wins (ADR-003).
pub fn load_config(path: &Path) -> Result<Config, ConfigError> {
    if !path.exists() {
        return Err(ConfigError::NotFound {
            path: path.to_path_buf(),
        });
    }

    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content).map_err(|e| ConfigError::Invalid {
        reason: e.to_string(),
    })?;

    validate_config(&config)?;
    Ok(config)
}

/// Validate that all glob patterns compile.
fn validate_config(config: &Config) -> Result<(), ConfigError> {
    for (name, component) in &config.components {
        for pattern in &component.globs {
            globset::Glob::new(pattern).map_err(|e| ConfigError::InvalidGlob {
                component: name.clone(),
                pattern: pattern.clone(),
                reason: e.to_string(),
            })?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_toml(dir: &std::path::Path, content: &str) -> std::path::PathBuf {
        let path = dir.join(".atomic.toml");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
        path
    }

    #[test]
    fn load_valid_config() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_toml(
            dir.path(),
            r#"
[settings]
base_branch = "develop"

[components.frontend]
globs = ["src/ui/**"]

[components.backend]
globs = ["src/api/**", "src/db/**"]
commit_type = "fix"
"#,
        );

        let cfg = load_config(&path).unwrap();
        assert_eq!(cfg.settings.base_branch, "develop");
        assert_eq!(cfg.settings.unmatched_files, UnmatchedPolicy::Error);
        assert_eq!(cfg.components.len(), 2);

        // Verify insertion order is preserved (IndexMap).
        let keys: Vec<_> = cfg.components.keys().cloned().collect::<Vec<_>>();
        assert_eq!(keys[0], "frontend");
        assert_eq!(keys[1], "backend");
    }

    #[test]
    fn defaults_applied() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_toml(
            dir.path(),
            r#"
[components.app]
globs = ["**"]
"#,
        );

        let cfg = load_config(&path).unwrap();
        assert_eq!(cfg.settings.base_branch, "main");
        assert_eq!(cfg.settings.branch_template, "atomic/{component}");
        assert_eq!(cfg.settings.unmatched_files, UnmatchedPolicy::Error);
    }

    #[test]
    fn invalid_glob_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_toml(
            dir.path(),
            r#"
[components.bad]
globs = ["[invalid"]
"#,
        );

        let err = load_config(&path).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidGlob { .. }));
    }

    #[test]
    fn missing_config_file() {
        let err = load_config(Path::new("/nonexistent/.atomic.toml")).unwrap_err();
        assert!(matches!(err, ConfigError::NotFound { .. }));
    }
}
