use figment::value::{Dict, Map};
use figment::{Error, Metadata, Profile, Provider};

/// Custom figment `Provider` that reads `atomic.*` keys from git config.
///
/// Uses gix's `config_snapshot()` which automatically merges
/// system < global < local < worktree scopes. The merged result
/// is treated as a single "git config" source for provenance tracking.
pub struct GitConfigProvider {
    settings: Dict,
}

impl GitConfigProvider {
    /// Build provider from a gix repository. Returns empty provider if `None`.
    pub fn new(repo: Option<&gix::Repository>) -> Self {
        let mut settings = Dict::new();
        if let Some(repo) = repo {
            let snapshot = repo.config_snapshot();

            if let Some(v) = snapshot.string_by("atomic", None, "baseBranch") {
                settings.insert(
                    "base_branch".into(),
                    v.to_string().into(),
                );
            }
            if let Some(v) = snapshot.string_by("atomic", None, "branchTemplate") {
                settings.insert(
                    "branch_template".into(),
                    v.to_string().into(),
                );
            }
            if let Some(v) = snapshot.string_by("atomic", None, "unmatchedFiles") {
                settings.insert(
                    "unmatched_files".into(),
                    v.to_string().into(),
                );
            }
            if let Some(v) = snapshot.string_by("atomic", None, "defaultCommitType") {
                settings.insert(
                    "default_commit_type".into(),
                    v.to_string().into(),
                );
            }
        }
        Self { settings }
    }
}

impl Provider for GitConfigProvider {
    fn metadata(&self) -> Metadata {
        Metadata::named("git config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        // Nest settings under "settings" key to match Config struct
        let mut inner = Dict::new();
        if !self.settings.is_empty() {
            inner.insert("settings".into(), self.settings.clone().into());
        }
        Ok(Profile::Default.collect(inner))
    }
}
