use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching CBOR map structures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapPattern {
    /// Matches any map.
    Any,
    /// Matches maps with keys that match the given pattern.
    WithKey(Box<Pattern>),
    /// Matches maps with values that match the given pattern.
    WithValue(Box<Pattern>),
    /// Matches maps with key-value pairs where both key and value match their respective patterns.
    WithKeyValue {
        key_pattern: Box<Pattern>,
        value_pattern: Box<Pattern>,
    },
    /// Matches maps with a specific number of key-value pairs.
    WithLength(usize),
    /// Matches maps with number of key-value pairs in the given range (inclusive).
    WithLengthRange(std::ops::RangeInclusive<usize>),
}

impl MapPattern {
    /// Creates a new `MapPattern` that matches any map.
    pub fn any() -> Self {
        MapPattern::Any
    }

    /// Creates a new `MapPattern` that matches maps with keys
    /// that match the given pattern.
    pub fn with_key(pattern: Pattern) -> Self {
        MapPattern::WithKey(Box::new(pattern))
    }

    /// Creates a new `MapPattern` that matches maps with values
    /// that match the given pattern.
    pub fn with_value(pattern: Pattern) -> Self {
        MapPattern::WithValue(Box::new(pattern))
    }

    /// Creates a new `MapPattern` that matches maps with key-value pairs
    /// where both key and value match their respective patterns.
    pub fn with_key_value(key_pattern: Pattern, value_pattern: Pattern) -> Self {
        MapPattern::WithKeyValue {
            key_pattern: Box::new(key_pattern),
            value_pattern: Box::new(value_pattern),
        }
    }

    /// Creates a new `MapPattern` that matches maps with a specific number of key-value pairs.
    pub fn with_length(length: usize) -> Self {
        MapPattern::WithLength(length)
    }

    /// Creates a new `MapPattern` that matches maps with number of key-value pairs in the given range.
    pub fn with_length_range(range: std::ops::RangeInclusive<usize>) -> Self {
        MapPattern::WithLengthRange(range)
    }
}

impl Matcher for MapPattern {
    fn paths(&self, cbor: &CBOR) -> Vec<Path> {
        // First check if this is a map
        match cbor.as_case() {
            CBORCase::Map(map) => {
                match self {
                    MapPattern::Any => {
                        // Match any map - return the map itself
                        vec![vec![cbor.clone()]]
                    }
                    MapPattern::WithKey(pattern) => {
                        // Check if any keys match the pattern
                        for (key, _value) in map.iter() {
                            if pattern.matches(key) {
                                return vec![vec![cbor.clone()]];
                            }
                        }
                        vec![]
                    }
                    MapPattern::WithValue(pattern) => {
                        // Check if any values match the pattern
                        for (_key, value) in map.iter() {
                            if pattern.matches(value) {
                                return vec![vec![cbor.clone()]];
                            }
                        }
                        vec![]
                    }
                    MapPattern::WithKeyValue { key_pattern, value_pattern } => {
                        // Check if any key-value pairs match both patterns
                        for (key, value) in map.iter() {
                            if key_pattern.matches(key) && value_pattern.matches(value) {
                                return vec![vec![cbor.clone()]];
                            }
                        }
                        vec![]
                    }
                    MapPattern::WithLength(target_length) => {
                        if map.len() == *target_length {
                            vec![vec![cbor.clone()]]
                        } else {
                            vec![]
                        }
                    }
                    MapPattern::WithLengthRange(range) => {
                        if range.contains(&map.len()) {
                            vec![vec![cbor.clone()]]
                        } else {
                            vec![]
                        }
                    }
                }
            }
            _ => {
                // Not a map, no match
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
            crate::pattern::StructurePattern::Map(self.clone())
        ));
        code.push(Instr::MatchStructure(idx));
    }

    fn collect_capture_names(&self, names: &mut Vec<String>) {
        match self {
            MapPattern::Any => {
                // No captures in a simple any pattern
            }
            MapPattern::WithKey(pattern) => {
                // Collect captures from key pattern
                pattern.collect_capture_names(names);
            }
            MapPattern::WithValue(pattern) => {
                // Collect captures from value pattern
                pattern.collect_capture_names(names);
            }
            MapPattern::WithKeyValue { key_pattern, value_pattern } => {
                // Collect captures from both key and value patterns
                key_pattern.collect_capture_names(names);
                value_pattern.collect_capture_names(names);
            }
            MapPattern::WithLength(_) => {
                // No captures in length patterns
            }
            MapPattern::WithLengthRange(_) => {
                // No captures in length range patterns
            }
        }
    }
}

impl std::fmt::Display for MapPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapPattern::Any => write!(f, "MAP"),
            MapPattern::WithKey(pattern) => {
                write!(f, "MAP_KEY({})", pattern)
            }
            MapPattern::WithValue(pattern) => {
                write!(f, "MAP_VALUE({})", pattern)
            }
            MapPattern::WithKeyValue { key_pattern, value_pattern } => {
                write!(f, "MAP_KV({}, {})", key_pattern, value_pattern)
            }
            MapPattern::WithLength(length) => {
                write!(f, "MAP({{{}}})", length)
            }
            MapPattern::WithLengthRange(range) => {
                if range.end() == &usize::MAX {
                    write!(f, "MAP({{{},}})", range.start())
                } else {
                    write!(f, "MAP({{{},{}}})", range.start(), range.end())
                }
            }
        }
    }
}
