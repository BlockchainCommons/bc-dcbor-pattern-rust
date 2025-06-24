use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// A pattern that matches with repetition (stub implementation).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepeatPattern;

impl RepeatPattern {
    /// Creates a new `RepeatPattern` (placeholder).
    pub fn new() -> Self { RepeatPattern }
}

impl Default for RepeatPattern {
    fn default() -> Self { Self::new() }
}

impl Matcher for RepeatPattern {
    fn paths(&self, _cbor: &CBOR) -> Vec<Path> {
        // TODO: Implement repeat pattern matching
        unimplemented!("RepeatPattern not yet implemented")
    }

    fn compile(
        &self,
        _code: &mut Vec<Instr>,
        _literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        // TODO: Implement repeat pattern compilation
        unimplemented!("RepeatPattern compile not yet implemented")
    }
}

impl std::fmt::Display for RepeatPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "REPEAT(TODO)")
    }
}
