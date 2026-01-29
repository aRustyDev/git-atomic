use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Root configuration loaded from `.atomic.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Config {
    #[serde(default)]
    pub settings: Settings,

    /// Ordered list of component definitions.
    /// Order determines match priority (first-match-wins per ADR-003).
    /// Order guaranteed by TOML array-of-tables spec (ADR-007).
    #[serde(default)]
    pub components: Vec<Component>,
}

/// Global settings with defaults.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Settings {
    /// Base branch for atomic branches.
    #[serde(default = "default_base_branch")]
    pub base_branch: String,

    /// Branch naming template. `{component}` is replaced at runtime.
    #[serde(default = "default_branch_template")]
    pub branch_template: String,

    /// Policy when files match no component.
    #[serde(default)]
    pub unmatched_files: UnmatchedPolicy,

    /// Default conventional-commit type.
    pub default_commit_type: Option<String>,
}

/// Policy for files that match no component glob.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum UnmatchedPolicy {
    /// Fail with exit code 4 (default).
    #[default]
    Error,
    /// Log a warning but continue.
    Warn,
    /// Silently skip unmatched files.
    Ignore,
}

impl FromStr for UnmatchedPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "error" => Ok(UnmatchedPolicy::Error),
            "warn" => Ok(UnmatchedPolicy::Warn),
            "ignore" => Ok(UnmatchedPolicy::Ignore),
            other => Err(format!(
                "invalid unmatched_files policy: {other:?} (expected \"error\", \"warn\", or \"ignore\")"
            )),
        }
    }
}

impl fmt::Display for UnmatchedPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnmatchedPolicy::Error => f.write_str("error"),
            UnmatchedPolicy::Warn => f.write_str("warn"),
            UnmatchedPolicy::Ignore => f.write_str("ignore"),
        }
    }
}

/// A single component definition.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Component {
    /// Component name (previously the map key in `[components.<name>]`).
    pub name: String,

    /// Glob patterns that claim files for this component.
    pub globs: Vec<String>,

    /// Override the conventional-commit type for this component.
    pub commit_type: Option<String>,

    /// Override the branch name for this component.
    pub branch: Option<String>,
}

fn default_base_branch() -> String {
    "main".into()
}

fn default_branch_template() -> String {
    "atomic/{component}".into()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            base_branch: default_base_branch(),
            branch_template: default_branch_template(),
            unmatched_files: UnmatchedPolicy::default(),
            default_commit_type: None,
        }
    }
}

impl Config {
    /// Build a sample config with default settings and example components.
    /// Used by `init` to generate `.atomic.toml` from the real types.
    pub fn sample() -> Self {
        Self {
            settings: Settings::default(),
            components: vec![
                Component {
                    name: "frontend".into(),
                    globs: vec!["src/ui/**".into(), "src/components/**".into()],
                    commit_type: None,
                    branch: None,
                },
                Component {
                    name: "backend".into(),
                    globs: vec!["src/api/**".into(), "src/db/**".into()],
                    commit_type: Some("fix".into()),
                    branch: None,
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unmatched_policy_from_str() {
        assert_eq!(UnmatchedPolicy::from_str("error").unwrap(), UnmatchedPolicy::Error);
        assert_eq!(UnmatchedPolicy::from_str("WARN").unwrap(), UnmatchedPolicy::Warn);
        assert_eq!(UnmatchedPolicy::from_str("Ignore").unwrap(), UnmatchedPolicy::Ignore);
        assert!(UnmatchedPolicy::from_str("invalid").is_err());
    }

    #[test]
    fn unmatched_policy_display() {
        assert_eq!(UnmatchedPolicy::Error.to_string(), "error");
        assert_eq!(UnmatchedPolicy::Warn.to_string(), "warn");
        assert_eq!(UnmatchedPolicy::Ignore.to_string(), "ignore");
    }

    #[test]
    fn config_serde_round_trip() {
        let original = Config::sample();
        let toml_str = toml::to_string(&original).expect("serialize to TOML");
        let deserialized: Config = toml::from_str(&toml_str).expect("deserialize from TOML");

        assert_eq!(deserialized.settings.base_branch, original.settings.base_branch);
        assert_eq!(deserialized.settings.branch_template, original.settings.branch_template);
        assert_eq!(deserialized.settings.unmatched_files, original.settings.unmatched_files);
        assert_eq!(deserialized.settings.default_commit_type, original.settings.default_commit_type);
        assert_eq!(deserialized.components.len(), original.components.len());
        for (d, o) in deserialized.components.iter().zip(original.components.iter()) {
            assert_eq!(d.name, o.name);
            assert_eq!(d.globs, o.globs);
            assert_eq!(d.commit_type, o.commit_type);
            assert_eq!(d.branch, o.branch);
        }
    }

    #[test]
    fn unmatched_policy_serde_round_trip() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Wrapper {
            policy: UnmatchedPolicy,
        }
        for policy in [UnmatchedPolicy::Error, UnmatchedPolicy::Warn, UnmatchedPolicy::Ignore] {
            let w = Wrapper { policy: policy.clone() };
            let serialized = toml::to_string(&w).expect("serialize");
            let deserialized: Wrapper = toml::from_str(&serialized).expect("deserialize");
            assert_eq!(deserialized.policy, policy);
        }
    }

    #[test]
    fn empty_string_from_str_returns_error() {
        let result = UnmatchedPolicy::from_str("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid unmatched_files policy"));
    }
}
