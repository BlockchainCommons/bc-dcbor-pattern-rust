use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// A pattern that matches if all contained patterns match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AndPattern(Vec<Pattern>);

impl AndPattern {
    /// Creates a new `AndPattern` with the given patterns.
    pub fn new(patterns: Vec<Pattern>) -> Self { AndPattern(patterns) }

    /// Returns the patterns contained in this AND pattern.
    pub fn patterns(&self) -> &[Pattern] { &self.0 }
}

impl Matcher for AndPattern {
    fn paths(&self, cbor: &CBOR) -> Vec<Path> {
        if self.patterns().iter().all(|pattern| pattern.matches(cbor)) {
            vec![vec![cbor.clone()]]
        } else {
            vec![]
        }
    }

    fn paths_with_captures(
        &self,
        cbor: &dcbor::CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        // For AND patterns, all patterns must match, and we merge captures
        let mut all_captures = std::collections::HashMap::new();

        for pattern in self.patterns() {
            let (paths, captures) = pattern.paths_with_captures(cbor);
            if paths.is_empty() {
                // If any pattern fails to match, AND fails
                return (vec![], std::collections::HashMap::new());
            }

            // Merge captures
            for (name, capture_paths) in captures {
                all_captures
                    .entry(name)
                    .or_insert_with(Vec::new)
                    .extend(capture_paths);
            }
        }

        // If all patterns matched, return the basic path and merged captures
        (vec![vec![cbor.clone()]], all_captures)
    }

    /// Compile into byte-code (AND = all must match).
    fn compile(
        &self,
        code: &mut Vec<Instr>,
        lits: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        // Each pattern must match at this position
        for pattern in self.patterns() {
            pattern.compile(code, lits, captures);
        }
    }

    fn collect_capture_names(&self, names: &mut Vec<String>) {
        // Collect captures from all patterns
        for pattern in self.patterns() {
            pattern.collect_capture_names(names);
        }
    }

    fn is_complex(&self) -> bool {
        // The pattern is complex if it contains more than one pattern, or if
        // the one pattern is complex itself.
        self.patterns().len() > 1
            || self.patterns().iter().any(|p| p.is_complex())
    }
}

impl std::fmt::Display for AndPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.patterns()
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join("&")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_pattern_display() {
        let pattern1 = Pattern::number_greater_than(5);
        let pattern2 = Pattern::number_less_than(10);
        let and_pattern = AndPattern::new(vec![pattern1, pattern2]);
        assert_eq!(and_pattern.to_string(), ">5&<10");
    }

    #[test]
    fn test_and_pattern_matches_when_all_patterns_match() {
        let pattern = AndPattern::new(vec![
            Pattern::number_greater_than(5),
            Pattern::number_less_than(10),
        ]);

        let cbor_7 = CBOR::from(7);
        assert!(pattern.matches(&cbor_7));
    }

    #[test]
    fn test_and_pattern_fails_when_any_pattern_fails() {
        let pattern = AndPattern::new(vec![
            Pattern::number_greater_than(5),
            Pattern::number_less_than(10),
        ]);

        let cbor_12 = CBOR::from(12); // > 10, so second pattern fails
        assert!(!pattern.matches(&cbor_12));

        let cbor_3 = CBOR::from(3); // < 5, so first pattern fails
        assert!(!pattern.matches(&cbor_3));
    }

    #[test]
    fn test_and_pattern_empty_returns_true() {
        let pattern = AndPattern::new(vec![]);
        let cbor = CBOR::from("any value");
        assert!(pattern.matches(&cbor));
    }
}
