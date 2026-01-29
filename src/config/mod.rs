pub mod git_provider;
pub mod layered;
pub mod source;
pub mod types;

pub use layered::{load_layered_config, ResolvedConfig};
pub use source::{ConfigSource, Sourced};
pub use types::{Component, Config, Settings, UnmatchedPolicy};

use crate::core::ConfigError;
use std::path::Path;

/// Load configuration from `.atomic.toml` (or a custom path).
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

/// Validate that all glob patterns compile and component names are unique.
fn validate_config(config: &Config) -> Result<(), ConfigError> {
    // Check component name uniqueness
    let mut seen = std::collections::HashSet::new();
    for component in &config.components {
        if !seen.insert(&component.name) {
            return Err(ConfigError::Invalid {
                reason: format!("duplicate component name: {:?}", component.name),
            });
        }
        for pattern in &component.globs {
            globset::Glob::new(pattern).map_err(|e| ConfigError::InvalidGlob {
                component: component.name.clone(),
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

[[components]]
name = "frontend"
globs = ["src/ui/**"]

[[components]]
name = "backend"
globs = ["src/api/**", "src/db/**"]
commit_type = "fix"
"#,
        );

        let cfg = load_config(&path).unwrap();
        assert_eq!(cfg.settings.base_branch, "develop");
        assert_eq!(cfg.settings.unmatched_files, UnmatchedPolicy::Error);
        assert_eq!(cfg.components.len(), 2);

        // Verify document order is preserved.
        assert_eq!(cfg.components[0].name, "frontend");
        assert_eq!(cfg.components[1].name, "backend");
    }

    #[test]
    fn defaults_applied() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_toml(
            dir.path(),
            r#"
[[components]]
name = "app"
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
[[components]]
name = "bad"
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

    #[test]
    fn duplicate_component_names_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_toml(
            dir.path(),
            r#"
[[components]]
name = "app"
globs = ["src/**"]

[[components]]
name = "app"
globs = ["lib/**"]
"#,
        );

        let err = load_config(&path).unwrap_err();
        assert!(matches!(err, ConfigError::Invalid { .. }));
    }
}
