use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Root configuration loaded from `.atomic.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Config {
    #[serde(default)]
    pub settings: Settings,

    /// Ordered map of component name → definition.
    /// Insertion order determines match priority (first-match-wins per ADR-003).
    #[schemars(with = "std::collections::HashMap<String, Component>")]
    pub components: IndexMap<String, Component>,
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

/// A single component definition.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Component {
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
        let mut components = IndexMap::new();
        components.insert(
            "frontend".into(),
            Component {
                globs: vec!["src/ui/**".into(), "src/components/**".into()],
                commit_type: None,
                branch: None,
            },
        );
        components.insert(
            "backend".into(),
            Component {
                globs: vec!["src/api/**".into(), "src/db/**".into()],
                commit_type: Some("fix".into()),
                branch: None,
            },
        );
        Self {
            settings: Settings::default(),
            components,
        }
    }
}
