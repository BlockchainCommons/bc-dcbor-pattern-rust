use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching null values in dCBOR.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct NullPattern;

impl NullPattern {
    /// Creates a new `NullPattern` that matches null values.
    pub fn new() -> Self { NullPattern }
}

impl Default for NullPattern {
    fn default() -> Self { NullPattern::new() }
}

impl Matcher for NullPattern {
    fn paths(&self, cbor: &CBOR) -> Vec<Path> {
        if cbor.is_null() {
            vec![vec![cbor.clone()]]
        } else {
            vec![]
        }
    }

    fn compile(
        &self,
        _code: &mut Vec<Instr>,
        _literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        // TODO: Implement VM compilation when VM is ready
        unimplemented!("NullPattern::compile not yet implemented");
    }
}

impl std::fmt::Display for NullPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NULL")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_pattern_matching() {
        let null_cbor = CBOR::null();
        let pattern = NullPattern::new();
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
        assert_eq!(NullPattern::new().to_string(), "NULL");
    }

    #[test]
    fn test_null_pattern_matches() {
        let pattern = NullPattern::new();
        let null_cbor = CBOR::null();
        assert!(pattern.matches(&null_cbor));

        let text_cbor = "test".to_cbor();
        assert!(!pattern.matches(&text_cbor));

        let bool_cbor = true.to_cbor();
        assert!(!pattern.matches(&bool_cbor));
    }
}
