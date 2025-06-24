use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// A pattern that captures matches (stub implementation).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturePattern;

impl CapturePattern {
    /// Creates a new `CapturePattern` (placeholder).
    pub fn new() -> Self {
        CapturePattern
    }
}

impl Default for CapturePattern {
    fn default() -> Self {
        Self::new()
    }
}

impl Matcher for CapturePattern {
    fn paths(&self, _cbor: &CBOR) -> Vec<Path> {
        // TODO: Implement capture pattern matching
        unimplemented!("CapturePattern not yet implemented")
    }

    fn compile(
        &self,
        _code: &mut Vec<Instr>,
        _literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        // TODO: Implement capture pattern compilation
        unimplemented!("CapturePattern compile not yet implemented")
    }
}

impl std::fmt::Display for CapturePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CAPTURE(TODO)")
    }
}
