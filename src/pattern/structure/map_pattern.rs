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
    /// Matches maps with key-value pairs where both key and value match their
    /// respective patterns.
    WithKeyValue {
        key_pattern: Box<Pattern>,
        value_pattern: Box<Pattern>,
    },
    /// Matches maps with multiple key-value constraints that must all be
    /// satisfied.
    WithKeyValueConstraints(Vec<(Pattern, Pattern)>),
    /// Matches maps with a specific number of key-value pairs.
    WithLength(usize),
    /// Matches maps with number of key-value pairs in the given range
    /// (inclusive).
    WithLengthRange(std::ops::RangeInclusive<usize>),
}

impl MapPattern {
    /// Creates a new `MapPattern` that matches any map.
    pub fn any() -> Self { MapPattern::Any }

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
    pub fn with_key_value(
        key_pattern: Pattern,
        value_pattern: Pattern,
    ) -> Self {
        MapPattern::WithKeyValue {
            key_pattern: Box::new(key_pattern),
            value_pattern: Box::new(value_pattern),
        }
    }

    /// Creates a new `MapPattern` that matches maps with multiple key-value
    /// constraints that must all be satisfied.
    pub fn with_key_value_constraints(
        constraints: Vec<(Pattern, Pattern)>,
    ) -> Self {
        MapPattern::WithKeyValueConstraints(constraints)
    }

    /// Creates a new `MapPattern` that matches maps with a specific number of
    /// key-value pairs.
    pub fn with_length(length: usize) -> Self { MapPattern::WithLength(length) }

