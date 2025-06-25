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
                        // For unified syntax, the pattern should match against the array elements
                        // as a sequence, not against any individual element.
                        //
                        // Examples:
                        // - ARRAY(NUMBER(42)) should match [42] but not [1, 42, 3]
                        // - ARRAY(TEXT("a") > TEXT("b")) should match ["a", "b"] but not ["a", "x", "b"]

                        // Check if this is a simple single-element case
                        use crate::pattern::{Pattern, MetaPattern};

                        match pattern.as_ref() {
                            // Simple case: single pattern should match array with exactly one element
                            Pattern::Value(_) | Pattern::Structure(_) => {
                                if arr.len() == 1 {
                                    if pattern.matches(&arr[0]) {
                                        vec![vec![cbor.clone()]]
                                    } else {
                                        vec![]
                                    }
                                } else {
                                    vec![]
                                }
                            }

                            // Complex case: sequences, repeats, etc.
                            Pattern::Meta(MetaPattern::Sequence(seq_pattern)) => {
                                // For sequences, we need to match each pattern against consecutive elements
                                let patterns = seq_pattern.patterns();
                                if patterns.len() == arr.len() {
                                    // Check if each pattern matches the corresponding array element
                                    for (i, element_pattern) in patterns.iter().enumerate() {
                                        if !element_pattern.matches(&arr[i]) {
                                            return vec![];
                                        }
                                    }
                                    vec![vec![cbor.clone()]]
                                } else {
                                    vec![]
                                }
                            }

                            // For other meta patterns (or, and, etc.), delegate to the pattern matcher
                            // This handles cases like ARRAY(NUMBER | TEXT)
                            _ => {
                                // Check if the pattern matches the array as a whole sequence
                                // For now, use a heuristic: if it's a simple meta pattern,
                                // apply it to each element and require at least one match
                                // This is not perfect but maintains some compatibility
                                let mut result = Vec::new();
                                for element in arr {
                                    if pattern.matches(element) {
                                        result.push(vec![cbor.clone()]);
                                        break;
                                    }
                                }
                                result
                            }
                        }
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

    fn collect_capture_names(&self, names: &mut Vec<String>) {
        match self {
            ArrayPattern::Any => {
                // No captures in a simple any pattern
            }
            ArrayPattern::WithElements(pattern) => {
                // Collect captures from the element pattern
                pattern.collect_capture_names(names);
            }
            ArrayPattern::WithLength(_) => {
                // No captures in length patterns
            }
            ArrayPattern::WithLengthRange(_) => {
                // No captures in length range patterns
            }
        }
    }
}

impl std::fmt::Display for ArrayPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayPattern::Any => write!(f, "ARRAY"),
            ArrayPattern::WithElements(pattern) => {
                write!(f, "ARRAY({})", pattern)
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
