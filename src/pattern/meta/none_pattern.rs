use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// A pattern that never matches any CBOR value.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct NonePattern;

impl NonePattern {
    /// Creates a new `NonePattern`.
    pub fn new() -> Self { NonePattern }
}

impl Default for NonePattern {
    fn default() -> Self { Self::new() }
}

impl Matcher for NonePattern {
    fn paths(&self, _cbor: &CBOR) -> Vec<Path> {
        // Never matches - always return empty vector
        vec![]
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        _literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        // None pattern never matches - use Jump to invalid location to kill
        // thread
        code.push(Instr::Jump(usize::MAX));
    }

    fn collect_capture_names(&self, _names: &mut Vec<String>) {
        // NonePattern doesn't contain captures
    }

    fn paths_with_captures(
        &self,
        cbor: &dcbor::CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        // NonePattern has no internal captures, so just return paths and empty captures
        (self.paths(cbor), std::collections::HashMap::new())
    }
}

impl std::fmt::Display for NonePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NONE")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none_pattern_display() {
        let pattern = NonePattern::new();
        assert_eq!(pattern.to_string(), "NONE");
    }

    #[test]
    fn test_none_pattern_matches_nothing() {
        let pattern = NonePattern::new();

        // Should never match any kind of CBOR value
        assert!(!pattern.matches(&CBOR::from(42)));
        assert!(!pattern.matches(&CBOR::from("hello")));
        assert!(!pattern.matches(&CBOR::from(true)));
        assert!(!pattern.matches(&CBOR::from(vec![1, 2, 3])));
        assert!(!pattern.matches(&CBOR::null()));
    }

    #[test]
    fn test_none_pattern_paths() {
        let pattern = NonePattern::new();
        let cbor = CBOR::from("test");
        let paths = pattern.paths(&cbor);

        assert_eq!(paths.len(), 0);
    }
}
