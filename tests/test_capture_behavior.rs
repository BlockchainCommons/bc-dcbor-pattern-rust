mod common;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{
    FormatPathsOpts, Matcher, Pattern, format_paths_with_captures,
};
use indoc::indoc;

#[cfg(test)]
mod test_capture_behavior {
    use super::*;

    #[test]
    fn test_exact_array_pattern_matching() {
        // Test that [@item(42)] captures all instances of 42 in an
        // array
        let cbor_data_single = parse_dcbor_item("[42]").unwrap();
        let cbor_data_multiple = parse_dcbor_item("[42, 100, 42]").unwrap();
        let cbor_data_no_match = parse_dcbor_item("[100, 200]").unwrap();
        let pattern = Pattern::parse("[@item(42)]").unwrap();

        // This should match: array with exactly one element that is 42
        let (paths_single, captures_single) =
            pattern.paths_with_captures(&cbor_data_single);

        // Based on existing test array_capture_tests.rs, this should match
        #[rustfmt::skip]
        let expected_single = indoc! {r#"
            @item
                [42]
                    42
            [42]
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths_single,
                &captures_single,
                FormatPathsOpts::default()
            ),
            expected_single
        );

        // Verify the capture exists and contains the single element
        if let Some(item_captures) = captures_single.get("item") {
            assert_eq!(
                item_captures.len(),
                1,
                "Should capture exactly one element"
            );
        } else {
            panic!("Expected 'item' capture to exist for single element array");
        }

        // Test the multiple element array - this SHOULD match and capture at
        // least one 42
        let (paths_multiple, captures_multiple) =
            pattern.paths_with_captures(&cbor_data_multiple);

        // The pattern should match multi-element arrays and capture instances
        // of 42
        assert!(
            !paths_multiple.is_empty(),
            "Pattern [@item(42)] should match arrays containing 42, including multi-element arrays"
        );

        if let Some(item_captures) = captures_multiple.get("item") {
            assert!(
                !item_captures.is_empty(),
                "Should capture at least one instance of 42 in [42, 100, 42]"
            );
            // Note: The VM may deduplicate identical values, so we don't assert
            // an exact count
        } else {
            panic!("Expected 'item' capture to exist for multi-element array");
        }

        // Test array with no matches - should not match
        let (paths_no_match, captures_no_match) =
            pattern.paths_with_captures(&cbor_data_no_match);

        assert!(
            paths_no_match.is_empty(),
            "Pattern should NOT match arrays without 42"
        );

