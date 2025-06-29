mod common;

use dcbor::CBORCase;
use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{
    FormatPathsOpts, Matcher, Pattern, format_paths, format_paths_with_captures,
};
use indoc::indoc;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> dcbor::CBOR { parse_dcbor_item(s).unwrap() }

#[test]
fn test_array_pattern_paths_with_captures() {
    // Parse the inner capture pattern directly
    let inner_pattern = Pattern::parse("[@item(42)]").unwrap();
    let cbor_data = cbor("[42]");

    // Test the inner pattern directly on the array
    let (inner_paths, inner_captures) =
        inner_pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected_inner = indoc! {r#"
        @item
            [42]
                42
        [42]
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &inner_paths,
            &inner_captures,
            FormatPathsOpts::default()
        ),
        expected_inner
    );

    // Test the inner pattern on the array element directly
    let element = cbor("42");
    let element_pattern = Pattern::parse("@item(42)").unwrap();
    let (element_paths, element_captures) =
        element_pattern.paths_with_captures(&element);

    #[rustfmt::skip]
    let expected_element = indoc! {r#"
        @item
            42
        42
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &element_paths,
            &element_captures,
            FormatPathsOpts::default()
        ),
        expected_element
    );

    // Test what happens when we call paths() on the inner pattern with the
    // array
    let pattern_paths = inner_pattern.paths(&cbor_data);
    #[rustfmt::skip]
    let expected_paths_only = indoc! {r#"
        [42]
    "#}.trim();
    assert_actual_expected!(format_paths(&pattern_paths), expected_paths_only);
}

#[test]
fn test_array_element_traversal() {
    let cbor_data = cbor("[42]");

    if let CBORCase::Array(arr) = cbor_data.as_case() {
        assert_eq!(arr.len(), 1, "Array should have one element");

        for element in arr.iter() {
            let pattern = Pattern::parse("@item(42)").unwrap();
            let (paths, captures) = pattern.paths_with_captures(element);

            #[rustfmt::skip]
            let expected = indoc! {r#"
                @item
                    42
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
        }
    } else {
        panic!("CBOR data should be an array");
    }
}

#[test]
fn test_array_pattern_with_multiple_elements() {
    let cbor_data = cbor("[42, 100, 200]");
    let pattern = Pattern::parse("[@item(number)]").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

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
fn test_array_pattern_nested_structure() {
    let cbor_data = cbor(r#"[[42], [100]]"#);
    let pattern = Pattern::parse("[@outer_item([@inner_item(number)])]").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

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
fn test_array_pattern_specific_value_matching() {
    let cbor_data = cbor("[42, 100, 42]");
    let pattern = Pattern::parse("[@specific(42)]").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected = indoc! {r#"
        @specific
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
}

#[test]
fn test_array_pattern_no_match() {
    let cbor_data = cbor("[100, 200]");
    let pattern = Pattern::parse("[@item(42)]").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected = indoc! {r#"

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
fn test_array_pattern_mixed_types() {
    let cbor_data = cbor(r#"[42, "hello", true, [1, 2]]"#);
    let pattern = Pattern::parse("[@any_item(ANY)]").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected = indoc! {r#"
        @any_item
            [42, "hello", true, [1, 2]]
                [1, 2]
            [42, "hello", true, [1, 2]]
                true
            [42, "hello", true, [1, 2]]
                "hello"
            [42, "hello", true, [1, 2]]
                42
        [42, "hello", true, [1, 2]]
            [1, 2]
        [42, "hello", true, [1, 2]]
        [42, "hello", true, [1, 2]]
            true
        [42, "hello", true, [1, 2]]
            "hello"
        [42, "hello", true, [1, 2]]
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
}
