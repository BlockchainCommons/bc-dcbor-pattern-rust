mod common;

use std::collections::HashMap;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{
    FormatPathsOpts, Matcher, Pattern, format_paths_with_captures,
};
use indoc::indoc;

#[cfg(test)]
mod test_comprehensive_variadic_sequences {
    use super::*;

    // ============================================================================
    // PHASE 2.1: Basic Quantifiers (Greedy) - Default behavior
    // ============================================================================

    #[test]
    fn test_zero_or_more_greedy() {
        // Pattern: [(*)*] should match arrays of any length
        let pattern = Pattern::parse("[(*)*]").unwrap();

        // Test cases that should match
        let empty_array = parse_dcbor_item("[]").unwrap();
        let single_element = parse_dcbor_item("[42]").unwrap();
        let multiple_elements = parse_dcbor_item("[1, 2, 3]").unwrap();

        // Empty array should match (zero repetitions)
        let (paths, captures) = pattern.paths_with_captures(&empty_array);
        assert!(!paths.is_empty(), "[(*)*] should match empty array");
        assert!(
            captures.is_empty(),
            "No captures expected for basic quantifier"
        );

        #[rustfmt::skip]
        let expected_empty = indoc! {r#"
            []
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths,
                &captures,
                FormatPathsOpts::default()
            ),
            expected_empty
        );

        // Single element should match (one repetition)
        let (paths, captures) = pattern.paths_with_captures(&single_element);
        assert!(
            !paths.is_empty(),
            "[(*)*] should match single element array"
        );

