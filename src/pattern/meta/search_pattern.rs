use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// A pattern that searches the entire dCBOR tree for matches (stub
/// implementation).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchPattern;

impl SearchPattern {
    /// Creates a new `SearchPattern` (placeholder).
    pub fn new() -> Self { SearchPattern }
}

impl Default for SearchPattern {
    fn default() -> Self { Self::new() }
}

impl Matcher for SearchPattern {
    fn paths(&self, _cbor: &CBOR) -> Vec<Path> {
        // TODO: Implement search pattern matching
        unimplemented!("SearchPattern not yet implemented")
    }

    fn compile(
        &self,
        _code: &mut Vec<Instr>,
        _literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        // TODO: Implement search pattern compilation
        unimplemented!("SearchPattern compile not yet implemented")
    }
}

impl std::fmt::Display for SearchPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SEARCH(TODO)")
    }
}
