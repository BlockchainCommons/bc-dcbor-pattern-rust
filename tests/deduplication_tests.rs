#[cfg(test)]
mod deduplication_tests {
    use dcbor_parse::parse_dcbor_item;
    use dcbor_pattern::{Matcher, Pattern};

    #[test]
    fn test_no_duplicate_paths_simple_array() {
        // Test case that previously showed duplicate paths
        let cbor_data = parse_dcbor_item("[42, 100, 200]").unwrap();
        let pattern = Pattern::parse("ARRAY(@item(NUMBER))").unwrap();

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);

        // Check for duplicates in paths
        let mut unique_paths = std::collections::HashSet::new();
        for path in &paths {
            assert!(
                unique_paths.insert(path.clone()),
                "Found duplicate path: {:?}",
                path
            );
        }

        // Verify we have exactly one main path (the array itself)
        assert_eq!(paths.len(), 1, "Should have exactly one path");

        // Check for duplicates in captures
        for (name, captured_paths) in &captures {
            let mut unique_capture_paths = std::collections::HashSet::new();
            for path in captured_paths {
                assert!(
                    unique_capture_paths.insert(path.clone()),
                    "Found duplicate capture path for @{}: {:?}",
                    name,
                    path
                );
            }
        }

        // Verify captures are as expected
        if let Some(item_captures) = captures.get("item") {
            assert_eq!(item_captures.len(), 3, "Should capture all 3 numbers");
        } else {
            panic!("Should have 'item' captures");
        }
    }

    #[test]
    fn test_no_duplicate_paths_nested_array() {
        let nested_cbor = parse_dcbor_item(r#"[[42], [100]]"#).unwrap();
        let nested_pattern = Pattern::parse("ARRAY(@outer_item(ARRAY(@inner_item(NUMBER))))").unwrap();

        let (nested_paths, nested_captures) = nested_pattern.paths_with_captures(&nested_cbor);

        // Check for duplicates in nested paths
        let mut unique_paths = std::collections::HashSet::new();
        for path in &nested_paths {
            assert!(
                unique_paths.insert(path.clone()),
                "Found duplicate nested path: {:?}",
                path
            );
        }

        // Verify we have exactly one main path (the outer array itself)
        assert_eq!(nested_paths.len(), 1, "Should have exactly one nested path");

        // Check for duplicates in nested captures
        for (name, captured_paths) in &nested_captures {
            let mut unique_capture_paths = std::collections::HashSet::new();
            for path in captured_paths {
                assert!(
                    unique_capture_paths.insert(path.clone()),
                    "Found duplicate nested capture path for @{}: {:?}",
                    name,
                    path
                );
            }
        }

        // Verify captures are as expected
        if let Some(outer_captures) = nested_captures.get("outer_item") {
            assert_eq!(outer_captures.len(), 2, "Should capture both inner arrays");
        }
        if let Some(inner_captures) = nested_captures.get("inner_item") {
            assert_eq!(inner_captures.len(), 2, "Should capture both numbers");
        }
    }

    #[test]
    fn test_no_duplicate_paths_with_repeated_values() {
        // Test with actual duplicate values that should create identical paths
        let cbor_data = parse_dcbor_item("[42, 100, 42]").unwrap();
        let pattern = Pattern::parse("ARRAY(@specific(NUMBER(42)))").unwrap();

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);

        // Check for duplicates in paths
        let mut unique_paths = std::collections::HashSet::new();
        for path in &paths {
            assert!(
                unique_paths.insert(path.clone()),
                "Found duplicate path: {:?}",
                path
            );
        }

        // Check for duplicates in captures
        for (name, captured_paths) in &captures {
            let mut unique_capture_paths = std::collections::HashSet::new();
            for path in captured_paths {
                assert!(
                    unique_capture_paths.insert(path.clone()),
                    "Found duplicate capture path for @{}: {:?}",
                    name,
                    path
                );
            }
        }

        // With duplicate values 42, we should only get one captured path since both instances
        // of 42 create identical paths [array, 42]
        if let Some(specific_captures) = captures.get("specific") {
            assert_eq!(specific_captures.len(), 1, "Should deduplicate identical paths to 42");
        }
    }
}