        #[rustfmt::skip]
        let expected_single = indoc! {r#"
            [42]
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths,
                &captures,
                FormatPathsOpts::default()
            ),
            expected_single
        );

        // Multiple elements should match (multiple repetitions)
        let (paths, captures) =
            pattern.paths_with_captures(&multiple_elements);
        assert!(
            !paths.is_empty(),
            "[(*)*] should match multiple element array"
        );

        #[rustfmt::skip]
        let expected_multiple = indoc! {r#"
            [1, 2, 3]
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths,
                &captures,
                FormatPathsOpts::default()
            ),
            expected_multiple
        );
    }

    #[test]
    fn test_one_or_more_greedy() {
        // Pattern: [(*)+] should match arrays with at least one element
        let pattern = Pattern::parse("[(*)+]").unwrap();

        // Test cases
        let empty_array = parse_dcbor_item("[]").unwrap();
        let single_element = parse_dcbor_item("[42]").unwrap();
        let multiple_elements = parse_dcbor_item("[1, 2, 3]").unwrap();

        // Empty array should NOT match (requires at least one)
        let (paths, captures) = pattern.paths_with_captures(&empty_array);
        assert!(paths.is_empty(), "[(*)+] should NOT match empty array");
        assert!(captures.is_empty(), "No captures expected when no match");

        // Single element should match (one repetition)
        let (paths, captures) = pattern.paths_with_captures(&single_element);
        assert!(
            !paths.is_empty(),
            "[(*)+] should match single element array"
        );

        #[rustfmt::skip]
        let expected_single = indoc! {r#"
            [42]
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths,
                &captures,
                FormatPathsOpts::default()
            ),
            expected_single
        );

        // Multiple elements should match (multiple repetitions)
        let (paths, captures) = pattern.paths_with_captures(&multiple_elements);
        assert!(
            !paths.is_empty(),
            "[(*)+] should match multiple element array"
        );

        #[rustfmt::skip]
        let expected_multiple = indoc! {r#"
            [1, 2, 3]
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths,
                &captures,
                FormatPathsOpts::default()
            ),
            expected_multiple
        );
    }

    #[test]
    fn test_zero_or_one_greedy() {
        // Pattern: [(*)?] should match arrays with zero or one element
        let pattern = Pattern::parse("[(*)?]").unwrap();

        // Test cases
        let empty_array = parse_dcbor_item("[]").unwrap();
        let single_element = parse_dcbor_item("[42]").unwrap();
        let multiple_elements = parse_dcbor_item("[1, 2]").unwrap();

        // Empty array should match (zero repetitions)
        let (paths, captures) = pattern.paths_with_captures(&empty_array);
        assert!(!paths.is_empty(), "[(*)?] should match empty array");

        #[rustfmt::skip]
        let expected_empty = indoc! {r#"
            []
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths,
                &captures,
                FormatPathsOpts::default()
            ),
            expected_empty
        );

        // Single element should match (one repetition)
        let (paths, captures) = pattern.paths_with_captures(&single_element);
        assert!(
            !paths.is_empty(),
            "[(*)?] should match single element array"
        );

        #[rustfmt::skip]
        let expected_single = indoc! {r#"
            [42]
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths,
                &captures,
                FormatPathsOpts::default()
            ),
            expected_single
        );

        // Multiple elements should NOT match (exceeds one repetition)
        let (paths, captures) = pattern.paths_with_captures(&multiple_elements);
        assert!(
            paths.is_empty(),
            "[(*)?] should NOT match multiple element array"
        );
        assert!(captures.is_empty(), "No captures expected when no match");
    }

    #[test]
    fn test_exactly_once_default() {
        // Pattern: [(*)] should match arrays with exactly one element
        // This tests that undecorated parentheses are interpreted as
        // RepeatPattern with "exactly one" quantifier
        let pattern = Pattern::parse("[(*)]").unwrap();

        // Test cases
        let empty_array = parse_dcbor_item("[]").unwrap();
        let single_element = parse_dcbor_item("[42]").unwrap();
        let multiple_elements = parse_dcbor_item("[1, 2]").unwrap();

        // Empty array should NOT match (requires exactly one)
        let (paths, _captures) = pattern.paths_with_captures(&empty_array);
        assert!(paths.is_empty(), "[(*)] should NOT match empty array");

        // Single element should match (exactly one repetition)
        let (paths, captures) = pattern.paths_with_captures(&single_element);
        assert!(
            !paths.is_empty(),
            "[(*)] should match single element array"
        );

        #[rustfmt::skip]
        let expected_single = indoc! {r#"
            [42]
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths,
                &captures,
                FormatPathsOpts::default()
            ),
            expected_single
        );

        // Multiple elements should NOT match (exceeds one repetition)
        let (paths, _captures) =
            pattern.paths_with_captures(&multiple_elements);
        assert!(
            paths.is_empty(),
            "[(*)] should NOT match multiple element array"
        );
    }

    // ============================================================================
    // PHASE 2.2: Lazy Quantifiers - Minimal matching
    // ============================================================================

    #[test]
    fn test_zero_or_more_lazy() {
        // Pattern: [(*)*?] should match arrays but prefer fewer repetitions
        let pattern = Pattern::parse("[(*)*?]").unwrap();

        // Test cases that should match (same as greedy for basic matching)
        let empty_array = parse_dcbor_item("[]").unwrap();
        let single_element = parse_dcbor_item("[42]").unwrap();
        let multiple_elements = parse_dcbor_item("[1, 2, 3]").unwrap();

        // All arrays should match, but lazy behavior is more relevant in
        // complex patterns
        let (paths, _) = pattern.paths_with_captures(&empty_array);
        assert!(!paths.is_empty(), "[(*)*?] should match empty array");

        let (paths, _) = pattern.paths_with_captures(&single_element);
        assert!(
            !paths.is_empty(),
            "[(*)*?] should match single element array"
        );

        let (paths, _) = pattern.paths_with_captures(&multiple_elements);
        assert!(
            !paths.is_empty(),
            "[(*)*?] should match multiple element array"
        );
    }

    #[test]
    fn test_one_or_more_lazy() {
        // Pattern: [(*)+?] should match arrays with at least one element,
        // preferring fewer
        let pattern = Pattern::parse("[(*)+?]").unwrap();

        let empty_array = parse_dcbor_item("[]").unwrap();
        let single_element = parse_dcbor_item("[42]").unwrap();
        let multiple_elements = parse_dcbor_item("[1, 2, 3]").unwrap();

        // Empty array should NOT match (requires at least one)
        let (paths, _) = pattern.paths_with_captures(&empty_array);
        assert!(paths.is_empty(), "[(*)+?] should NOT match empty array");

        // Non-empty arrays should match
        let (paths, _) = pattern.paths_with_captures(&single_element);
        assert!(
            !paths.is_empty(),
            "[(*)+?] should match single element array"
        );

        let (paths, _) = pattern.paths_with_captures(&multiple_elements);
        assert!(
            !paths.is_empty(),
            "[(*)+?] should match multiple element array"
        );
    }

    #[test]
    fn test_zero_or_one_lazy() {
        // Pattern: [(*)??] should match zero or one element, preferring zero
        let pattern = Pattern::parse("[(*)??]").unwrap();

        let empty_array = parse_dcbor_item("[]").unwrap();
        let single_element = parse_dcbor_item("[42]").unwrap();
        let multiple_elements = parse_dcbor_item("[1, 2]").unwrap();

        // Should match empty and single, not multiple
        let (paths, _) = pattern.paths_with_captures(&empty_array);
        assert!(!paths.is_empty(), "[(*)??] should match empty array");

        let (paths, _) = pattern.paths_with_captures(&single_element);
        assert!(
            !paths.is_empty(),
            "[(*)??] should match single element array"
        );

        let (paths, _) = pattern.paths_with_captures(&multiple_elements);
        assert!(
            paths.is_empty(),
            "[(*)??] should NOT match multiple element array"
        );
    }

    // ============================================================================
    // PHASE 2.3: Possessive Quantifiers - No backtracking
    // ============================================================================

    #[test]
    fn test_zero_or_more_possessive() {
        // Pattern: [(*)*+] should match arrays, no backtracking allowed
        let pattern = Pattern::parse("[(*)*+]").unwrap();

        let empty_array = parse_dcbor_item("[]").unwrap();
        let single_element = parse_dcbor_item("[42]").unwrap();
        let multiple_elements = parse_dcbor_item("[1, 2, 3]").unwrap();

        // Should match all arrays (same as greedy for simple cases)
        let (paths, _) = pattern.paths_with_captures(&empty_array);
        assert!(!paths.is_empty(), "[(*)*+] should match empty array");

        let (paths, _) = pattern.paths_with_captures(&single_element);
        assert!(
            !paths.is_empty(),
            "[(*)*+] should match single element array"
        );

        let (paths, _) = pattern.paths_with_captures(&multiple_elements);
        assert!(
            !paths.is_empty(),
            "[(*)*+] should match multiple element array"
        );
    }

    #[test]
    fn test_one_or_more_possessive() {
        // Pattern: [(*)++] should match non-empty arrays, no backtracking
        let pattern = Pattern::parse("[(*)++]").unwrap();

        let empty_array = parse_dcbor_item("[]").unwrap();
        let single_element = parse_dcbor_item("[42]").unwrap();
        let multiple_elements = parse_dcbor_item("[1, 2, 3]").unwrap();

        // Empty should not match, others should
        let (paths, _) = pattern.paths_with_captures(&empty_array);
        assert!(paths.is_empty(), "[(*)++] should NOT match empty array");

        let (paths, _) = pattern.paths_with_captures(&single_element);
        assert!(
            !paths.is_empty(),
            "[(*)++] should match single element array"
        );

        let (paths, _) = pattern.paths_with_captures(&multiple_elements);
        assert!(
            !paths.is_empty(),
            "[(*)++] should match multiple element array"
        );
    }

    #[test]
    fn test_zero_or_one_possessive() {
        // Pattern: [(*)?+] should match zero or one element, no backtracking
        let pattern = Pattern::parse("[(*)?+]").unwrap();

        let empty_array = parse_dcbor_item("[]").unwrap();
        let single_element = parse_dcbor_item("[42]").unwrap();
        let multiple_elements = parse_dcbor_item("[1, 2]").unwrap();

        // Should match zero or one, not multiple
        let (paths, _) = pattern.paths_with_captures(&empty_array);
        assert!(!paths.is_empty(), "[(*)?+] should match empty array");

        let (paths, _) = pattern.paths_with_captures(&single_element);
        assert!(
            !paths.is_empty(),
            "[(*)?+] should match single element array"
        );

        let (paths, _) = pattern.paths_with_captures(&multiple_elements);
        assert!(
            paths.is_empty(),
            "[(*)?+] should NOT match multiple element array"
        );
    }

    // ============================================================================
    // PHASE 2.4: Interval Quantifiers - Exact count ranges
    // ============================================================================

    #[test]
    fn test_exact_count_interval() {
        // Pattern: [(*){3}] should match arrays with exactly 3 elements
        let pattern = Pattern::parse("[(*){3}]").unwrap();

        let empty_array = parse_dcbor_item("[]").unwrap();
        let two_elements = parse_dcbor_item("[1, 2]").unwrap();
        let three_elements = parse_dcbor_item("[1, 2, 3]").unwrap();
        let four_elements = parse_dcbor_item("[1, 2, 3, 4]").unwrap();

        // Only exactly 3 elements should match
        let (paths, _) = pattern.paths_with_captures(&empty_array);
        assert!(
            paths.is_empty(),
            "[(*){{3}}] should NOT match empty array"
        );

        let (paths, _) = pattern.paths_with_captures(&two_elements);
        assert!(
            paths.is_empty(),
            "[(*){{3}}] should NOT match 2-element array"
        );

        let (paths, _) = pattern.paths_with_captures(&three_elements);
        assert!(
            !paths.is_empty(),
            "[(*){{3}}] should match 3-element array"
        );

        #[rustfmt::skip]
        let expected_three = indoc! {r#"
            [1, 2, 3]
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths,
                &HashMap::new(),
                FormatPathsOpts::default()
            ),
            expected_three
        );

        let (paths, _) = pattern.paths_with_captures(&four_elements);
        assert!(
            paths.is_empty(),
            "[(*){{3}}] should NOT match 4-element array"
        );
    }

    #[test]
    fn test_range_interval() {
        // Pattern: [(*){2,4}] should match arrays with 2-4 elements
        let pattern = Pattern::parse("[(*){2,4}]").unwrap();

        let one_element = parse_dcbor_item("[1]").unwrap();
        let two_elements = parse_dcbor_item("[1, 2]").unwrap();
        let three_elements = parse_dcbor_item("[1, 2, 3]").unwrap();
        let four_elements = parse_dcbor_item("[1, 2, 3, 4]").unwrap();
        let five_elements = parse_dcbor_item("[1, 2, 3, 4, 5]").unwrap();

        // Only 2-4 elements should match
        let (paths, _) = pattern.paths_with_captures(&one_element);
        assert!(
            paths.is_empty(),
            "[(*){{2,4}}] should NOT match 1-element array"
        );

        let (paths, _) = pattern.paths_with_captures(&two_elements);
        assert!(
            !paths.is_empty(),
            "[(*){{2,4}}] should match 2-element array"
        );

        let (paths, _) = pattern.paths_with_captures(&three_elements);
        assert!(
            !paths.is_empty(),
            "[(*){{2,4}}] should match 3-element array"
        );

        let (paths, _) = pattern.paths_with_captures(&four_elements);
        assert!(
            !paths.is_empty(),
            "[(*){{2,4}}] should match 4-element array"
        );

        let (paths, _) = pattern.paths_with_captures(&five_elements);
        assert!(
            paths.is_empty(),
            "[(*){{2,4}}] should NOT match 5-element array"
        );
    }

    #[test]
    fn test_minimum_interval() {
        // Pattern: [(*){2,}] should match arrays with at least 2 elements
        let pattern = Pattern::parse("[(*){2,}]").unwrap();

        let one_element = parse_dcbor_item("[1]").unwrap();
        let two_elements = parse_dcbor_item("[1, 2]").unwrap();
        let five_elements = parse_dcbor_item("[1, 2, 3, 4, 5]").unwrap();

        // At least 2 elements should match
        let (paths, _) = pattern.paths_with_captures(&one_element);
        assert!(
            paths.is_empty(),
            "[(*){{2,}}] should NOT match 1-element array"
        );

        let (paths, _) = pattern.paths_with_captures(&two_elements);
        assert!(
            !paths.is_empty(),
            "[(*){{2,}}] should match 2-element array"
        );

        let (paths, _) = pattern.paths_with_captures(&five_elements);
        assert!(
            !paths.is_empty(),
            "[(*){{2,}}] should match 5-element array"
        );
    }

    #[test]
    fn test_maximum_interval() {
        // Pattern: [(*){0,3}] should match arrays with at most 3 elements
        let pattern = Pattern::parse("[(*){0,3}]").unwrap();

        let empty_array = parse_dcbor_item("[]").unwrap();
        let two_elements = parse_dcbor_item("[1, 2]").unwrap();
        let three_elements = parse_dcbor_item("[1, 2, 3]").unwrap();
        let four_elements = parse_dcbor_item("[1, 2, 3, 4]").unwrap();

        // At most 3 elements should match
        let (paths, _) = pattern.paths_with_captures(&empty_array);
        assert!(!paths.is_empty(), "[(*){{0,3}}] should match empty array");

        let (paths, _) = pattern.paths_with_captures(&two_elements);
        assert!(
            !paths.is_empty(),
            "[(*){{0,3}}] should match 2-element array"
        );

        let (paths, _) = pattern.paths_with_captures(&three_elements);
        assert!(
            !paths.is_empty(),
            "[(*){{0,3}}] should match 3-element array"
        );

        let (paths, _) = pattern.paths_with_captures(&four_elements);
        assert!(
            paths.is_empty(),
            "[(*){{0,3}}] should NOT match 4-element array"
        );
    }

    // ============================================================================
    // PHASE 2.5: Complex Scenarios - Combinations and edge cases
    // ============================================================================

    #[test]
    fn test_quantifiers_with_captures() {
        // Test: [(number)*, @item(text)]
        // This should match arrays with zero or more numbers followed by a
        // captured text
        let pattern = Pattern::parse("[(number)*, @item(text)]").unwrap();

        let numbers_then_text = parse_dcbor_item(r#"[1, 2, "hello"]"#).unwrap();
        let only_text = parse_dcbor_item(r#"["hello"]"#).unwrap();
        let only_numbers = parse_dcbor_item("[1, 2]").unwrap();

        // Numbers then text should match and capture the text
        let (paths, captures) =
            pattern.paths_with_captures(&numbers_then_text);
        assert!(
            !paths.is_empty(),
            "[(number)*, @item(text)] should match numbers then text"
        );

        // Should have captured the text item
        assert!(captures.contains_key("item"), "Should have @item capture");

        // Only text should match (zero numbers, one text)
        let (paths, captures) = pattern.paths_with_captures(&only_text);
        assert!(
            !paths.is_empty(),
            "[(number)*, @item(text)] should match only text"
        );
        assert!(
            captures.contains_key("item"),
            "Should have @item capture for text-only"
        );

        // Only numbers should NOT match (missing required text)
        let (paths, captures) = pattern.paths_with_captures(&only_numbers);
        assert!(
            paths.is_empty(),
            "[(number)*, @item(text)] should NOT match only numbers"
        );
        assert!(
            captures.is_empty(),
            "Should have no captures when no match"
        );

        // Test another pattern: [@first(number), (*)*]
        // This captures the first number and allows any additional elements
        let first_capture_pattern =
            Pattern::parse("[@first(number), (*)*]").unwrap();

        let multi_element = parse_dcbor_item(r#"[42, "text", true]"#).unwrap();
        let (paths, captures) =
            first_capture_pattern.paths_with_captures(&multi_element);

        assert!(
            !paths.is_empty(),
            "[@first(number), (*)*] should match multi-element array"
        );
        assert!(
            captures.contains_key("first"),
            "Should capture the first number"
        );

        // Verify the captured value
        if let Some(first_captures) = captures.get("first") {
            assert_eq!(
                first_captures.len(),
                1,
                "Should capture exactly one first element"
            );
        }
    }

    #[test]
    fn test_multiple_quantifiers_in_pattern() {
        // Pattern: [(number)*, (text)+] should match arrays with zero+ numbers
        // followed by one+ texts
        let pattern = Pattern::parse("[(number)*, (text)+]").unwrap();

        let numbers_then_texts =
            parse_dcbor_item(r#"[1, 2, "a", "b"]"#).unwrap();
        let only_texts = parse_dcbor_item(r#"["a", "b"]"#).unwrap();
        let only_numbers = parse_dcbor_item("[1, 2]").unwrap();
        let empty_array = parse_dcbor_item("[]").unwrap();

        // Numbers then texts should match
        let (paths, _) = pattern.paths_with_captures(&numbers_then_texts);
        assert!(
            !paths.is_empty(),
            "[(number)*, (text)+] should match numbers then texts"
        );

        // Only texts should match (zero numbers, one+ texts)
        let (paths, _) = pattern.paths_with_captures(&only_texts);
        assert!(
            !paths.is_empty(),
            "[(number)*, (text)+] should match only texts"
        );

        // Only numbers should NOT match (missing required texts)
        let (paths, _) = pattern.paths_with_captures(&only_numbers);
        assert!(
            paths.is_empty(),
            "[(number)*, (text)+] should NOT match only numbers"
        );

        // Empty should NOT match (missing required texts)
        let (paths, _) = pattern.paths_with_captures(&empty_array);
        assert!(
            paths.is_empty(),
            "[(number)*, (text)+] should NOT match empty array"
        );
    }
}
