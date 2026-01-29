use crate::config::git_provider::GitConfigProvider;
use crate::config::source::{ConfigSource, Sourced};
use crate::config::types::{Component, Config, Settings, UnmatchedPolicy};
use crate::core::ConfigError;
use figment::providers::{Env, Format, Serialized, Toml};
use figment::value::magic::Tagged;
use figment::value::Tag;
use figment::Figment;
use serde::Deserialize;
use std::path::Path;

/// Internal extraction target that uses `Tagged` for provenance tracking.
#[derive(Debug, Deserialize)]
struct TaggedSettings {
    #[serde(default = "default_base_branch")]
    base_branch: Tagged<String>,
    #[serde(default = "default_branch_template")]
    branch_template: Tagged<String>,
    #[serde(default = "default_unmatched_files")]
    unmatched_files: Tagged<UnmatchedPolicy>,
    default_commit_type: Option<Tagged<String>>,
}

impl Default for TaggedSettings {
    fn default() -> Self {
        Self {
            base_branch: default_base_branch(),
            branch_template: default_branch_template(),
            unmatched_files: default_unmatched_files(),
            default_commit_type: None,
        }
    }
}

fn default_base_branch() -> Tagged<String> {
    Tagged::from("main".to_string())
}

fn default_branch_template() -> Tagged<String> {
    Tagged::from("atomic/{component}".to_string())
}

fn default_unmatched_files() -> Tagged<UnmatchedPolicy> {
    Tagged::from(UnmatchedPolicy::Error)
}

/// Internal extraction target matching Config but with tagged settings.
#[derive(Debug, Deserialize)]
struct TaggedConfig {
    #[serde(default)]
    settings: TaggedSettings,
    #[serde(default)]
    components: Vec<Component>,
}

/// Fully resolved configuration with provenance tracking.
#[derive(Debug)]
pub struct ResolvedConfig {
    pub base_branch: Sourced<String>,
    pub branch_template: Sourced<String>,
    pub unmatched_files: Sourced<UnmatchedPolicy>,
    pub default_commit_type: Sourced<Option<String>>,
    /// Components from .atomic.toml. Order preserved by TOML array-of-tables spec.
    pub components: Vec<Component>,
}

impl ResolvedConfig {
    /// Convert back to a plain `Config` for use with `ComponentMatcher` etc.
    pub fn to_config(&self) -> Config {
        Config {
            settings: Settings {
                base_branch: self.base_branch.value.clone(),
                branch_template: self.branch_template.value.clone(),
                unmatched_files: self.unmatched_files.value.clone(),
                default_commit_type: self.default_commit_type.value.clone(),
            },
            components: self.components.clone(),
        }
    }
}

/// Map a figment metadata name to our `ConfigSource` enum.
fn source_from_metadata_name(name: &str) -> ConfigSource {
    if name == "git config" {
        ConfigSource::GitConfig
    } else if name.contains(".atomic.toml") || name.starts_with("TOML") {
        ConfigSource::File
    } else if name.contains("GIT_ATOMIC") || name.contains("env") {
        ConfigSource::Env
    } else {
        ConfigSource::Default
    }
}

/// Resolve the source of a `Tagged<T>` value from the figment.
fn resolve_source(figment: &Figment, tag: Tag) -> ConfigSource {
    if tag.is_default() {
        return ConfigSource::Default;
    }
    match figment.get_metadata(tag) {
        Some(md) => source_from_metadata_name(&md.name),
        None => ConfigSource::Default,
    }
}

