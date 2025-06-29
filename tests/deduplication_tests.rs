mod common;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{
    FormatPathsOpts, Matcher, Pattern, format_paths_with_captures,
};
use indoc::indoc;

#[cfg(test)]
mod deduplication_tests {
    use super::*;

    #[test]
    fn test_no_duplicate_paths_simple_array() {
        // Test case that previously showed duplicate paths
        let cbor_data = parse_dcbor_item("[42, 100, 200]").unwrap();
        let pattern = Pattern::parse("[@item(number)]").unwrap();

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);

        let actual = format_paths_with_captures(
            &paths,
            &captures,
            FormatPathsOpts::default(),
        );
        #[rustfmt::skip]
        let expected = indoc! {r#"
            @item
                [42, 100, 200]
                    200
                [42, 100, 200]
                    100
                [42, 100, 200]
                    42
            [42, 100, 200]
        "#}.trim();
        assert_actual_expected!(actual, expected);
    }

    #[test]
    fn test_no_duplicate_paths_nested_array() {
        let nested_cbor = parse_dcbor_item(r#"[[42], [100]]"#).unwrap();
        let nested_pattern =
            Pattern::parse("[@outer_item([@inner_item(number)])]")
                .unwrap();

        let (nested_paths, nested_captures) =
            nested_pattern.paths_with_captures(&nested_cbor);

        let actual = format_paths_with_captures(
            &nested_paths,
            &nested_captures,
            FormatPathsOpts::default(),
        );
        #[rustfmt::skip]
        let expected = indoc! {r#"
            @inner_item
                [[42], [100]]
                    [100]
                        100
                [[42], [100]]
                    [42]
                        42
            @outer_item
                [[42], [100]]
                    [100]
                [[42], [100]]
                    [42]
            [[42], [100]]
        "#}.trim();
        assert_actual_expected!(actual, expected);
    }

    #[test]
    fn test_no_duplicate_paths_with_repeated_values() {
        // Test with actual duplicate values that should create identical paths
        let cbor_data = parse_dcbor_item("[42, 100, 42]").unwrap();
        let pattern = Pattern::parse("[@specific(42)]").unwrap();

        let (paths, captures) = pattern.paths_with_captures(&cbor_data);

        let actual = format_paths_with_captures(
            &paths,
            &captures,
            FormatPathsOpts::default(),
        );
        #[rustfmt::skip]
        let expected = indoc! {r#"
            @specific
                [42, 100, 42]
                    42
            [42, 100, 42]
        "#}.trim();
        assert_actual_expected!(actual, expected);
    }
}
