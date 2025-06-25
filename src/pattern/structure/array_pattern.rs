use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching CBOR array structures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayPattern {
    /// Matches any array.
    Any,
    /// Matches arrays with elements that match the given pattern.
    WithElements(Box<Pattern>),
    /// Matches arrays with a specific length.
    WithLength(usize),
    /// Matches arrays with length in the given range (inclusive).
    WithLengthRange(std::ops::RangeInclusive<usize>),
}

impl ArrayPattern {
    /// Creates a new `ArrayPattern` that matches any array.
    pub fn any() -> Self { ArrayPattern::Any }

    /// Creates a new `ArrayPattern` that matches arrays with elements
    /// that match the given pattern.
    pub fn with_elements(pattern: Pattern) -> Self {
        ArrayPattern::WithElements(Box::new(pattern))
    }

    /// Creates a new `ArrayPattern` that matches arrays with a specific length.
    pub fn with_length(length: usize) -> Self {
        ArrayPattern::WithLength(length)
    }

    /// Creates a new `ArrayPattern` that matches arrays with length in the
    /// given range.
    pub fn with_length_range(range: std::ops::RangeInclusive<usize>) -> Self {
        ArrayPattern::WithLengthRange(range)
    }
}

impl Matcher for ArrayPattern {
    fn paths(&self, cbor: &CBOR) -> Vec<Path> {
        // First check if this is an array
        match cbor.as_case() {
            CBORCase::Array(arr) => {
                match self {
                    ArrayPattern::Any => {
                        // Match any array - return the array itself
                        vec![vec![cbor.clone()]]
                    }
                    ArrayPattern::WithElements(pattern) => {
                        // Check if any elements match the pattern
                        let mut result = Vec::new();
                        for element in arr {
                            if pattern.matches(element) {
                                result.push(vec![cbor.clone()]);
                                break; // Found at least one matching element
                            }
                        }
                        result
                    }
                    ArrayPattern::WithLength(target_length) => {
                        if arr.len() == *target_length {
                            vec![vec![cbor.clone()]]
                        } else {
                            vec![]
                        }
                    }
                    ArrayPattern::WithLengthRange(range) => {
                        if range.contains(&arr.len()) {
                            vec![vec![cbor.clone()]]
                        } else {
                            vec![]
                        }
                    }
                }
            }
            _ => {
                // Not an array, no match
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
        let idx = literals.len();
        literals.push(Pattern::Structure(
            crate::pattern::StructurePattern::Array(self.clone()),
        ));
        code.push(Instr::MatchStructure(idx));
    }
}

impl std::fmt::Display for ArrayPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayPattern::Any => write!(f, "ARRAY"),
            ArrayPattern::WithElements(pattern) => {
                write!(f, "ARRAY_ELEM({})", pattern)
            }
            ArrayPattern::WithLength(length) => {
                write!(f, "ARRAY({{{}}})", length)
            }
            ArrayPattern::WithLengthRange(range) => {
                if range.end() == &usize::MAX {
                    write!(f, "ARRAY({{{},}})", range.start())
                } else {
                    write!(f, "ARRAY({{{},{}}})", range.start(), range.end())
                }
            }
        }
    }
}
