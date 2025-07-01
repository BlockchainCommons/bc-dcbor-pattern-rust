use std::ops::RangeBounds;

use dcbor::prelude::*;

use crate::{
    Interval,
    pattern::{Matcher, Path, Pattern, vm::Instr},
};

/// Pattern for matching CBOR map structures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapPattern {
    /// Matches any map.
    Any,
    /// Matches maps with multiple key-value constraints that must all be
    /// satisfied.
    WithKeyValueConstraints(Vec<(Pattern, Pattern)>),
    /// Matches maps with number of key-value pairs in the given interval.
    WithLengthInterval(Interval),
}

impl MapPattern {
    /// Creates a new `MapPattern` that matches any map.
    pub fn any() -> Self { MapPattern::Any }

    /// Creates a new `MapPattern` that matches maps with multiple key-value
    /// constraints that must all be satisfied.
    pub fn with_key_value_constraints(
        constraints: Vec<(Pattern, Pattern)>,
    ) -> Self {
        MapPattern::WithKeyValueConstraints(constraints)
    }

    /// Creates a new `MapPattern` that matches maps with a specific number of
    /// key-value pairs.
    pub fn with_length(length: usize) -> Self {
        MapPattern::WithLengthInterval(Interval::new(length..=length))
    }

    /// Creates a new `MapPattern` that matches maps with number of key-value
    /// pairs in the given range.
    pub fn with_length_range<R: RangeBounds<usize>>(range: R) -> Self {
        MapPattern::WithLengthInterval(Interval::new(range))
    }

    /// Creates a new `MapPattern` that matches maps with number of key-value
    /// pairs in the given range.
    pub fn with_length_interval(interval: Interval) -> Self {
        MapPattern::WithLengthInterval(interval)
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
                    MapPattern::WithLengthInterval(interval) => {
                        if interval.contains(map.len()) {
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
            MapPattern::WithKeyValueConstraints(constraints) => {
                // Collect captures from all key and value patterns
                for (key_pattern, value_pattern) in constraints {
                    key_pattern.collect_capture_names(names);
                    value_pattern.collect_capture_names(names);
                }
            }
            MapPattern::WithLengthInterval(_) => {
                // No captures in length interval patterns
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
            MapPattern::Any => write!(f, "{{*}}"),
            MapPattern::WithKeyValueConstraints(constraints) => {
                write!(f, "{{")?;
                for (i, (key_pattern, value_pattern)) in
                    constraints.iter().enumerate()
                {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key_pattern, value_pattern)?;
                }
                write!(f, "}}")
            }
            MapPattern::WithLengthInterval(interval) => {
                write!(f, "{{{}}}", interval)
            }
        }
    }
}
