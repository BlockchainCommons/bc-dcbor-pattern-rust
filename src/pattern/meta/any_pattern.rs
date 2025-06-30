use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// A pattern that always matches any CBOR value.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AnyPattern;

impl AnyPattern {
    /// Creates a new `AnyPattern`.
    pub fn new() -> Self { AnyPattern }
}

impl Default for AnyPattern {
    fn default() -> Self { Self::new() }
}

impl Matcher for AnyPattern {
    fn paths(&self, cbor: &CBOR) -> Vec<Path> {
        // Always matches - return the current CBOR value as a path
        vec![vec![cbor.clone()]]
    }

    fn paths_with_captures(
        &self,
        cbor: &dcbor::CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        // AnyPattern has no internal captures, so just return paths and empty captures
        (self.paths(cbor), std::collections::HashMap::new())
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        _literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        // Any pattern always matches - just save the current path
        code.push(Instr::Save);
    }

    fn collect_capture_names(&self, _names: &mut Vec<String>) {
        // AnyPattern doesn't contain captures
    }
}

impl std::fmt::Display for AnyPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_pattern_display() {
        let pattern = AnyPattern::new();
        assert_eq!(pattern.to_string(), "*");
    }

    #[test]
    fn test_any_pattern_matches_everything() {
        let pattern = AnyPattern::new();

        // Should match all kinds of CBOR values
        assert!(pattern.matches(&CBOR::from(42)));
        assert!(pattern.matches(&CBOR::from("hello")));
        assert!(pattern.matches(&CBOR::from(true)));
        assert!(pattern.matches(&CBOR::from(vec![1, 2, 3])));
        assert!(pattern.matches(&CBOR::null()));
    }

    #[test]
    fn test_any_pattern_paths() {
        let pattern = AnyPattern::new();
        let cbor = CBOR::from("test");
        let paths = pattern.paths(&cbor);

        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec![cbor]);
    }
}
