use git_atomic::core::refspec::RefSpec;
use proptest::prelude::*;

proptest! {
    #[test]
    fn refspec_parse_never_panics(s in "\\PC{0,100}") {
        let _ = RefSpec::parse(&s);
    }

    #[test]
    fn refspec_single_roundtrip(ref_name in "[a-zA-Z][a-zA-Z0-9_/-]{0,50}") {
        // No ".." means Single
        if !ref_name.contains("..") {
            let spec = RefSpec::parse(&ref_name).unwrap();
            assert!(matches!(spec, RefSpec::Single(_)));
        }
    }

    #[test]
    fn refspec_range_has_two_parts(
        start in "[a-zA-Z][a-zA-Z0-9]{0,20}",
        end in "[a-zA-Z][a-zA-Z0-9]{0,20}"
    ) {
        let input = format!("{start}..{end}");
        let spec = RefSpec::parse(&input).unwrap();
        match spec {
            RefSpec::Range { start: s, end: e } => {
                assert_eq!(s, start);
                assert_eq!(e, end);
            }
            _ => panic!("expected Range"),
        }
    }

    #[test]
    fn triple_dot_always_errors(
        start in "[a-zA-Z]{0,10}",
        end in "[a-zA-Z]{0,10}"
    ) {
        let input = format!("{start}...{end}");
        assert!(RefSpec::parse(&input).is_err());
    }
}
