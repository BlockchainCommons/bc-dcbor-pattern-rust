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
    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// Returns the quantifier of this repeat pattern.
    pub fn quantifier(&self) -> &Quantifier {
        &self.quantifier
    }
}

impl Matcher for RepeatPattern {
    fn paths(&self, haystack: &CBOR) -> Vec<Path> {
        // For repeat patterns, we need to handle the quantifier logic
        // This is a simplified implementation that doesn't use the full VM
        // capability

        let inner_paths = self.pattern.paths(haystack);
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
                vec![vec![haystack.clone()]]
            } else {
                // Zero matches not allowed, so fail
                vec![]
            }
        }
    }

    fn paths_with_captures(
        &self,
        haystack: &CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        // Check if the inner pattern has any captures
        let mut capture_names = Vec::new();
        self.pattern.collect_capture_names(&mut capture_names);

        if capture_names.is_empty() {
            // No captures in the inner pattern, use basic implementation
            return (self.paths(haystack), std::collections::HashMap::new());
        }

        // For patterns with captures, we need to handle different quantifier cases
        match haystack.as_case() {
            CBORCase::Array(arr) => {
                // For array inputs, the repeat pattern should match against elements
                let mut all_captures = std::collections::HashMap::new();
                let mut valid_match = false;

                // Check if this pattern can match the array length
                if self.quantifier.contains(arr.len()) {
                    valid_match = true;

                    // If minimum is 0 and array is empty, we have an empty capture
                    if arr.is_empty() && self.quantifier.contains(0) {
                        // For empty arrays, captures should be empty but present
                        for name in &capture_names {
                            all_captures.insert(name.clone(), vec![]);
                        }
                    } else {
                        // For non-empty arrays, collect captures from each element
                        for element in arr {
                            let (_element_paths, element_captures) =
                                self.pattern.paths_with_captures(element);

                            for (capture_name, captured_paths) in
                                element_captures
                            {
                                all_captures
                                    .entry(capture_name)
                                    .or_insert_with(Vec::new)
                                    .extend(captured_paths);
                            }
                        }
                    }
                }

                if valid_match {
                    (vec![vec![haystack.clone()]], all_captures)
                } else {
                    (vec![], std::collections::HashMap::new())
                }
            }
            _ => {
                // For non-array inputs, use the basic repeat logic
                let inner_paths = self.pattern.paths(haystack);
                let matches = !inner_paths.is_empty();

                if matches && self.quantifier.contains(1) {
                    // Inner pattern matches and quantifier allows 1 match
                    let (_paths, captures) =
                        self.pattern.paths_with_captures(haystack);
                    (vec![vec![haystack.clone()]], captures)
                } else if !matches && self.quantifier.contains(0) {
                    // Inner pattern doesn't match but quantifier allows 0 matches
                    let mut empty_captures = std::collections::HashMap::new();
                    for name in &capture_names {
                        empty_captures.insert(name.clone(), vec![]);
                    }
                    (vec![vec![haystack.clone()]], empty_captures)
                } else {
                    // No valid match
                    (vec![], std::collections::HashMap::new())
                }
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

    fn collect_capture_names(&self, names: &mut Vec<String>) {
        // Collect captures from the repeated pattern
        self.pattern.collect_capture_names(names);
    }
}

impl std::fmt::Display for RepeatPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_range = self.quantifier.to_string();
        write!(f, "({}){}", self.pattern, formatted_range)
    }
}