    /// Creates a new `MapPattern` that matches maps with number of key-value
    /// pairs in the given range.
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
                            if key_pattern.matches(key)
                                && value_pattern.matches(value)
                            {
                                return vec![vec![cbor.clone()]];
                            }
                        }
                        vec![]
                    }
                    MapPattern::WithKeyValueConstraints(constraints) => {
                        // All constraints must be satisfied
                        for (key_pattern, value_pattern) in constraints {
                            let mut found_match = false;
                            for (key, value) in map.iter() {
                                if key_pattern.matches(key)
                                    && value_pattern.matches(value)
                                {
                                    found_match = true;
                                    break;
                                }
                            }
                            if !found_match {
                                return vec![];
                            }
                        }
                        vec![vec![cbor.clone()]]
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
        captures: &mut Vec<String>,
    ) {
        // Collect capture names from inner patterns
        self.collect_capture_names(captures);

        let idx = literals.len();
        literals.push(Pattern::Structure(
            crate::pattern::StructurePattern::Map(self.clone()),
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
            MapPattern::WithKeyValueConstraints(constraints) => {
                // Collect captures from all key and value patterns
                for (key_pattern, value_pattern) in constraints {
                    key_pattern.collect_capture_names(names);
                    value_pattern.collect_capture_names(names);
                }
            }
            MapPattern::WithLength(_) => {
                // No captures in length patterns
            }
            MapPattern::WithLengthRange(_) => {
                // No captures in length range patterns
            }
        }
    }

    fn paths_with_captures(
        &self,
        cbor: &dcbor::CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        // Check if this CBOR value is a map
        let dcbor::CBORCase::Map(map) = cbor.as_case() else {
            return (vec![], std::collections::HashMap::new());
        };

        match self {
            MapPattern::Any => {
                // Matches any map, no captures
                (vec![vec![cbor.clone()]], std::collections::HashMap::new())
            }
            MapPattern::WithKey(key_pattern) => {
                // Match if any key matches the pattern
                let mut all_captures = std::collections::HashMap::new();
                for (key, _value) in map.iter() {
                    let (key_paths, captures) =
                        key_pattern.paths_with_captures(key);
                    if !key_paths.is_empty() {
                        // Merge captures, adjusting paths to include map
                        // context
                        for (name, capture_paths) in captures {
                            let updated_paths: Vec<Path> = capture_paths
                                .iter()
                                .map(|_capture_path| {
                                    // For map keys, the capture path should be
                                    // [map, key]
                                    vec![cbor.clone(), key.clone()]
                                })
                                .collect();
                            all_captures
                                .entry(name)
                                .or_insert_with(Vec::new)
                                .extend(updated_paths);
                        }
                        return (vec![vec![cbor.clone()]], all_captures);
                    }
                }
                (vec![], all_captures)
            }
            MapPattern::WithValue(value_pattern) => {
                // Match if any value matches the pattern
                let mut all_captures = std::collections::HashMap::new();
                for (_key, value) in map.iter() {
                    let (value_paths, captures) =
                        value_pattern.paths_with_captures(value);
                    if !value_paths.is_empty() {
                        // Merge captures, adjusting paths to include map
                        // context
                        for (name, capture_paths) in captures {
                            let updated_paths: Vec<Path> = capture_paths
                                .iter()
                                .map(|_capture_path| {
                                    // For map values, the capture path should
                                    // be [map, value]
                                    vec![cbor.clone(), value.clone()]
                                })
                                .collect();
                            all_captures
                                .entry(name)
                                .or_insert_with(Vec::new)
                                .extend(updated_paths);
                        }
                        return (vec![vec![cbor.clone()]], all_captures);
                    }
                }
                (vec![], all_captures)
            }
            MapPattern::WithKeyValue { key_pattern, value_pattern } => {
                // Match if there's a key-value pair where both patterns match
                let mut all_captures = std::collections::HashMap::new();
                for (key, value) in map.iter() {
                    let (key_paths, key_captures) =
                        key_pattern.paths_with_captures(key);
                    let (value_paths, value_captures) =
                        value_pattern.paths_with_captures(value);

                    if !key_paths.is_empty() && !value_paths.is_empty() {
                        // Merge key captures
                        for (name, capture_paths) in key_captures {
                            let updated_paths: Vec<Path> = capture_paths
                                .iter()
                                .map(|_capture_path| {
                                    vec![cbor.clone(), key.clone()]
                                })
                                .collect();
                            all_captures
                                .entry(name)
                                .or_insert_with(Vec::new)
                                .extend(updated_paths);
                        }

                        // Merge value captures
                        for (name, capture_paths) in value_captures {
                            let updated_paths: Vec<Path> = capture_paths
                                .iter()
                                .map(|_capture_path| {
                                    vec![cbor.clone(), value.clone()]
                                })
                                .collect();
                            all_captures
                                .entry(name)
                                .or_insert_with(Vec::new)
                                .extend(updated_paths);
                        }

                        return (vec![vec![cbor.clone()]], all_captures);
                    }
                }
                (vec![], all_captures)
            }
            MapPattern::WithKeyValueConstraints(constraints) => {
                // Match if all key-value constraints are satisfied
                let mut all_captures = std::collections::HashMap::new();
                let mut all_constraints_satisfied = true;

                for (key_pattern, value_pattern) in constraints {
                    let mut constraint_satisfied = false;

                    for (key, value) in map.iter() {
                        let (key_paths, key_captures) =
                            key_pattern.paths_with_captures(key);
                        let (value_paths, value_captures) =
                            value_pattern.paths_with_captures(value);

                        if !key_paths.is_empty() && !value_paths.is_empty() {
                            constraint_satisfied = true;

                            // Merge key captures
                            for (name, capture_paths) in key_captures {
                                let updated_paths: Vec<Path> = capture_paths
                                    .iter()
                                    .map(|_capture_path| {
                                        vec![cbor.clone(), key.clone()]
                                    })
                                    .collect();
                                all_captures
                                    .entry(name)
                                    .or_insert_with(Vec::new)
                                    .extend(updated_paths);
                            }

                            // Merge value captures
                            for (name, capture_paths) in value_captures {
                                let updated_paths: Vec<Path> = capture_paths
                                    .iter()
                                    .map(|_capture_path| {
                                        vec![cbor.clone(), value.clone()]
                                    })
                                    .collect();
                                all_captures
                                    .entry(name)
                                    .or_insert_with(Vec::new)
                                    .extend(updated_paths);
                            }
                            break; // Found a matching key-value pair for this constraint
                        }
                    }

                    if !constraint_satisfied {
                        all_constraints_satisfied = false;
                        break;
                    }
                }

                if all_constraints_satisfied {
                    (vec![vec![cbor.clone()]], all_captures)
                } else {
                    (vec![], all_captures)
                }
            }
            _ => {
                // For other variants, fall back to basic paths without captures
                (self.paths(cbor), std::collections::HashMap::new())
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
            MapPattern::WithKeyValueConstraints(constraints) => {
                write!(f, "MAP(")?;
                for (i, (key_pattern, value_pattern)) in
                    constraints.iter().enumerate()
                {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}:{}", key_pattern, value_pattern)?;
                }
                write!(f, ")")
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
