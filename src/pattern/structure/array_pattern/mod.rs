use std::ops::RangeBounds;

use dcbor::prelude::*;

use crate::{
    Interval,
    pattern::{
        Matcher, MetaPattern, Path, Pattern,
        meta::{RepeatPattern, SequencePattern},
        vm::Instr,
    },
};

mod assigner;
mod backtrack;
mod helpers;

use assigner::SequenceAssigner;
use helpers::*;

/// Pattern for matching CBOR array structures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayPattern {
    /// Matches any array.
    Any,
    /// Matches arrays with elements that match the given pattern.
    Elements(Box<Pattern>),
    /// Matches arrays with length in the given interval.
    Length(Interval),
}

impl ArrayPattern {
    /// Creates a new `ArrayPattern` that matches any array.
    pub fn any() -> Self { ArrayPattern::Any }

    /// Creates a new `ArrayPattern` that matches arrays with elements
    /// that match the given pattern.
    pub fn with_elements(pattern: Pattern) -> Self {
        ArrayPattern::Elements(Box::new(pattern))
    }

    pub fn with_length_range<R: RangeBounds<usize>>(range: R) -> Self {
        ArrayPattern::Length(Interval::new(range))
    }

    /// Creates a new `ArrayPattern` that matches arrays with length in the
    /// given range.
    pub fn with_length_interval(interval: Interval) -> Self {
        ArrayPattern::Length(interval)
    }

    /// Match a complex sequence against array elements using VM-based matching.
    /// This handles patterns with repeats and other complex constructs that
    /// require backtracking and proper quantifier evaluation.
    fn match_complex_sequence(
        &self,
        cbor: &CBOR,
        pattern: &Pattern,
    ) -> Vec<Path> {
        // For complex sequences containing repeats, we need to check if the
        // pattern can match the array elements in sequence.
        // The key insight is that a sequence pattern like
        // `(*)*, 42, (*)*`  should match if there's any way to
        // arrange the array elements to satisfy the sequence
        // requirements.

        match cbor.as_case() {
            CBORCase::Array(arr) => {
                // Create a synthetic "element sequence" CBOR value to match
                // against This represents the array elements as
                // a sequence that the pattern can evaluate

                // For sequences with repeats, we need to check if the pattern
                // can be satisfied by the array elements in order
                let can_match =
                    self.can_match_sequence_against_array(pattern, arr);

                if can_match {
                    vec![vec![cbor.clone()]]
                } else {
                    vec![]
                }
            }
            _ => {
                vec![] // Not an array
            }
        }
    }

    /// Check if a sequence pattern can match against array elements.
    /// This implements the core logic for matching patterns like
    /// `(*)*, 42, (*)*` against array elements.
    fn can_match_sequence_against_array(
        &self,
        pattern: &Pattern,
        arr: &[CBOR],
    ) -> bool {
        match pattern {
            Pattern::Meta(MetaPattern::Sequence(seq_pattern)) => {
                self.match_sequence_patterns_against_array(seq_pattern, arr)
            }
            Pattern::Meta(MetaPattern::Repeat(repeat_pattern)) => {
                // Single repeat pattern: check if it can consume all array
                // elements
                self.match_repeat_pattern_against_array(repeat_pattern, arr)
            }
            _ => {
                // For non-sequence patterns, fall back to simple matching
                let array_cbor = arr.to_cbor();
                pattern.matches(&array_cbor)
            }
        }
    }

    /// Match a sequence of patterns against array elements.
    /// This is the core algorithm for handling sequences with repeats.
    fn match_sequence_patterns_against_array(
        &self,
        seq_pattern: &SequencePattern,
        arr: &[CBOR],
    ) -> bool {
        let patterns = seq_pattern.patterns();
        let assigner = SequenceAssigner::new(patterns, arr);
        assigner.can_match()
    }

    /// Match a single repeat pattern against array elements.
    fn match_repeat_pattern_against_array(
        &self,
        repeat_pattern: &RepeatPattern,
        arr: &[CBOR],
    ) -> bool {
        let quantifier = repeat_pattern.quantifier();
        let min_count = quantifier.min();
        let max_count = quantifier.max().unwrap_or(arr.len());

        // Check if the array length is within the valid range for this repeat
        if arr.len() < min_count || arr.len() > max_count {
            return false;
        }

        // Check if all elements match the repeated pattern
        arr.iter()
            .all(|element| repeat_pattern.pattern().matches(element))
    }