/// Load configuration with layered resolution and provenance tracking.
///
/// Priority: defaults < git config < .atomic.toml < ENV
///
/// `repo` may be `None` if not inside a git repository (e.g. `init` outside a repo).
/// `config_path` may point to a non-existent file (settings resolve from other sources;
/// components will be empty).
pub fn load_layered_config(
    repo: Option<&gix::Repository>,
    config_path: &Path,
) -> Result<ResolvedConfig, ConfigError> {
    let mut figment = Figment::new()
        .merge(Serialized::defaults(Settings::default()))
        .merge(GitConfigProvider::new(repo));

    if config_path.exists() {
        figment = figment.merge(Toml::file(config_path));
    }

    figment = figment.merge(
        Env::prefixed("GIT_ATOMIC_")
            .map(|key| {
                // GIT_ATOMIC_BASE_BRANCH -> settings.base_branch
                let k = key.as_str().to_lowercase();
                format!("settings.{k}").into()
            }),
    );

    let tagged: TaggedConfig = figment.extract().map_err(|e| ConfigError::Invalid {
        reason: e.to_string(),
    })?;

    // Validate component name uniqueness
    let mut seen = std::collections::HashSet::new();
    for component in &tagged.components {
        if !seen.insert(&component.name) {
            return Err(ConfigError::Invalid {
                reason: format!("duplicate component name: {:?}", component.name),
            });
        }
        // Validate globs
        for pattern in &component.globs {
            globset::Glob::new(pattern).map_err(|e| ConfigError::InvalidGlob {
                component: component.name.clone(),
                pattern: pattern.clone(),
                reason: e.to_string(),
            })?;
        }
    }

    // Build provenance-tracked config
    let base_branch_source = resolve_source(&figment, tagged.settings.base_branch.tag());
    let branch_template_source = resolve_source(&figment, tagged.settings.branch_template.tag());
    let unmatched_files_source = resolve_source(&figment, tagged.settings.unmatched_files.tag());
    let default_commit_type_source = tagged
        .settings
        .default_commit_type
        .as_ref()
        .map(|t| resolve_source(&figment, t.tag()))
        .unwrap_or(ConfigSource::Default);

    Ok(ResolvedConfig {
        base_branch: Sourced::new(
            tagged.settings.base_branch.into_inner(),
            base_branch_source,
        ),
        branch_template: Sourced::new(
            tagged.settings.branch_template.into_inner(),
            branch_template_source,
        ),
        unmatched_files: Sourced::new(
            tagged.settings.unmatched_files.into_inner(),
            unmatched_files_source,
        ),
        default_commit_type: Sourced::new(
            tagged.settings.default_commit_type.map(|t| t.into_inner()),
            default_commit_type_source,
        ),
        components: tagged.components,
    })
}

/// Configuration warnings (non-fatal).
#[derive(Debug)]
pub struct ConfigWarning {
    pub message: String,
}

