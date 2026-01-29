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

    #[test]
    fn parse_unicode_refs() {
        // Emoji ref
        assert_eq!(
            RefSpec::parse("\u{1F680}").unwrap(),
            RefSpec::Single("\u{1F680}".into())
        );
        // CJK characters
        assert_eq!(
            RefSpec::parse("\u{4E16}\u{754C}").unwrap(),
            RefSpec::Single("\u{4E16}\u{754C}".into())
        );
        // Unicode range
        assert_eq!(
            RefSpec::parse("\u{1F680}..\u{4E16}\u{754C}").unwrap(),
            RefSpec::Range {
                start: "\u{1F680}".into(),
                end: "\u{4E16}\u{754C}".into(),
            }
        );
    }

    #[test]
    fn parse_special_chars() {
        // Spaces are preserved as-is (git will reject, but parser doesn't)
        assert_eq!(
            RefSpec::parse("my branch").unwrap(),
            RefSpec::Single("my branch".into())
        );
        // Backslashes
        assert_eq!(
            RefSpec::parse("refs\\heads\\main").unwrap(),
            RefSpec::Single("refs\\heads\\main".into())
        );
    }

    #[test]
    fn parse_whitespace_only() {
        assert_eq!(
            RefSpec::parse("   ").unwrap(),
            RefSpec::Single("   ".into())
        );
        assert_eq!(
            RefSpec::parse("\t").unwrap(),
            RefSpec::Single("\t".into())
        );
    }

    #[test]
    fn parse_very_long_string() {
        let long = "a".repeat(250);
        assert_eq!(
            RefSpec::parse(&long).unwrap(),
            RefSpec::Single(long.clone())
        );
        // Long range
        let long_range = format!("{}..{}", "b".repeat(200), "c".repeat(200));
        assert_eq!(
            RefSpec::parse(&long_range).unwrap(),
            RefSpec::Range {
                start: "b".repeat(200),
                end: "c".repeat(200),
            }
        );
    }

    #[test]
    fn parse_empty_string() {
        assert_eq!(
            RefSpec::parse("").unwrap(),
            RefSpec::Single("".into())
        );
    }
}
