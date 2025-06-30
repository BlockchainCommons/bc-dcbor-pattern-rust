use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// A pattern that negates another pattern; matches when the inner pattern does
/// not match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotPattern(Box<Pattern>);

impl NotPattern {
    /// Creates a new `NotPattern` with the given pattern.
    pub fn new(pattern: Pattern) -> Self { NotPattern(Box::new(pattern)) }

    /// Returns the pattern being negated.
    pub fn pattern(&self) -> &Pattern { &self.0 }
}

impl Matcher for NotPattern {
    fn paths(&self, cbor: &CBOR) -> Vec<Path> {
        // If the inner pattern doesn't match, then we return the current
        // CBOR value as a match
        if !self.pattern().matches(cbor) {
            vec![vec![cbor.clone()]]
        } else {
            vec![]
        }
    }

    fn paths_with_captures(
        &self,
        cbor: &dcbor::CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        // For NOT patterns, we match if the inner pattern does NOT match
        let (inner_paths, _inner_captures) =
            self.pattern().paths_with_captures(cbor);
        if inner_paths.is_empty() {
            // Inner pattern doesn't match, so NOT matches
            (vec![vec![cbor.clone()]], std::collections::HashMap::new())
        } else {
            // Inner pattern matches, so NOT doesn't match
            (vec![], std::collections::HashMap::new())
        }
    }

    /// Compile into byte-code (NOT = negation of the inner pattern).
    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        // NOT = check that pattern doesn't match
        let idx = literals.len();
        literals.push(self.pattern().clone());
        code.push(Instr::NotMatch { pat_idx: idx });
    }

    fn collect_capture_names(&self, names: &mut Vec<String>) {
        // NOT patterns do not expose their inner pattern's captures
        // since the semantics would be unclear - what does it mean to
        // capture from a pattern that must NOT match?
        // So we do nothing here.
        let _ = names; // Suppress unused warning
    }

    fn is_complex(&self) -> bool {
        // NOT patterns are always considered complex for display purposes
        true
    }
}

impl std::fmt::Display for NotPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.pattern().is_complex() {
            write!(f, "!({})", self.pattern())
        } else {
            write!(f, "!{}", self.pattern())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_pattern_display() {
        let not_pattern = NotPattern::new(Pattern::number(5));
        assert_eq!(not_pattern.to_string(), "!5");
    }

    #[test]
    fn test_not_pattern_display_complex() {
        let and_pattern =
            Pattern::Meta(crate::pattern::meta::MetaPattern::And(
                crate::pattern::meta::AndPattern::new(vec![
                    Pattern::number(5),
                    Pattern::text("hello"),
                ]),
            ));
        let not_pattern = NotPattern::new(and_pattern);
        assert_eq!(not_pattern.to_string(), r#"!(5 & "hello")"#);
    }

    #[test]
    fn test_not_pattern_matches_when_inner_fails() {
        let pattern = NotPattern::new(Pattern::number(5));

        let cbor_42 = CBOR::from(42); // Not 5, so NOT pattern should match
        assert!(pattern.matches(&cbor_42));

        let cbor_text = CBOR::from("hello"); // Not a number, so NOT pattern should match
        assert!(pattern.matches(&cbor_text));
    }

    #[test]
    fn test_not_pattern_fails_when_inner_matches() {
        let pattern = NotPattern::new(Pattern::number(5));

        let cbor_5 = CBOR::from(5); // Exactly 5, so NOT pattern should fail
        assert!(!pattern.matches(&cbor_5));
    }
}
