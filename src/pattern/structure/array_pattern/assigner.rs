use dcbor::prelude::*;
use crate::pattern::{Pattern, Matcher};
use super::backtrack::{BooleanBacktrackState, AssignmentBacktrackState, GenericBacktracker};
use super::helpers::has_repeat_patterns_in_slice;

/// Helper struct for handling element-to-pattern assignment logic.
/// Encapsulates the complex logic for mapping array elements to sequence patterns
/// that was previously duplicated between matching and capture collection.
pub struct SequenceAssigner<'a> {
    patterns: &'a [Pattern],
    arr: &'a [CBOR],
}

impl<'a> SequenceAssigner<'a> {
    /// Create a new SequenceAssigner for the given patterns and array elements.
    pub fn new(patterns: &'a [Pattern], arr: &'a [CBOR]) -> Self {
        Self { patterns, arr }
    }

    /// Check if the sequence can match against the array elements (boolean result).
    pub fn can_match(&self) -> bool {
        // Simple case: if no patterns, then empty array should match
        if self.patterns.is_empty() {
            return self.arr.is_empty();
        }

        // Check if we have any repeat patterns that require backtracking
        let has_repeat_patterns =
            has_repeat_patterns_in_slice(self.patterns);

        // Simple case: if pattern count equals element count AND no repeat
        // patterns
        if self.patterns.len() == self.arr.len() && !has_repeat_patterns {
            // Try one-to-one matching
            return self
                .patterns
                .iter()
                .enumerate()
                .all(|(i, pattern)| pattern.matches(&self.arr[i]));
        }

        // Complex case: use generic backtracking framework
        let backtracker = GenericBacktracker::new(self.patterns, self.arr);
        let mut state = BooleanBacktrackState;
        backtracker.backtrack(&mut state, 0, 0)
    }

    /// Find the element-to-pattern assignments (returns assignment pairs).
    pub fn find_assignments(&self) -> Option<Vec<(usize, usize)>> {
        // Simple case: if no patterns, then empty array should match
        if self.patterns.is_empty() {
            return if self.arr.is_empty() {
                Some(Vec::new())
            } else {
                None
            };
        }

        // Check if we have any repeat patterns that require backtracking
        let has_repeat_patterns =
            has_repeat_patterns_in_slice(self.patterns);

        // Simple case: if pattern count equals element count AND no repeat patterns
        if self.patterns.len() == self.arr.len() && !has_repeat_patterns {
            let mut assignments = Vec::new();
            for (pattern_idx, pattern) in self.patterns.iter().enumerate() {
                let element = &self.arr[pattern_idx];
                if pattern.matches(element) {
                    assignments.push((pattern_idx, pattern_idx));
                } else {
                    return None; // Pattern doesn't match its corresponding element
                }
            }
            return Some(assignments);
        }

        // Complex case: use generic backtracking framework
        let backtracker = GenericBacktracker::new(self.patterns, self.arr);
        let mut state = AssignmentBacktrackState::new();
        if backtracker.backtrack(&mut state, 0, 0) {
            Some(state.assignments)
        } else {
            None
        }
    }
}
