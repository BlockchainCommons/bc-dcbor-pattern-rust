use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching boolean values in dCBOR.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum BoolPattern {
    /// Matches any boolean value.
    Any,
    /// Matches the specific boolean value.
    Value(bool),
}

impl BoolPattern {
    /// Creates a new `BoolPattern` that matches any boolean value.
    pub fn any() -> Self { BoolPattern::Any }

    /// Creates a new `BoolPattern` that matches the specific boolean value.
    pub fn value(value: bool) -> Self { BoolPattern::Value(value) }
}

impl Matcher for BoolPattern {
    fn paths(&self, haystack: &CBOR) -> Vec<Path> {
        let is_hit = haystack.as_bool().is_some_and(|value| match self {
            BoolPattern::Any => true,
            BoolPattern::Value(want) => value == *want,
        });

        if is_hit {
            vec![vec![haystack.clone()]]
        } else {
            vec![]
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        let idx = literals.len();
        literals.push(Pattern::Value(crate::pattern::ValuePattern::Bool(
            self.clone(),
        )));
        code.push(Instr::MatchPredicate(idx));
    }
}

impl std::fmt::Display for BoolPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoolPattern::Any => write!(f, "bool"),
            BoolPattern::Value(true) => write!(f, "true"),
            BoolPattern::Value(false) => write!(f, "false"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_pattern_display() {
        assert_eq!(BoolPattern::any().to_string(), "bool");
        assert_eq!(BoolPattern::value(true).to_string(), "true");
        assert_eq!(BoolPattern::value(false).to_string(), "false");
    }

    #[test]
    fn test_bool_pattern_matching() {
        let true_cbor = true.to_cbor();
        let false_cbor = false.to_cbor();
        let number_cbor = 42.to_cbor();

        // Test Any pattern
        let any_pattern = BoolPattern::any();
        assert!(any_pattern.matches(&true_cbor));
        assert!(any_pattern.matches(&false_cbor));
        assert!(!any_pattern.matches(&number_cbor));

        // Test specific value patterns
        let true_pattern = BoolPattern::value(true);
        assert!(true_pattern.matches(&true_cbor));
        assert!(!true_pattern.matches(&false_cbor));
        assert!(!true_pattern.matches(&number_cbor));

        let false_pattern = BoolPattern::value(false);
        assert!(!false_pattern.matches(&true_cbor));
        assert!(false_pattern.matches(&false_cbor));
        assert!(!false_pattern.matches(&number_cbor));
    }

    #[test]
    fn test_bool_pattern_paths() {
        let true_cbor = true.to_cbor();
        let false_cbor = false.to_cbor();

        let any_pattern = BoolPattern::any();
        let true_paths = any_pattern.paths(&true_cbor);
        assert_eq!(true_paths.len(), 1);
        assert_eq!(true_paths[0].len(), 1);
        assert_eq!(true_paths[0][0], true_cbor);

        let false_paths = any_pattern.paths(&false_cbor);
        assert_eq!(false_paths.len(), 1);
        assert_eq!(false_paths[0].len(), 1);
        assert_eq!(false_paths[0][0], false_cbor);

        let true_pattern = BoolPattern::value(true);
        let paths = true_pattern.paths(&false_cbor);
        assert_eq!(paths.len(), 0);
    }
}
