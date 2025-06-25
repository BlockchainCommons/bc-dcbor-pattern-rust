#[cfg(test)]
mod test_order_preservation {
    use dcbor_parse::parse_dcbor_item;
    use dcbor_pattern::{Matcher, Pattern};

    #[test]
    fn test_path_order_deterministic() {
        let cbor_data = parse_dcbor_item("[42, 100, 200]").unwrap();
        let pattern = Pattern::parse("ARRAY(@item(NUMBER))").unwrap();

        // Run the same pattern multiple times to check for deterministic
        // ordering
        let mut all_results = Vec::new();

        for _ in 0..10 {
            let (paths, captures) = pattern.paths_with_captures(&cbor_data);
            all_results.push((paths, captures));
        }

        // All results should be identical
        let first_result = &all_results[0];
        for result in &all_results[1..] {
            assert_eq!(
                result.0, first_result.0,
                "Paths should be deterministic"
            );
            assert_eq!(
                result.1, first_result.1,
                "Captures should be deterministic"
            );
        }

        // Verify we have exactly one path (not duplicates)
        assert_eq!(
            first_result.0.len(),
            1,
            "Should have exactly one deduplicated path"
        );

        // Verify we have the expected number of captures
        if let Some(item_captures) = first_result.1.get("item") {
            assert_eq!(item_captures.len(), 3, "Should have 3 captured items");
        } else {
            panic!("Should have 'item' captures");
        }
    }

    #[test]
    fn test_capture_order_deterministic() {
        let cbor_data = parse_dcbor_item("[1, 2, 3, 1, 2, 3]").unwrap(); // Intentional duplicates
        let pattern = Pattern::parse("ARRAY(@num(NUMBER))").unwrap();

        // Run multiple times to check deterministic ordering
        let mut all_results = Vec::new();

        for _ in 0..10 {
            let (paths, captures) = pattern.paths_with_captures(&cbor_data);
            all_results.push((paths, captures));
        }

        // All results should be identical
        let first_result = &all_results[0];
        for result in &all_results[1..] {
            assert_eq!(
                result.0, first_result.0,
                "Paths should be deterministic"
            );
            assert_eq!(
                result.1, first_result.1,
                "Captures should be deterministic"
            );
        }

        // Check that captures are deduplicated properly
        if let Some(num_captures) = first_result.1.get("num") {
            // We should have captured paths [array,1], [array,2], [array,3]
            // (deduplicated but order preserved) Since we have
            // duplicate values 1,2,3 appearing twice, but they create identical
            // paths, we should only see 3 unique captured paths,
            // not 6
            assert_eq!(
                num_captures.len(),
                3,
                "Should capture 3 unique paths (deduplicated)"
            );
        } else {
            panic!("Should have 'num' captures");
        }
    }
}