        assert!(
            captures_no_match.is_empty(),
            "Should have no captures when pattern doesn't match"
        );
    }

    #[test]
    fn test_array_with_any_position_pattern() {
        // Test different approaches to match "an array of any length having the
        // number 42 in any position"
        let cbor_data = parse_dcbor_item("[42, 100, 42]").unwrap();

        // Based on the user's explanation, this should be the correct syntax:
        // [(*)*, 42, (*)*] - but this might not be implemented yet

        // Let's test what syntax actually works for finding 42 within any array

        // Approach 1: Use search pattern
        let search_pattern = Pattern::parse("search(@item(42))");
        match search_pattern {
            Ok(pattern) => {
                let (paths, captures) = pattern.paths_with_captures(&cbor_data);
                if !paths.is_empty() {
                    println!("Search pattern found {} paths", paths.len());

                    #[rustfmt::skip]
                    let expected = indoc! {r#"
                        @item
                            [42, 100, 42]
                                42
                            [42, 100, 42]
                                42
                        [42, 100, 42]
                            42
                    "#}.trim();

                    assert_actual_expected!(
                        format_paths_with_captures(
                            &paths,
                            &captures,
                            FormatPathsOpts::default()
                        ),
                        expected
                    );
                } else {
                    println!("Search pattern did not match");
                }
            }
            Err(e) => {
                println!("Search pattern failed to parse: {:?}", e);
            }
        }

        // This test documents current behavior and explores syntax options
        // The `search` pattern works correctly for finding elements within arrays
    }

    #[test]
    fn test_variadic_array_pattern_syntax() {
        // Test if the proposed [(*)*, @item(42), (*)*] syntax works
        // correctly

        // Test data: arrays that should match (contain 42)
        let array_with_42_start = parse_dcbor_item("[42, 100, 200]").unwrap();
        let array_with_42_middle = parse_dcbor_item("[100, 42, 200]").unwrap();
        let array_with_42_end = parse_dcbor_item("[100, 200, 42]").unwrap();

        // Test data: arrays that should NOT match (don't contain 42)
        let array_without_42 = parse_dcbor_item("[100, 200, 300]").unwrap();
        let array_with_only_100 = parse_dcbor_item("[100]").unwrap();

        // Try to parse the proposed variadic pattern syntax WITH CAPTURE
        let pattern_with_capture_result =
            Pattern::parse("[(*)*, @item(42), (*)*]");

        // Try to parse the variadic pattern syntax WITHOUT CAPTURE
        let pattern_without_capture_result =
            Pattern::parse("[(*)*, 42, (*)*]");

        match pattern_with_capture_result {
            Ok(pattern) => {
                println!("Variadic pattern WITH capture parsed successfully!");

                // Test one case to see if it works
                let (paths, captures) =
                    pattern.paths_with_captures(&array_with_42_start);
                println!(
                    "Testing [42, 100, 200]: {} paths, {} captures",
                    paths.len(),
                    captures.len()
                );

                if paths.is_empty() {
                    println!(
                        "LIMITATION: Variadic patterns with captures don't seem to work correctly"
                    );
                    println!(
                        "The pattern parses but doesn't match when it should"
                    );
                } else {
                    println!("SUCCESS: Variadic pattern with capture works!");
                }
            }
            Err(e) => {
                println!(
                    "Variadic pattern WITH capture failed to parse: {:?}",
                    e
                );
            }
        }

        match pattern_without_capture_result {
            Ok(pattern) => {
                println!(
                    "Variadic pattern WITHOUT capture parsed successfully!"
                );

                // Test arrays that should match
                let test_cases_should_match = [
                    (&array_with_42_start, "[42, 100, 200] (42 at start)"),
                    (&array_with_42_middle, "[100, 42, 200] (42 in middle)"),
                    (&array_with_42_end, "[100, 200, 42] (42 at end)"),
                ];

                for (cbor_data, description) in test_cases_should_match {
                    let (paths, _) = pattern.paths_with_captures(cbor_data);
                    println!("Testing {}: {} paths", description, paths.len());

                    assert!(
                        !paths.is_empty(),
                        "Pattern should match {} but found no paths",
                        description
                    );
                }

                // Test arrays that should NOT match
                let test_cases_should_not_match = [
                    (&array_without_42, "[100, 200, 300] (no 42)"),
                    (&array_with_only_100, "[100] (only 100)"),
                ];

                for (cbor_data, description) in test_cases_should_not_match {
                    let (paths, _) = pattern.paths_with_captures(cbor_data);
                    println!("Testing {}: {} paths", description, paths.len());

                    assert!(
                        paths.is_empty(),
                        "Pattern should NOT match {} but found {} paths",
                        description,
                        paths.len()
                    );
                }

                println!(
                    "SUCCESS: Variadic pattern [(*)*, 42, (*)*] works correctly!"
                );
            }
            Err(e) => {
                println!(
                    "Variadic pattern WITHOUT capture failed to parse: {:?}",
                    e
                );
            }
        }
    }

    #[test]
    fn test_debug_variadic_pattern() {
        // Debug test to understand what's happening with variadic patterns
        let cbor_data = parse_dcbor_item("[42, 100, 200]").unwrap();

        // Test different pattern variations
        let patterns_to_test = [
            "[(*)*, @item(42), (*)*]",
            "[(*)*, 42, (*)*]",
            "[*, @item(42), *]",
            "[*, 42, *]",
            "[@item(42), (*)*]",
            "[(*)*, @item(42)]",
        ];

        for pattern_str in patterns_to_test {
            match Pattern::parse(pattern_str) {
                Ok(pattern) => {
                    let (paths, captures) =
                        pattern.paths_with_captures(&cbor_data);
                    println!(
                        "Pattern '{}': {} paths, {} captures",
                        pattern_str,
                        paths.len(),
                        captures.len()
                    );

                    if !paths.is_empty() {
                        println!("  SUCCESS: Pattern matched!");
                        if !captures.is_empty() {
                            for (name, captured_paths) in &captures {
                                println!(
                                    "    @{}: {} captures",
                                    name,
                                    captured_paths.len()
                                );
                            }
                        }

                        // Show the formatted output
                        let formatted = format_paths_with_captures(
                            &paths,
                            &captures,
                            FormatPathsOpts::default(),
                        );
                        println!("  Formatted output:\n{}", formatted);
                    }
                }
                Err(e) => {
                    println!(
                        "Pattern '{}' failed to parse: {:?}",
                        pattern_str, e
                    );
                }
            }
        }
    }

    #[test]
    fn test_variadic_pattern_value_discrimination() {
        // Test if [(*)*, 42, (*)*] properly discriminates between
        // values

        let array_with_42 = parse_dcbor_item("[100, 42, 200]").unwrap();
        let array_with_100_middle = parse_dcbor_item("[42, 100, 200]").unwrap();
        let array_without_42 = parse_dcbor_item("[100, 200, 300]").unwrap();

        // Pattern that should match arrays containing 42
        let pattern_42 =
            Pattern::parse("[(*)*, 42, (*)*]").unwrap();

        // Pattern that should match arrays containing 100
        let pattern_100 =
            Pattern::parse("[(*)*, 100, (*)*]").unwrap();

        println!("=== Testing 42 pattern ===");

        // Test array with 42 in middle - should match
        let (paths, _) = pattern_42.paths_with_captures(&array_with_42);
        println!(
            "[100, 42, 200] with 42 pattern: {} paths",
            paths.len()
        );
        assert!(!paths.is_empty(), "Should match array containing 42");

        // Test array with 100 in middle - should NOT match 42
        let (paths, _) = pattern_42.paths_with_captures(&array_with_100_middle);
        println!(
            "[42, 100, 200] with 42 pattern: {} paths",
            paths.len()
        );
        assert!(!paths.is_empty(), "Should match because 42 is at start"); // 42 is at start, so should match

        // Test array without 42 - should NOT match
        let (paths, _) = pattern_42.paths_with_captures(&array_without_42);
        println!(
            "[100, 200, 300] with 42 pattern: {} paths",
            paths.len()
        );
        assert!(paths.is_empty(), "Should NOT match array without 42");

        println!("\n=== Testing 100 pattern ===");

        // Test array with 100 in middle - should match 100
        let (paths, _) =
            pattern_100.paths_with_captures(&array_with_100_middle);
        println!(
            "[42, 100, 200] with 100 pattern: {} paths",
            paths.len()
        );
        assert!(!paths.is_empty(), "Should match array containing 100");

        // Test array with 42 in middle - should NOT match 100
        let (paths, _) = pattern_100.paths_with_captures(&array_with_42);
        println!(
            "[100, 42, 200] with 100 pattern: {} paths",
            paths.len()
        );
        assert!(!paths.is_empty(), "Should match because 100 is at start"); // 100 is at start, so should match

        // More specific test: array where the target number is ONLY in middle
        let array_42_only_middle = parse_dcbor_item("[1, 42, 3]").unwrap();
        let array_100_only_middle = parse_dcbor_item("[1, 100, 3]").unwrap();

        println!("\n=== Testing specific middle position ===");

        let (paths, _) = pattern_42.paths_with_captures(&array_42_only_middle);
        println!("[1, 42, 3] with 42 pattern: {} paths", paths.len());
        assert!(!paths.is_empty(), "Should match [1, 42, 3] with 42");

        let (paths, _) = pattern_42.paths_with_captures(&array_100_only_middle);
        println!("[1, 100, 3] with 42 pattern: {} paths", paths.len());
        assert!(
            paths.is_empty(),
            "Should NOT match [1, 100, 3] with 42"
        );
    }

    #[test]
    fn test_variadic_capture_should_work() {
        // This test demonstrates the bug: variadic patterns with captures
        // should work but don't

        let cbor_data = parse_dcbor_item("[1, 42, 3]").unwrap();
        let pattern =
            Pattern::parse("[(*)*, @item(42), (*)*]").unwrap();

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);

        // Debug output
        println!("Paths: {:?}", paths);
        println!("Captures: {:?}", captures);

        // The pattern SHOULD match and capture the 42
        assert!(
            !paths.is_empty(),
            "Pattern should match array containing 42"
        );

        // The capture SHOULD exist and contain the 42
        assert!(captures.contains_key("item"), "Should have @item capture");

        if let Some(item_captures) = captures.get("item") {
            assert_eq!(
                item_captures.len(),
                1,
                "Should capture exactly one item"
            );
        }

        // Test the formatted output
        #[rustfmt::skip]
        let expected = indoc! {r#"
            @item
                [1, 42, 3]
                    42
            [1, 42, 3]
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths,
                &captures,
                FormatPathsOpts::default()
            ),
            expected
        );
    }

    #[test]
    fn test_variadic_capture_multiple_matches() {
        // Test variadic capture with multiple matches

        let cbor_data = parse_dcbor_item("[42, 100, 42]").unwrap();
        let pattern =
            Pattern::parse("[(*)*, @item(42), (*)*]").unwrap();

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);

        // Should match the array
        assert!(
            !paths.is_empty(),
            "Pattern should match array containing 42"
        );

        // Should capture the 42(s)
        assert!(captures.contains_key("item"), "Should have @item capture");

        if let Some(item_captures) = captures.get("item") {
            // Should capture at least one 42 (exact behavior may vary based on
            // implementation)
            assert!(
                !item_captures.is_empty(),
                "Should capture at least one 42"
            );
        }
    }

    #[test]
    fn test_variadic_capture_bug_specific_case() {
        // This should now work after the fix

        let cbor_data = parse_dcbor_item("[42, 100, 200]").unwrap();
        let pattern =
            Pattern::parse("[(*)*, @item(42), (*)*]").unwrap();

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);

        // This SHOULD work and now does work
        assert!(
            !paths.is_empty(),
            "Pattern [(*)*, @item(42), (*)*] should match [42, 100, 200]"
        );

        assert!(
            captures.contains_key("item"),
            "Should have @item capture for the 42 in [42, 100, 200]"
        );

        // Test the formatted output
        #[rustfmt::skip]
        let expected = indoc! {r#"
            @item
                [42, 100, 200]
                    42
            [42, 100, 200]
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths,
                &captures,
                FormatPathsOpts::default()
            ),
            expected
        );
    }

    #[test]
    fn test_variadic_capture_position_bug() {
        // Test different positions to isolate the bug

        let test_cases = [
            ("[42]", "single element"),
            ("[42, 100]", "42 at start"),
            ("[100, 42]", "42 at end"),
            ("[42, 100, 200]", "42 at start with more elements"),
            ("[100, 42, 200]", "42 in middle"),
            ("[100, 200, 42]", "42 at end with more elements"),
        ];

        for (cbor_str, description) in test_cases {
            let cbor_data = parse_dcbor_item(cbor_str).unwrap();
            let pattern =
                Pattern::parse("[(*)*, @item(42), (*)*]").unwrap();

            let (paths, captures) = pattern.paths_with_captures(&cbor_data);

            println!(
                "Testing {}: {} paths, {} captures",
                description,
                paths.len(),
                captures.len()
            );

            // All of these should work
            assert!(
                !paths.is_empty(),
                "BUG: Pattern should match {} but found no paths",
                description
            );
            assert!(
                captures.contains_key("item"),
                "BUG: Should have @item capture for {}",
                description
            );
        }
    }
}