    /// Handle sequence patterns with captures by manually matching elements
    /// and collecting captures with proper array context.
    fn handle_sequence_captures(
        &self,
        seq_pattern: &SequencePattern,
        array_cbor: &CBOR,
        arr: &[CBOR],
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        // Use the existing sequence matching logic to find element assignments
        if let Some(assignments) =
            self.find_sequence_element_assignments(seq_pattern, arr)
        {
            let mut all_captures = std::collections::HashMap::new();

            // Process each pattern and its assigned elements
            for (pattern_idx, pattern) in
                seq_pattern.patterns().iter().enumerate()
            {
                // Check if this is a capture pattern containing a repeat
                // pattern
                if let Pattern::Meta(crate::pattern::MetaPattern::Capture(
                    capture_pattern,
                )) = pattern
                {
                    // Check if the capture contains a repeat pattern
                    if extract_capture_with_repeat(pattern).is_some() {
                        // This is a capture pattern with a repeat (like
                        // @rest((*)*)) We need to
                        // capture the sub-array of matched elements
                        let captured_elements: Vec<CBOR> = assignments
                            .iter()
                            .filter_map(|&(p_idx, e_idx)| {
                                if p_idx == pattern_idx {
                                    Some(arr[e_idx].clone())
                                } else {
                                    None
                                }
                            })
                            .collect();

                        // Create a sub-array from the captured elements
                        let sub_array = captured_elements.to_cbor();

                        // For capture patterns, we directly capture the
                        // sub-array with the capture name
                        let capture_name = capture_pattern.name().to_string();
                        let array_context_path =
                            build_simple_array_context_path(
                                array_cbor, &sub_array,
                            );

                        all_captures
                            .entry(capture_name.clone())
                            .or_insert_with(Vec::new)
                            .push(array_context_path);

                        continue;
                    }
                }
                // Check if this is a direct repeat pattern that might capture
                // multiple elements
                else if is_repeat_pattern(pattern) {
                    if let Pattern::Meta(crate::pattern::MetaPattern::Repeat(
                        repeat_pattern,
                    )) = pattern
                    {
                        // For repeat patterns, check if they have captures
                        let mut repeat_capture_names = Vec::new();
                        repeat_pattern
                            .collect_capture_names(&mut repeat_capture_names);

                        if !repeat_capture_names.is_empty() {
                            // This is a repeat pattern with captures (like
                            // @rest((*)*))
                            // We need to capture the sub-array of matched
                            // elements
                            let captured_elements: Vec<CBOR> = assignments
                                .iter()
                                .filter_map(|&(p_idx, e_idx)| {
                                    if p_idx == pattern_idx {
                                        Some(arr[e_idx].clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            // Create a sub-array from the captured elements
                            let sub_array = captured_elements.to_cbor();

                            // Get captures from the repeat pattern against the
                            // sub-array
                            let (_sub_paths, sub_captures) =
                                repeat_pattern.paths_with_captures(&sub_array);

                            // Transform captures to include array context
                            transform_captures_with_array_context(
                                array_cbor,
                                &sub_array,
                                sub_captures,
                                &mut all_captures,
                            );
                            continue;
                        }
                    }
                }

                // For non-repeat patterns or repeat patterns without captures,
                // process each assigned element individually
                for element_idx in
                    assignments.iter().filter_map(|&(p_idx, e_idx)| {
                        if p_idx == pattern_idx {
                            Some(e_idx)
                        } else {
                            None
                        }
                    })
                {
                    let element = &arr[element_idx];

                    // Get captures from this pattern matching this element
                    let (_element_paths, element_captures) =
                        pattern.paths_with_captures(element);

                    // Transform captures to include array context
                    transform_captures_with_array_context(
                        array_cbor,
                        element,
                        element_captures,
                        &mut all_captures,
                    );
                }
            }

            // Return the array path and all captures
            (vec![vec![array_cbor.clone()]], all_captures)
        } else {
            // Sequence doesn't match the array
            (vec![], std::collections::HashMap::new())
        }
    }

    /// Find which array elements are assigned to which sequence patterns.
    /// Returns a vector of (pattern_index, element_index) pairs if the sequence
    /// matches.
    fn find_sequence_element_assignments(
        &self,
        seq_pattern: &SequencePattern,
        arr: &[CBOR],
    ) -> Option<Vec<(usize, usize)>> {
        let patterns = seq_pattern.patterns();
        let assigner = SequenceAssigner::new(patterns, arr);
        assigner.find_assignments()
    }
}

impl Matcher for ArrayPattern {
    fn paths(&self, haystack: &CBOR) -> Vec<Path> {
        // First check if this is an array
        match haystack.as_case() {
            CBORCase::Array(arr) => {
                match self {
                    ArrayPattern::Any => {
                        // Match any array - return the array itself
                        vec![vec![haystack.clone()]]
                    }
                    ArrayPattern::Elements(pattern) => {
                        // For unified syntax, the pattern should match against
                        // the array elements
                        // as a sequence, not against any individual element.
                        //
                        // Examples:
                        // - [42] should match [42] but not [1, 42, 3]
                        // - ["a" , "b"] should match ["a", "b"] but not ["a",
                        //   "x", "b"]

                        // Check if this is a simple single-element case
                        use crate::pattern::{MetaPattern, Pattern};

                        match pattern.as_ref() {
                            // Simple case: single pattern should match array
                            // with exactly one element
                            Pattern::Value(_)
                            | Pattern::Structure(_)
                            | Pattern::Meta(MetaPattern::Any(_)) => {
                                if arr.len() == 1 {
                                    if pattern.matches(&arr[0]) {
                                        vec![vec![haystack.clone()]]
                                    } else {
                                        vec![]
                                    }
                                } else {
                                    vec![]
                                }
                            }

                            // Complex case: sequences, repeats, etc.
                            Pattern::Meta(MetaPattern::Sequence(
                                seq_pattern,
                            )) => {
                                let patterns = seq_pattern.patterns();

                                // Check if this sequence contains any repeat
                                // patterns that require VM-based matching
                                let has_repeat_patterns =
                                    has_repeat_patterns_in_slice(patterns);

                                if has_repeat_patterns {
                                    // Use VM-based matching for complex
                                    // sequences

                                    self.match_complex_sequence(
                                        haystack, pattern,
                                    )
                                } else {
                                    // Simple sequence: match each pattern
                                    // against consecutive elements
                                    if patterns.len() == arr.len() {
                                        // Check if each pattern matches the
                                        // corresponding array element
                                        for (i, element_pattern) in
                                            patterns.iter().enumerate()
                                        {
                                            if !element_pattern.matches(&arr[i])
                                            {
                                                return vec![];
                                            }
                                        }
                                        vec![vec![haystack.clone()]]
                                    } else {
                                        vec![]
                                    }
                                }
                            }

                            // For individual repeat patterns
                            Pattern::Meta(MetaPattern::Repeat(_)) => {
                                // Use VM-based matching for repeat patterns
                                self.match_complex_sequence(haystack, pattern)
                            }

                            // For other meta patterns, handle them properly
                            Pattern::Meta(MetaPattern::Capture(
                                capture_pattern,
                            )) => {
                                // Capture patterns should search within array
                                // elements
                                // (This is different from non-capture patterns)
                                let has_matching_element =
                                    arr.iter().any(|element| {
                                        capture_pattern
                                            .pattern()
                                            .matches(element)
                                    });

                                if has_matching_element {
                                    vec![vec![haystack.clone()]]
                                } else {
                                    vec![]
                                }
                            }

                            // For other meta patterns (or, and, etc.), use the
                            // old heuristic
                            // This handles cases like `[(number | text)]`
                            _ => {
                                // Check if the pattern matches the array as a
                                // whole sequence
                                // For now, use a heuristic: if it's a simple
                                // meta pattern,
                                // apply it to each element and require at least
                                // one match
                                // This is not perfect but maintains some
                                // compatibility
                                let mut result = Vec::new();
                                for element in arr {
                                    if pattern.matches(element) {
                                        result.push(vec![haystack.clone()]);
                                        break;
                                    }
                                }
                                result
                            }
                        }
                    }
                    ArrayPattern::Length(interval) => {
                        if interval.contains(arr.len()) {
                            vec![vec![haystack.clone()]]
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
        captures: &mut Vec<String>,
    ) {
        // Collect capture names from inner patterns
        self.collect_capture_names(captures);

        // Check if this pattern has captures
        let mut capture_names = Vec::new();
        self.collect_capture_names(&mut capture_names);

        if capture_names.is_empty() {
            // No captures, use the simple MatchStructure approach
            let idx = literals.len();
            literals.push(Pattern::Structure(
                crate::pattern::StructurePattern::Array(self.clone()),
            ));
            code.push(Instr::MatchStructure(idx));
        } else {
            // Has captures, compile to VM navigation instructions
            match self {
                ArrayPattern::Elements(pattern) => {
                    // First check that we have an array
                    let array_check_idx = literals.len();
                    literals.push(Pattern::Structure(
                        crate::pattern::StructurePattern::Array(
                            ArrayPattern::Any,
                        ),
                    ));
                    code.push(Instr::MatchStructure(array_check_idx));

                    // Navigate to array elements
                    code.push(Instr::PushAxis(
                        crate::pattern::vm::Axis::ArrayElement,
                    ));

                    // Compile the inner pattern with captures
                    pattern.compile(code, literals, captures);

                    // Pop back to array level
                    code.push(Instr::Pop);
                }
                _ => {
                    // Other array patterns (length-based) don't support
                    // captures in this context Fall back to
                    // MatchStructure
                    let idx = literals.len();
                    literals.push(Pattern::Structure(
                        crate::pattern::StructurePattern::Array(self.clone()),
                    ));
                    code.push(Instr::MatchStructure(idx));
                }
            }
        }
    }

    fn collect_capture_names(&self, names: &mut Vec<String>) {
        match self {
            ArrayPattern::Any => {
                // No captures in a simple any pattern
            }
            ArrayPattern::Elements(pattern) => {
                // Collect captures from the element pattern
                pattern.collect_capture_names(names);
            }
            ArrayPattern::Length(_) => {
                // No captures in length range patterns
            }
        }
    }

    fn paths_with_captures(
        &self,
        cbor: &CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        // For simple cases that never have captures, use the fast path
        match self {
            ArrayPattern::Any | ArrayPattern::Length(_) => {
                return (self.paths(cbor), std::collections::HashMap::new());
            }
            ArrayPattern::Elements(pattern) => {
                // Check if this specific pattern has any captures
                let mut capture_names = Vec::new();
                pattern.collect_capture_names(&mut capture_names);

                if capture_names.is_empty() {
                    // No captures in the element pattern, use the fast path
                    return (
                        self.paths(cbor),
                        std::collections::HashMap::new(),
                    );
                }

                // Has captures, continue with complex logic below
            }
        }

        match cbor.as_case() {
            CBORCase::Array(_arr) => {
                if let ArrayPattern::Elements(pattern) = self {
                    // First check if this array pattern matches at all
                    if self.paths(cbor).is_empty() {
                        return (vec![], std::collections::HashMap::new());
                    }

                    // For patterns with captures, we need special handling
                    // depending on the inner pattern type
                    match pattern.as_ref() {
                        Pattern::Meta(
                            crate::pattern::MetaPattern::Sequence(seq_pattern),
                        ) => {
                            // Special handling for SequencePattern with
                            // captures
                            self.handle_sequence_captures(
                                seq_pattern,
                                cbor,
                                _arr,
                            )
                        }
                        Pattern::Meta(
                            crate::pattern::MetaPattern::Capture(
                                _capture_pattern,
                            ),
                        ) => {
                            // For capture patterns like [@item(number)] or
                            // [@item(42)],
                            // use the VM approach for consistency with existing
                            // behavior

                            // Use the VM approach for consistent behavior
                            let mut code = Vec::new();
                            let mut literals = Vec::new();
                            let mut captures_list = Vec::new();

                            // Compile the entire ArrayPattern (not just the
                            // inner pattern)
                            let array_pattern = Pattern::Structure(
                                crate::pattern::StructurePattern::Array(
                                    self.clone(),
                                ),
                            );
                            array_pattern.compile(
                                &mut code,
                                &mut literals,
                                &mut captures_list,
                            );
                            code.push(crate::pattern::vm::Instr::Accept);

                            let program = crate::pattern::vm::Program {
                                code,
                                literals,
                                capture_names: captures_list,
                            };

                            // Run the VM program against the CBOR
                            crate::pattern::vm::run(&program, cbor)
                        }
                        _ => {
                            // For non-sequence patterns, use the original VM
                            // approach
                            // but start with the main Pattern's VM compilation
                            // for better compatibility
                            let mut code = Vec::new();
                            let mut literals = Vec::new();
                            let mut captures = Vec::new();

                            // Compile the entire ArrayPattern (not just the
                            // inner pattern)
                            let array_pattern = Pattern::Structure(
                                crate::pattern::StructurePattern::Array(
                                    self.clone(),
                                ),
                            );
                            array_pattern.compile(
                                &mut code,
                                &mut literals,
                                &mut captures,
                            );
                            code.push(crate::pattern::vm::Instr::Accept);

                            let program = crate::pattern::vm::Program {
                                code,
                                literals,
                                capture_names: captures,
                            };

                            // Run the VM program against the CBOR
                            crate::pattern::vm::run(&program, cbor)
                        }
                    }
                } else {
                    // Other array patterns (length-based) don't have inner
                    // patterns with captures
                    (self.paths(cbor), std::collections::HashMap::new())
                }
            }
            _ => {
                // Not an array, no match
                (vec![], std::collections::HashMap::new())
            }
        }
    }
}

impl std::fmt::Display for ArrayPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayPattern::Any => write!(f, "array"),
            ArrayPattern::Elements(pattern) => {
                let formatted_pattern = format_array_element_pattern(pattern);
                write!(f, "[{}]", formatted_pattern)
            }
            ArrayPattern::Length(interval) => {
                write!(f, "[{}]", interval)
            }
        }
    }
}
