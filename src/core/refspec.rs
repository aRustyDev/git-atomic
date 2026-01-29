/// Parsed ref specification: either a single ref or a two-dot range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefSpec {
    /// A single commit reference (e.g. "HEAD", "abc123", "HEAD~3").
    Single(String),
    /// A range of commits using `A..B` syntax.
    Range { start: String, end: String },
}

impl RefSpec {
    /// Parse a ref string into a `RefSpec`.
    ///
    /// - Contains `...` → error (triple-dot not supported)
    /// - Contains `..` → `Range` (empty sides default to HEAD)
    /// - Otherwise → `Single`
    pub fn parse(input: &str) -> Result<Self, String> {
        if input.contains("...") {
            return Err("triple-dot syntax is not supported; use `A..B`".into());
        }
        match input.split_once("..") {
            Some((start, end)) => {
                let start = if start.is_empty() { "HEAD" } else { start };
                let end = if end.is_empty() { "HEAD" } else { end };
                Ok(RefSpec::Range {
                    start: start.to_string(),
                    end: end.to_string(),
                })
            }
            None => Ok(RefSpec::Single(input.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_ref() {
        assert_eq!(RefSpec::parse("HEAD").unwrap(), RefSpec::Single("HEAD".into()));
        assert_eq!(RefSpec::parse("abc123").unwrap(), RefSpec::Single("abc123".into()));
        assert_eq!(RefSpec::parse("HEAD~3").unwrap(), RefSpec::Single("HEAD~3".into()));
    }

    #[test]
    fn parse_range() {
        assert_eq!(
            RefSpec::parse("main..feature").unwrap(),
            RefSpec::Range { start: "main".into(), end: "feature".into() }
        );
        assert_eq!(
            RefSpec::parse("HEAD~3..HEAD").unwrap(),
            RefSpec::Range { start: "HEAD~3".into(), end: "HEAD".into() }
        );
    }

    #[test]
    fn parse_empty_sides_default_to_head() {
        assert_eq!(
            RefSpec::parse("..feature").unwrap(),
            RefSpec::Range { start: "HEAD".into(), end: "feature".into() }
        );
        assert_eq!(
            RefSpec::parse("main..").unwrap(),
            RefSpec::Range { start: "main".into(), end: "HEAD".into() }
        );
        assert_eq!(
            RefSpec::parse("..").unwrap(),
            RefSpec::Range { start: "HEAD".into(), end: "HEAD".into() }
        );
    }

    #[test]
    fn parse_triple_dot_errors() {
        let err = RefSpec::parse("main...feature").unwrap_err();
        assert!(err.contains("triple-dot"));
    }
}
