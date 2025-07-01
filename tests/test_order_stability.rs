#[cfg(test)]
mod test_order_stability {
    use dcbor_parse::parse_dcbor_item;
    use dcbor_pattern::{Matcher, Pattern};

    #[test]
    fn test_deterministic_order_with_multiple_paths() {
        // Create a scenario that would generate multiple paths in a predictable
        // order
        let cbor_data = parse_dcbor_item(r#"[[1], [2], [3], [1]]"#).unwrap();
        let pattern = Pattern::parse("[@outer([@inner(number)])]").unwrap();

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);

        // Record the exact order we get
        let first_run_paths = paths.clone();
        let first_run_captures = captures.clone();

        // Run the same pattern many times and verify we always get the same
        // order
        for i in 0..100 {
            let (test_paths, test_captures) =
                pattern.paths_with_captures(&cbor_data);

            assert_eq!(
                test_paths, first_run_paths,
                "Paths order differed on iteration {}",
                i
            );
            assert_eq!(
                test_captures, first_run_captures,
                "Captures order differed on iteration {}",
                i
            );
        }

        // Additionally verify the structure makes sense
        assert_eq!(
            paths.len(),
            1,
            "Should have exactly one main path (the array)"
        );

        if let Some(outer_captures) = captures.get("outer") {
            // Should have [1], [2], [3] captured (deduplicated, [1] appears
            // twice but creates same path)
            assert_eq!(
                outer_captures.len(),
                3,
                "Should have 3 unique outer captures"
            );
        }

        if let Some(inner_captures) = captures.get("inner") {
            // Should have 1, 2, 3 captured (deduplicated)
            assert_eq!(
                inner_captures.len(),
                3,
                "Should have 3 unique inner captures"
            );
        }
    }

    #[test]
    fn test_order_preserved_across_hash_boundaries() {
        // Test with values that are likely to hash differently
        let cbor_data =
            parse_dcbor_item(r#"[1, 1000000, 2, 1000000, 3]"#).unwrap();
        let pattern = Pattern::parse("[@item(number)]").unwrap();

        let first_run = pattern.paths_with_captures(&cbor_data);

        // Verify deterministic behavior across many runs
        for i in 0..50 {
            let test_run = pattern.paths_with_captures(&cbor_data);
            assert_eq!(
                test_run.0, first_run.0,
                "Paths order changed on iteration {}",
                i
            );
            assert_eq!(
                test_run.1, first_run.1,
                "Captures order changed on iteration {}",
                i
            );
        }

        // Verify we get the expected deduplication
        if let Some(item_captures) = first_run.1.get("item") {
            // Should capture: 1, 1000000, 2, 3 (in order of first appearance,
            // duplicates removed)
            assert_eq!(
                item_captures.len(),
                4,
                "Should have 4 unique captured values"
            );
        }
    }
}
