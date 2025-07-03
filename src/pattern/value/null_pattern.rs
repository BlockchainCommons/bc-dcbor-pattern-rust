use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching null values in dCBOR.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default)]
pub struct NullPattern;

impl Matcher for NullPattern {
    fn paths(&self, haystack: &CBOR) -> Vec<Path> {
        if haystack.is_null() {
            vec![vec![haystack.clone()]]
        } else {
            vec![]
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        let idx = literals.len();
        literals.push(Pattern::Value(crate::pattern::ValuePattern::Null(
            self.clone(),
        )));
        code.push(Instr::MatchPredicate(idx));
    }
}

impl std::fmt::Display for NullPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_pattern_matching() {
        let null_cbor = CBOR::null();
        let pattern = NullPattern;
        let paths = pattern.paths(&null_cbor);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![null_cbor.clone()]);

        // Test with non-null CBOR
        let text_cbor = "test".to_cbor();
        let paths = pattern.paths(&text_cbor);
        assert!(paths.is_empty());

        let number_cbor = 42.to_cbor();
        let paths = pattern.paths(&number_cbor);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_null_pattern_display() {
        assert_eq!(NullPattern.to_string(), "null");
    }

    #[test]
    fn test_null_pattern_matches() {
        let pattern = NullPattern;
        let null_cbor = CBOR::null();
        assert!(pattern.matches(&null_cbor));

        let text_cbor = "test".to_cbor();
        assert!(!pattern.matches(&text_cbor));

        let bool_cbor = true.to_cbor();
        assert!(!pattern.matches(&bool_cbor));
    }
}
