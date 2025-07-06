//! Test to verify that the search pattern capture bug is fixed.
//!
//! This test demonstrates that search patterns with array captures now
//! behave consistently with non-search patterns, preserving element-level
//! captures while maintaining compatibility with complex pattern captures.

#[cfg(test)]
mod search_captures_fix_test {
    use crate::{Matcher, Pattern};
    use dcbor_parse::parse_dcbor_item;

    /// Helper function to parse CBOR diagnostic notation into CBOR objects
    fn cbor(s: &str) -> dcbor::prelude::CBOR {
        parse_dcbor_item(s).unwrap()
    }

    /// Helper function to parse pattern text into Pattern objects
    fn parse(s: &str) -> Pattern {
        Pattern::parse(s).unwrap()
    }

    #[test]
    fn test_array_pattern_captures_consistency() {
        // Test data
        let cbor_data = cbor("[1, 2, 3]");

        // Test without search
        let pattern_without_search = parse("[@a(*), @b(*), @c(*)]");
        let (_paths1, captures1) =
            pattern_without_search.paths_with_captures(&cbor_data);

        // Test with search
        let pattern_with_search = parse("search([@a(*), @b(*), @c(*)])");
        let (_paths2, captures2) =
            pattern_with_search.paths_with_captures(&cbor_data);

        // Verify captures are identical
        assert_eq!(captures1.len(), 3, "Should have 3 captures without search");
        assert_eq!(captures2.len(), 3, "Should have 3 captures with search");

        for capture_name in ["a", "b", "c"] {
            assert!(
                captures1.contains_key(capture_name),
                "Capture '{}' should exist without search",
                capture_name
            );
            assert!(
                captures2.contains_key(capture_name),
                "Capture '{}' should exist with search",
                capture_name
            );

            // The key fix: captures should be identical between search and non-search
            assert_eq!(
                captures1[capture_name], captures2[capture_name],
                "Capture '{}' should be identical with and without search",
                capture_name
            );
        }

        // Verify the specific capture paths are correct (element-level)
        assert_eq!(
            captures1["a"].len(),
            1,
            "Capture 'a' should have exactly one path"
        );
        assert_eq!(
            captures1["b"].len(),
            1,
            "Capture 'b' should have exactly one path"
        );
        assert_eq!(
            captures1["c"].len(),
            1,
            "Capture 'c' should have exactly one path"
        );

        // Each capture should point to the array and the specific element
        assert_eq!(
            captures1["a"][0].len(),
            2,
            "Capture 'a' should have array + element path"
        );
        assert_eq!(
            captures1["b"][0].len(),
            2,
            "Capture 'b' should have array + element path"
        );
        assert_eq!(
            captures1["c"][0].len(),
            2,
            "Capture 'c' should have array + element path"
        );
    }

    #[test]
    fn test_complex_pattern_captures_still_work() {
        // Verify that complex patterns still work as expected after the fix
        let pattern = parse(r#"search(@found({"id": @id_value(number)}))"#);
        let cbor_data = cbor(
            r#"{"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}"#,
        );

        let (_paths, captures) = pattern.paths_with_captures(&cbor_data);

        // Should have found both objects
        assert_eq!(captures.len(), 2, "Should have 2 different capture names");
        assert!(
            captures.contains_key("found"),
            "Should have 'found' captures"
        );
        assert!(
            captures.contains_key("id_value"),
            "Should have 'id_value' captures"
        );

        // Should have captured 2 instances of each (one for each user)
        assert_eq!(captures["found"].len(), 2, "Should have found 2 objects");
        assert_eq!(
            captures["id_value"].len(),
            2,
            "Should have captured 2 id values"
        );

        // Each capture should include the full path from root to the matched object
        for capture_path in &captures["found"] {
            assert!(
                capture_path.len() >= 3,
                "Complex pattern captures should include full search path"
            );
        }

        for capture_path in &captures["id_value"] {
            assert!(
                capture_path.len() >= 3,
                "Complex pattern captures should include full search path"
            );
        }
    }

    #[test]
    fn test_simple_search_captures_unchanged() {
        // Verify that simple search captures continue to work as before
        let pattern = parse("search(@found(42))");
        let cbor_data = cbor("[1, [2, 42], 3]");

        let (_paths, captures) = pattern.paths_with_captures(&cbor_data);

        assert_eq!(captures.len(), 1, "Should have 1 capture");
        assert!(
            captures.contains_key("found"),
            "Should have 'found' capture"
        );
        assert_eq!(captures["found"].len(), 1, "Should have found 1 instance");

        // Should include the full path to the found element
        let capture_path = &captures["found"][0];
        assert_eq!(
            capture_path.len(),
            3,
            "Should have path: root -> array -> 42"
        );
    }
}
