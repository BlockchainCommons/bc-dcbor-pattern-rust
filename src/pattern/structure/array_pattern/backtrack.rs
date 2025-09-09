use dcbor::prelude::*;

use super::helpers::{
    calculate_repeat_bounds, can_repeat_match, extract_capture_with_repeat,
};
use crate::pattern::{Matcher, MetaPattern, Pattern, meta::RepeatPattern};

/// Generic backtracking framework for unifying different types of backtracking
/// state management. This trait abstracts the differences between boolean
/// matching and assignment tracking.
pub trait BacktrackState<T> {
    /// Try to advance the state with a new assignment and return true if
    /// successful
    fn try_advance(&mut self, pattern_idx: usize, element_idx: usize) -> bool;

    /// Backtrack by removing the last state change
    fn backtrack(&mut self);

    /// Check if we've reached a successful final state
    fn is_success(
        &self,
        pattern_idx: usize,
        element_idx: usize,
        patterns_len: usize,
        elements_len: usize,
    ) -> bool;

    /// Get the final result
    #[allow(dead_code)]
    fn get_result(self) -> T;
}

/// Boolean backtracking state - just tracks success/failure
pub struct BooleanBacktrackState;

impl BacktrackState<bool> for BooleanBacktrackState {
    fn try_advance(
        &mut self,
        _pattern_idx: usize,
        _element_idx: usize,
    ) -> bool {
        true // Always allow advancement for boolean matching
    }

    fn backtrack(&mut self) {
        // Nothing to backtrack for boolean state
    }

    fn is_success(
        &self,
        pattern_idx: usize,
        element_idx: usize,
        patterns_len: usize,
        elements_len: usize,
    ) -> bool {
        pattern_idx >= patterns_len && element_idx >= elements_len
    }

    fn get_result(self) -> bool {
        true // If we get here, we succeeded
    }
}

/// Assignment tracking backtracking state - collects pattern-element pairs
pub struct AssignmentBacktrackState {
    pub assignments: Vec<(usize, usize)>,
}

impl AssignmentBacktrackState {
    pub fn new() -> Self { Self { assignments: Vec::new() } }

    #[allow(dead_code)]
    pub fn len(&self) -> usize { self.assignments.len() }

    #[allow(dead_code)]
    pub fn truncate(&mut self, len: usize) { self.assignments.truncate(len); }
}

impl BacktrackState<Vec<(usize, usize)>> for AssignmentBacktrackState {
    fn try_advance(&mut self, pattern_idx: usize, element_idx: usize) -> bool {
        self.assignments.push((pattern_idx, element_idx));
        true
    }

    fn backtrack(&mut self) { self.assignments.pop(); }

    fn is_success(
        &self,
        pattern_idx: usize,
        element_idx: usize,
        patterns_len: usize,
        elements_len: usize,
    ) -> bool {
        pattern_idx >= patterns_len && element_idx >= elements_len
    }

    fn get_result(self) -> Vec<(usize, usize)> { self.assignments }
}

/// Generic backtracking algorithm that works with any BacktrackState
pub struct GenericBacktracker<'a> {
    patterns: &'a [Pattern],
    arr: &'a [CBOR],
}

impl<'a> GenericBacktracker<'a> {
    pub fn new(patterns: &'a [Pattern], arr: &'a [CBOR]) -> Self {
        Self { patterns, arr }
    }

    /// Generic backtracking algorithm that works with any state type
    pub fn backtrack<T, S: BacktrackState<T>>(
        &self,
        state: &mut S,
        pattern_idx: usize,
        element_idx: usize,
    ) -> bool {
        // Base case: if we've matched all patterns
        if state.is_success(
            pattern_idx,
            element_idx,
            self.patterns.len(),
            self.arr.len(),
        ) {
            return true;
        }

        if pattern_idx >= self.patterns.len() {
            return false; // No more patterns but still have elements
        }

        let current_pattern = &self.patterns[pattern_idx];

        match current_pattern {
            Pattern::Meta(MetaPattern::Repeat(repeat_pattern)) => self
                .try_repeat_backtrack(
                    repeat_pattern,
                    state,
                    pattern_idx,
                    element_idx,
                ),
            Pattern::Meta(MetaPattern::Capture(_capture_pattern)) => {
                // Check if the capture pattern contains a repeat pattern
                if let Some(repeat_pattern) =
                    extract_capture_with_repeat(current_pattern)
                {
                    // Handle this like a repeat pattern
                    self.try_repeat_backtrack(
                        repeat_pattern,
                        state,
                        pattern_idx,
                        element_idx,
                    )
                } else {
                    // Handle as a normal single-element capture
                    if element_idx < self.arr.len() {
                        let element = &self.arr[element_idx];
                        let matches = current_pattern.matches(element);

                        if matches
                            && state.try_advance(pattern_idx, element_idx)
                        {
                            if self.backtrack(
                                state,
                                pattern_idx + 1,
                                element_idx + 1,
                            ) {
                                return true;
                            }
                            // Backtracking is handled by the recursive call
                            // failing
                            state.backtrack();
                        }
                    }
                    false
                }
            }
            _ => {
                // Non-repeat pattern: must match exactly one element
                if element_idx < self.arr.len() {
                    let element = &self.arr[element_idx];
                    let matches = current_pattern.matches(element);

                    if matches && state.try_advance(pattern_idx, element_idx) {
                        if self.backtrack(
                            state,
                            pattern_idx + 1,
                            element_idx + 1,
                        ) {
                            return true;
                        }
                        // Backtracking is handled by the recursive call failing
                        state.backtrack();
                    }
                }
                false
            }
        }
    }

    /// Helper for repeat pattern backtracking with generic state
    fn try_repeat_backtrack<T, S: BacktrackState<T>>(
        &self,
        repeat_pattern: &RepeatPattern,
        state: &mut S,
        pattern_idx: usize,
        element_idx: usize,
    ) -> bool {
        let quantifier = repeat_pattern.quantifier();
        let (min_count, max_count) =
            calculate_repeat_bounds(quantifier, element_idx, self.arr.len());

        // Try different numbers of repetitions (greedy: start with max)
        for rep_count in (min_count..=max_count).rev() {
            if element_idx + rep_count <= self.arr.len()
                && can_repeat_match(
                    repeat_pattern,
                    self.arr,
                    element_idx,
                    rep_count,
                )
            {
                // Record state for all consumed elements
                for i in 0..rep_count {
                    if !state.try_advance(pattern_idx, element_idx + i) {
                        // If we can't advance, backtrack what we've added
                        // and try next rep_count
                        for _ in 0..i {
                            state.backtrack();
                        }
                        break;
                    }
                }

                // Try to match the rest of the sequence recursively
                if self.backtrack(
                    state,
                    pattern_idx + 1,
                    element_idx + rep_count,
                ) {
                    return true;
                }

                // Backtrack: undo all the advances we made for this
                // rep_count
                for _ in 0..rep_count {
                    state.backtrack();
                }
            }
        }
        false
    }
}
