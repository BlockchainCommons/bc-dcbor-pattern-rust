use dcbor::prelude::*;

use crate::pattern::{
    Matcher, MetaPattern, Path, Pattern,
    meta::{RepeatPattern, SequencePattern},
    vm::Instr,
};

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
        // (ANY)*>NUMBER(42)>(ANY)* should match if there's any way to
        // arrange the array elements to satisfy the sequence
        // requirements.

        match cbor.as_case() {
            CBORCase::Array(arr) => {
                // Create a synthetic "element sequence" CBOR value to match
                // against This represents the array elements as
                // a sequence that the pattern can evaluate

                // For sequences with repeats, we need to check if the pattern
                // can be satisfied by the array elements in order
                if self.can_match_sequence_against_array(pattern, arr) {
                    vec![vec![cbor.clone()]]
                } else {
                    vec![]
                }
            }
            _ => vec![], // Not an array
        }
    }

    /// Check if a sequence pattern can match against array elements.
    /// This implements the core logic for matching patterns like
    /// (ANY)*>NUMBER(42)>(ANY)* against array elements.
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
                // Create an array CBOR value by creating a string
                // representation and parsing it
                let elements_str: Vec<String> =
                    arr.iter().map(|item| item.to_string()).collect();
                let array_str = format!("[{}]", elements_str.join(", "));
                if let Ok(array_cbor) =
                    dcbor_parse::parse_dcbor_item(&array_str)
                {
                    pattern.matches(&array_cbor)
                } else {
                    false
                }
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

        // For patterns like (ANY)*>NUMBER(42)>(ANY)*:
        // - First (ANY)* can consume 0 or more elements from the start
        // - NUMBER(42) must match exactly one element (which must be 42)
        // - Last (ANY)* can consume 0 or more elements from the end

        // Simple case: if no patterns, then empty array should match
        if patterns.is_empty() {
            return arr.is_empty();
        }

        // Try to match the sequence using a backtracking approach
        self.backtrack_sequence_match(patterns, arr, 0, 0)
    }

    /// Backtracking algorithm to match sequence patterns against array
    /// elements. pattern_idx: current pattern index in the sequence
    /// element_idx: current element index in the array
    #[allow(clippy::only_used_in_recursion)]
    fn backtrack_sequence_match(
        &self,
        patterns: &[Pattern],
        arr: &[CBOR],
        pattern_idx: usize,
        element_idx: usize,
    ) -> bool {
        // Base case: if we've matched all patterns
        if pattern_idx >= patterns.len() {
            // Success if we've also consumed all elements
            return element_idx >= arr.len();
        }

        let current_pattern = &patterns[pattern_idx];

        match current_pattern {
            Pattern::Meta(MetaPattern::Repeat(repeat_pattern)) => {
                let quantifier = repeat_pattern.quantifier();
                let min_count = quantifier.min();
                // Fix the infinite loop: limit max_count to reasonable bounds
                let remaining_elements = arr.len().saturating_sub(element_idx);
                let max_count = quantifier
                    .max()
                    .unwrap_or(remaining_elements)
                    .min(remaining_elements);

                // Try different numbers of repetitions (greedy: start with max)
                for rep_count in (min_count..=max_count).rev() {
                    // Check bounds to prevent out-of-bounds access
                    if element_idx + rep_count <= arr.len() {
                        let can_match_reps = if rep_count == 0 {
                            true // Zero repetitions always match for rep_count=0
                        } else {
                            (0..rep_count).all(|i| {
                                repeat_pattern
                                    .pattern()
                                    .matches(&arr[element_idx + i])
                            })
                        };

                        if can_match_reps {
                            // Try to match the rest of the sequence
                            if self.backtrack_sequence_match(
                                patterns,
                                arr,
                                pattern_idx + 1,
                                element_idx + rep_count,
                            ) {
                                return true;
                            }
                        }
                    }
                }
                false
            }
            _ => {
                // Non-repeat pattern: must match exactly one element
                if element_idx < arr.len()
                    && current_pattern.matches(&arr[element_idx])
                {
                    self.backtrack_sequence_match(
                        patterns,
                        arr,
                        pattern_idx + 1,
                        element_idx + 1,
                    )
                } else {
                    false
                }
            }
        }
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

            // For each pattern in the sequence, collect captures from its
            // assigned element
            for (pattern_idx, element_idx) in assignments {
                let pattern = &seq_pattern.patterns()[pattern_idx];
                let element = &arr[element_idx];

                // Get captures from this pattern matching this element
                let (_element_paths, element_captures) =
                    pattern.paths_with_captures(element);

                // Transform captures to include array context
                for (capture_name, captured_paths) in element_captures {
                    let mut array_context_paths = Vec::new();
                    for captured_path in captured_paths {
                        // Create path: [array] + [element_at_index] +
                        // rest_of_path
                        let mut array_path =
                            vec![array_cbor.clone(), element.clone()];
                        if captured_path.len() > 1 {
                            array_path
                                .extend(captured_path.iter().skip(1).cloned());
                        }
                        array_context_paths.push(array_path);
                    }
                    all_captures
                        .entry(capture_name)
                        .or_insert_with(Vec::new)
                        .extend(array_context_paths);
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

        // Simple case: if pattern count equals element count, try one-to-one
        // matching
        if patterns.len() == arr.len() {
            let mut assignments = Vec::new();
            for (pattern_idx, pattern) in patterns.iter().enumerate() {
                let element = &arr[pattern_idx];
                if pattern.matches(element) {
                    assignments.push((pattern_idx, pattern_idx));
                } else {
                    return None; // Pattern doesn't match its corresponding element
                }
            }
            return Some(assignments);
        }

        // TODO: Handle more complex cases with repeats and variable assignments
        // For now, return None for sequences that don't have one-to-one element
        // mapping
        None
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
                        // For unified syntax, the pattern should match against
                        // the array elements
                        // as a sequence, not against any individual element.
                        //
                        // Examples:
                        // - ARRAY(NUMBER(42)) should match [42] but not [1, 42,
                        //   3]
                        // - ARRAY(TEXT("a") > TEXT("b")) should match ["a",
                        //   "b"] but not ["a", "x", "b"]

                        // Check if this is a simple single-element case
                        use crate::pattern::{MetaPattern, Pattern};

                        match pattern.as_ref() {
                            // Simple case: single pattern should match array
                            // with exactly one element
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
                            Pattern::Meta(MetaPattern::Sequence(
                                seq_pattern,
                            )) => {
                                let patterns = seq_pattern.patterns();

                                // Check if this sequence contains any repeat
                                // patterns that
                                // require VM-based matching
                                let has_repeat_patterns =
                                    patterns.iter().any(|p| {
                                        matches!(
                                            p,
                                            Pattern::Meta(MetaPattern::Repeat(
                                                _
                                            ))
                                        )
                                    });

                                if has_repeat_patterns {
                                    // Use VM-based matching for complex
                                    // sequences
                                    self.match_complex_sequence(cbor, pattern)
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
                                        vec![vec![cbor.clone()]]
                                    } else {
                                        vec![]
                                    }
                                }
                            }

                            // For individual repeat patterns
                            Pattern::Meta(MetaPattern::Repeat(_)) => {
                                // Use VM-based matching for repeat patterns
                                self.match_complex_sequence(cbor, pattern)
                            }

                            // For other meta patterns (or, and, etc.), delegate
                            // to the pattern matcher
                            // This handles cases like ARRAY(NUMBER | TEXT)
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
                ArrayPattern::WithElements(pattern) => {
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

    fn paths_with_captures(
        &self,
        cbor: &dcbor::CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        // For simple cases that never have captures, use the fast path
        match self {
            ArrayPattern::Any
            | ArrayPattern::WithLength(_)
            | ArrayPattern::WithLengthRange(_) => {
                return (self.paths(cbor), std::collections::HashMap::new());
            }
            ArrayPattern::WithElements(pattern) => {
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

        use dcbor::CBORCase;

        match cbor.as_case() {
            CBORCase::Array(_arr) => {
                if let ArrayPattern::WithElements(pattern) = self {
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

impl ArrayPattern {
    // ...existing methods...

    /// Format a pattern for display within array context.
    /// This handles sequence patterns specially to use commas instead of >.
    fn format_array_element_pattern(pattern: &Pattern) -> String {
        match pattern {
            Pattern::Meta(crate::pattern::MetaPattern::Sequence(
                seq_pattern,
            )) => {
                // For sequence patterns within arrays, use commas instead of >
                let patterns_str: Vec<String> = seq_pattern
                    .patterns()
                    .iter()
                    .map(Self::format_array_element_pattern)
                    .collect();
                patterns_str.join(", ")
            }
            _ => pattern.to_string(),
        }
    }
}

impl std::fmt::Display for ArrayPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayPattern::Any => write!(f, "[*]"),
            ArrayPattern::WithElements(pattern) => {
                let formatted_pattern =
                    Self::format_array_element_pattern(pattern);
                write!(f, "[{}]", formatted_pattern)
            }
            ArrayPattern::WithLength(length) => {
                write!(f, "[{{{}}}]", length)
            }
            ArrayPattern::WithLengthRange(range) => {
                if range.end() == &usize::MAX {
                    write!(f, "[{{{},}}]", range.start())
                } else {
                    write!(f, "[{{{},{}}}]", range.start(), range.end())
                }
            }
        }
    }
}
