use crate::config::Config;
use crate::core::ConfigError;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::Path;

/// Maps file paths to components using first-match-wins (ADR-003).
///
/// Each glob is tagged with its component index (position in the IndexMap).
/// When `GlobSet` returns multiple matches, the lowest index wins.
pub struct ComponentMatcher {
    glob_set: GlobSet,
    /// For each glob in the set: (component_index, component_name).
    glob_owners: Vec<(usize, String)>,
    /// Component names in config order.
    component_names: Vec<String>,
}

impl ComponentMatcher {
    /// Build a matcher from the loaded config.
    pub fn from_config(config: &Config) -> Result<Self, ConfigError> {
        let mut builder = GlobSetBuilder::new();
        let mut glob_owners = Vec::new();
        let mut component_names = Vec::new();

        for (idx, (name, component)) in config.components.iter().enumerate() {
            component_names.push(name.clone());
            for pattern in &component.globs {
                let glob = Glob::new(pattern).map_err(|e| ConfigError::InvalidGlob {
                    component: name.clone(),
                    pattern: pattern.clone(),
                    reason: e.to_string(),
                })?;
                builder.add(glob);
                glob_owners.push((idx, name.clone()));
            }
        }

        let glob_set = builder.build().map_err(|e| ConfigError::Invalid {
            reason: format!("failed to build glob set: {e}"),
        })?;

        Ok(Self {
            glob_set,
            glob_owners,
            component_names,
        })
    }

    /// Return the component name for a file path, or `None` if unmatched.
    pub fn match_file(&self, path: &Path) -> Option<&str> {
        let matches = self.glob_set.matches(path);
        matches
            .iter()
            .map(|&i| &self.glob_owners[i])
            .min_by_key(|(idx, _)| *idx)
            .map(|(_, name)| name.as_str())
    }

    /// Group a set of file paths by component. Returns `(grouped, unmatched)`.
    pub fn group_files<'a>(
        &self,
        paths: &'a [&Path],
    ) -> (Vec<(&str, Vec<&'a Path>)>, Vec<&'a Path>) {
        // One vec per component index.
        let mut buckets: Vec<Vec<&Path>> = vec![vec![]; self.component_names.len()];
        let mut unmatched = Vec::new();

        for &path in paths {
            match self.match_file(path) {
                Some(name) => {
                    let idx = self
                        .component_names
                        .iter()
                        .position(|n| n == name)
                        .unwrap();
                    buckets[idx].push(path);
                }
                None => unmatched.push(path),
            }
        }

        let grouped = self
            .component_names
            .iter()
            .zip(buckets)
            .filter(|(_, files)| !files.is_empty())
            .map(|(name, files)| (name.as_str(), files))
            .collect();

        (grouped, unmatched)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Component, Config, Settings};
    use indexmap::IndexMap;

    fn make_config(components: Vec<(&str, Vec<&str>)>) -> Config {
        let mut map = IndexMap::new();
        for (name, globs) in components {
            map.insert(
                name.to_string(),
                Component {
                    globs: globs.into_iter().map(String::from).collect(),
                    commit_type: None,
                    branch: None,
                },
            );
        }
        Config {
            settings: Settings::default(),
            components: map,
        }
    }

    #[test]
    fn exact_match() {
        let cfg = make_config(vec![("ui", vec!["src/ui/**"]), ("api", vec!["src/api/**"])]);
        let m = ComponentMatcher::from_config(&cfg).unwrap();
        assert_eq!(m.match_file(Path::new("src/ui/button.rs")), Some("ui"));
        assert_eq!(m.match_file(Path::new("src/api/handler.rs")), Some("api"));
    }

    #[test]
    fn no_match() {
        let cfg = make_config(vec![("ui", vec!["src/ui/**"])]);
        let m = ComponentMatcher::from_config(&cfg).unwrap();
        assert_eq!(m.match_file(Path::new("README.md")), None);
    }

    #[test]
    fn first_match_wins() {
        let cfg = make_config(vec![
            ("specific", vec!["src/**"]),
            ("catchall", vec!["**"]),
        ]);
        let m = ComponentMatcher::from_config(&cfg).unwrap();
        // src/foo matches both, but "specific" is first.
        assert_eq!(m.match_file(Path::new("src/foo.rs")), Some("specific"));
        // README only matches catchall.
        assert_eq!(m.match_file(Path::new("README.md")), Some("catchall"));
    }

    #[test]
    fn group_files_works() {
        let cfg = make_config(vec![
            ("ui", vec!["src/ui/**"]),
            ("api", vec!["src/api/**"]),
        ]);
        let m = ComponentMatcher::from_config(&cfg).unwrap();

        let paths: Vec<&Path> = vec![
            Path::new("src/ui/a.rs"),
            Path::new("src/api/b.rs"),
            Path::new("README.md"),
        ];
        let (grouped, unmatched) = m.group_files(&paths);

        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped[0].0, "ui");
        assert_eq!(grouped[1].0, "api");
        assert_eq!(unmatched, vec![Path::new("README.md")]);
    }

    #[test]
    fn catch_all_last() {
        let cfg = make_config(vec![
            ("core", vec!["src/core/**"]),
            ("_other", vec!["**"]),
        ]);
        let m = ComponentMatcher::from_config(&cfg).unwrap();
        assert_eq!(m.match_file(Path::new("src/core/lib.rs")), Some("core"));
        assert_eq!(m.match_file(Path::new("docs/readme.md")), Some("_other"));
    }
}
