use dcbor::prelude::*;

use crate::{
    Quantifier,
    pattern::{Matcher, Path, Pattern, vm::Instr},
};

/// A pattern that matches with repetition using a quantifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepeatPattern {
    pattern: Box<Pattern>,
    quantifier: Quantifier,
}

impl RepeatPattern {
    /// Creates a new `RepeatPattern` with the specified sub-pattern and
    /// quantifier.
    pub fn repeat(pattern: Pattern, quantifier: Quantifier) -> Self {
        RepeatPattern { pattern: Box::new(pattern), quantifier }
    }

    /// Creates a new `RepeatPattern` with a quantifier that matches exactly
    /// once.
    pub fn new(pattern: Pattern) -> Self {
        RepeatPattern {
            pattern: Box::new(pattern),
            quantifier: Quantifier::default(),
        }
    }

    /// Returns the sub-pattern of this repeat pattern.
    pub fn pattern(&self) -> &Pattern { &self.pattern }

    /// Returns the quantifier of this repeat pattern.
    pub fn quantifier(&self) -> &Quantifier { &self.quantifier }
}

impl Matcher for RepeatPattern {
    fn paths(&self, cbor: &CBOR) -> Vec<Path> {
        // For repeat patterns, we need to handle the quantifier logic
        // This is a simplified implementation that doesn't use the full VM
        // capability

        let inner_paths = self.pattern.paths(cbor);
        let matches = !inner_paths.is_empty();

        if matches {
            // Inner pattern matches
            if self.quantifier.contains(1) {
                inner_paths
            } else {
                vec![]
            }
        } else {
            // Inner pattern doesn't match
            if self.quantifier.contains(0) {
                // Zero matches are allowed, so succeed
                vec![vec![cbor.clone()]]
            } else {
                // Zero matches not allowed, so fail
                vec![]
            }
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        // Emit a high-level `Repeat` instruction for the VM
        let idx = literals.len();
        literals.push((*self.pattern).clone());
        code.push(Instr::Repeat { pat_idx: idx, quantifier: self.quantifier });
    }
}

impl std::fmt::Display for RepeatPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_range = self.quantifier.to_string();
        write!(f, "({}){}", self.pattern, formatted_range)
    }
}