/// Validate the resolved (merged) configuration for cross-source consistency.
pub fn validate_resolved(config: &ResolvedConfig) -> Vec<ConfigWarning> {
    let mut warnings = Vec::new();

    if config.components.is_empty() {
        warnings.push(ConfigWarning {
            message: "no components defined — create .atomic.toml with [[components]] or run git-atomic init".into(),
        });
    }

    if !config.branch_template.value.contains("{component}") {
        warnings.push(ConfigWarning {
            message: format!(
                "branch_template {:?} does not contain {{component}} placeholder",
                config.branch_template.value
            ),
        });
    }

    if config.base_branch.value.is_empty() {
        warnings.push(ConfigWarning {
            message: "base_branch is empty".into(),
        });
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_when_no_file() {
        figment::Jail::expect_with(|_jail| {
            let resolved =
                load_layered_config(None, Path::new("nonexistent.toml")).unwrap();
            assert_eq!(resolved.base_branch.value, "main");
            assert_eq!(resolved.base_branch.source, ConfigSource::Default);
            assert!(resolved.components.is_empty());
            Ok(())
        });
    }

    #[test]
    fn file_overrides_defaults() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(
                ".atomic.toml",
                r#"
[settings]
base_branch = "develop"

[[components]]
name = "app"
globs = ["src/**"]
"#,
            )?;

            let resolved =
                load_layered_config(None, Path::new(".atomic.toml")).unwrap();
            assert_eq!(resolved.base_branch.value, "develop");
            assert_eq!(resolved.base_branch.source, ConfigSource::File);
            assert_eq!(resolved.components.len(), 1);

            Ok(())
        });
    }

    #[test]
    fn env_overrides_file() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(
                ".atomic.toml",
                r#"
[settings]
base_branch = "develop"

[[components]]
name = "app"
globs = ["src/**"]
"#,
            )?;

            jail.set_env("GIT_ATOMIC_BASE_BRANCH", "staging");

            let resolved =
                load_layered_config(None, Path::new(".atomic.toml")).unwrap();
            assert_eq!(resolved.base_branch.value, "staging");
            assert_eq!(resolved.base_branch.source, ConfigSource::Env);

            Ok(())
        });
    }

    #[test]
    fn duplicate_component_names_rejected() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(
                ".atomic.toml",
                r#"
[[components]]
name = "app"
globs = ["src/**"]

[[components]]
name = "app"
globs = ["lib/**"]
"#,
            )?;

            let err = load_layered_config(None, Path::new(".atomic.toml")).unwrap_err();
            assert!(matches!(err, ConfigError::Invalid { .. }));
            Ok(())
        });
    }

    #[test]
    fn component_order_preserved() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(
                ".atomic.toml",
                r#"
[[components]]
name = "zebra"
globs = ["z/**"]

[[components]]
name = "alpha"
globs = ["a/**"]
"#,
            )?;

            let resolved =
                load_layered_config(None, Path::new(".atomic.toml")).unwrap();
            assert_eq!(resolved.components[0].name, "zebra");
            assert_eq!(resolved.components[1].name, "alpha");
            Ok(())
        });
    }

    #[test]
    fn validate_resolved_warns_on_bad_template() {
        let resolved = ResolvedConfig {
            base_branch: Sourced::new("main".into(), ConfigSource::Default),
            branch_template: Sourced::new("bad-template".into(), ConfigSource::Default),
            unmatched_files: Sourced::new(UnmatchedPolicy::Error, ConfigSource::Default),
            default_commit_type: Sourced::new(None, ConfigSource::Default),
            components: vec![Component {
                name: "app".into(),
                globs: vec!["src/**".into()],
                commit_type: None,
                branch: None,
            }],
        };

        let warnings = validate_resolved(&resolved);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].message.contains("{component}"));
    }

    #[test]
    fn validate_resolved_warns_on_no_components() {
        let resolved = ResolvedConfig {
            base_branch: Sourced::new("main".into(), ConfigSource::Default),
            branch_template: Sourced::new("atomic/{component}".into(), ConfigSource::Default),
            unmatched_files: Sourced::new(UnmatchedPolicy::Error, ConfigSource::Default),
            default_commit_type: Sourced::new(None, ConfigSource::Default),
            components: vec![],
        };

        let warnings = validate_resolved(&resolved);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].message.contains("no components defined"));
    }

    #[test]
    fn to_config_bridges_correctly() {
        let resolved = ResolvedConfig {
            base_branch: Sourced::new("develop".into(), ConfigSource::File),
            branch_template: Sourced::new("atomic/{component}".into(), ConfigSource::Default),
            unmatched_files: Sourced::new(UnmatchedPolicy::Warn, ConfigSource::GitConfig),
            default_commit_type: Sourced::new(Some("feat".into()), ConfigSource::Env),
            components: vec![Component {
                name: "app".into(),
                globs: vec!["src/**".into()],
                commit_type: None,
                branch: None,
            }],
        };

        let config = resolved.to_config();
        assert_eq!(config.settings.base_branch, "develop");
        assert_eq!(config.settings.unmatched_files, UnmatchedPolicy::Warn);
        assert_eq!(config.components.len(), 1);
        assert_eq!(config.components[0].name, "app");
    }

    #[test]
    fn validate_resolved_warns_on_empty_base_branch() {
        let resolved = ResolvedConfig {
            base_branch: Sourced::new("".into(), ConfigSource::File),
            branch_template: Sourced::new("atomic/{component}".into(), ConfigSource::Default),
            unmatched_files: Sourced::new(UnmatchedPolicy::Error, ConfigSource::Default),
            default_commit_type: Sourced::new(None, ConfigSource::Default),
            components: vec![Component {
                name: "app".into(),
                globs: vec!["src/**".into()],
                commit_type: None,
                branch: None,
            }],
        };

        let warnings = validate_resolved(&resolved);
        assert!(warnings.iter().any(|w| w.message.contains("base_branch is empty")));
    }

    #[test]
    fn env_overrides_branch_template() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(
                ".atomic.toml",
                r#"
[[components]]
name = "app"
globs = ["src/**"]
"#,
            )?;

            jail.set_env("GIT_ATOMIC_BRANCH_TEMPLATE", "custom/{component}");

            let resolved =
                load_layered_config(None, Path::new(".atomic.toml")).unwrap();
            assert_eq!(resolved.branch_template.value, "custom/{component}");
            assert_eq!(resolved.branch_template.source, ConfigSource::Env);

            Ok(())
        });
    }
}
