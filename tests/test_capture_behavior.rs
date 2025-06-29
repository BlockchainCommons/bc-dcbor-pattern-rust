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
    fn test_capture_deduplication_behavior() {
        // Test with array that has duplicate values
        let cbor_data = parse_dcbor_item("[42, 100, 42]").unwrap();
        let pattern = Pattern::parse("[@item(NUMBER(42))]").unwrap();

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);

        // Test that we get the expected formatted output with captures
        // The key question: should we capture the value 42 twice (since it
        // appears twice in different positions) or should we
        // deduplicate and only capture it once?

        // Based on the user's request, captures should be unique paths
        // But the paths to [array, 42_at_index_0] and [array, 42_at_index_2]
        // are DIFFERENT paths So both should be captured!

        #[rustfmt::skip]
        let expected = indoc! {r#"
            @item
                [42, 100, 42]
                    42
            [42, 100, 42]
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths,
                &captures,
                FormatPathsOpts::default()
            ),
            expected
        );

        // Also verify the capture count programmatically
        if let Some(item_captures) = captures.get("item") {
            // The pattern only matches NUMBER(42), so we should only capture it
            // once even though it appears twice in the array,
            // because paths are deduplicated
            assert_eq!(
                item_captures.len(),
                1,
                "Should capture the value 42 only once due to path deduplication"
            );
        } else {
            panic!("Expected 'item' capture to exist");
        }
    }

    #[test]
    fn test_what_makes_paths_unique() {
        // Let's understand what makes paths unique in this context
        let cbor_data = parse_dcbor_item("[42, 100, 42]").unwrap();
        let pattern = Pattern::parse("[@item(ANY)]").unwrap();

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);

        // Test the formatted output to understand path uniqueness
        #[rustfmt::skip]
        let expected = indoc! {r#"
            @item
                [42, 100, 42]
                    42
                [42, 100, 42]
                    100
            [42, 100, 42]
                42
            [42, 100, 42]
            [42, 100, 42]
                100
        "#}.trim();

        assert_actual_expected!(
            format_paths_with_captures(
                &paths,
                &captures,
                FormatPathsOpts::default()
            ),
            expected
        );

        // Verify we capture all unique array elements
        if let Some(item_captures) = captures.get("item") {
            assert_eq!(
                item_captures.len(),
                2,
                "Should capture 2 unique elements (42 and 100), with 42 deduplicated"
            );
        } else {
            panic!("Expected 'item' capture to exist");
        }
    }
}
