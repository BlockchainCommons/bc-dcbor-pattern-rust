use dcbor::prelude::*;

use crate::pattern::{Matcher, MetaPattern, Pattern, meta::RepeatPattern};

// Helper functions for pattern type detection

/// Check if a pattern is a repeat pattern.
pub fn is_repeat_pattern(pattern: &Pattern) -> bool {
    matches!(pattern, Pattern::Meta(MetaPattern::Repeat(_)))
}

/// Check if a pattern is a capture pattern containing a repeat pattern.
/// Returns the inner repeat pattern if found.
pub fn extract_capture_with_repeat(
    pattern: &Pattern,
) -> Option<&RepeatPattern> {
    if let Pattern::Meta(MetaPattern::Capture(capture_pattern)) = pattern
        && let Pattern::Meta(MetaPattern::Repeat(repeat_pattern)) =
            capture_pattern.pattern()
    {
        return Some(repeat_pattern);
    }
    None
}

/// Extract any repeat pattern from a pattern, whether direct or within a
/// capture.
pub fn extract_repeat_pattern(pattern: &Pattern) -> Option<&RepeatPattern> {
    match pattern {
        Pattern::Meta(MetaPattern::Repeat(repeat_pattern)) => {
            Some(repeat_pattern)
        }
        Pattern::Meta(MetaPattern::Capture(capture_pattern)) => {
            if let Pattern::Meta(MetaPattern::Repeat(repeat_pattern)) =
                capture_pattern.pattern()
            {
                Some(repeat_pattern)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Check if a slice of patterns contains any repeat patterns (direct or in
/// captures).
pub fn has_repeat_patterns_in_slice(patterns: &[Pattern]) -> bool {
    patterns.iter().any(|p| extract_repeat_pattern(p).is_some())
}

/// Format a pattern for display within array context.
/// This handles sequence patterns specially to use commas instead of >.
pub fn format_array_element_pattern(pattern: &Pattern) -> String {
    match pattern {
        Pattern::Meta(crate::pattern::MetaPattern::Sequence(seq_pattern)) => {
            // For sequence patterns within arrays, use commas instead of >
            let patterns_str: Vec<String> = seq_pattern
                .patterns()
                .iter()
                .map(format_array_element_pattern)
                .collect();
            patterns_str.join(", ")
        }
        _ => pattern.to_string(),
    }
}

// Helper functions for repeat pattern quantifier logic

/// Calculate the bounds for repeat pattern matching based on quantifier and
/// available elements.
pub fn calculate_repeat_bounds(
    quantifier: &crate::Quantifier,
    element_idx: usize,
    arr_len: usize,
) -> (usize, usize) {
    let min_count = quantifier.min();
    let remaining_elements = arr_len.saturating_sub(element_idx);
    let max_count = quantifier
        .max()
        .unwrap_or(remaining_elements)
        .min(remaining_elements);
    (min_count, max_count)
}

/// Check if a repeat pattern can match a specific number of elements starting
/// at element_idx.
pub fn can_repeat_match(
    repeat_pattern: &RepeatPattern,
    arr: &[CBOR],
    element_idx: usize,
    rep_count: usize,
) -> bool {
    if rep_count == 0 {
        true // Zero repetitions always match
    } else {
        (0..rep_count).all(|i| {
            let element = &arr[element_idx + i];
            repeat_pattern.pattern().matches(element)
        })
    }
}

// Helper functions for capture context path building

/// Build a simple array context path: [array_cbor, element]
pub fn build_simple_array_context_path(
    array_cbor: &CBOR,
    element: &CBOR,
) -> Vec<CBOR> {
    vec![array_cbor.clone(), element.clone()]
}

/// Build an extended array context path: [array_cbor, element] + captured_path
/// (skip first element)
pub fn build_extended_array_context_path(
    array_cbor: &CBOR,
    element: &CBOR,
    captured_path: &[CBOR],
) -> Vec<CBOR> {
    let mut array_path = vec![array_cbor.clone(), element.clone()];
    if captured_path.len() > 1 {
        array_path.extend(captured_path.iter().skip(1).cloned());
    }
    array_path
}

/// Transform nested captures to include array context, extending all_captures
pub fn transform_captures_with_array_context(
    array_cbor: &CBOR,
    element: &CBOR,
    nested_captures: std::collections::HashMap<String, Vec<Vec<CBOR>>>,
    all_captures: &mut std::collections::HashMap<String, Vec<Vec<CBOR>>>,
) {
    for (capture_name, captured_paths) in nested_captures {
        let mut array_context_paths = Vec::new();
        for captured_path in captured_paths {
            let array_path = build_extended_array_context_path(
                array_cbor,
                element,
                &captured_path,
            );
            array_context_paths.push(array_path);
        }
        all_captures
            .entry(capture_name)
            .or_default()
            .extend(array_context_paths);
    }
}
