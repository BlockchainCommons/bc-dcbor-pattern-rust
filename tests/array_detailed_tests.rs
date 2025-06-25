mod common;

use dcbor::CBORCase;
use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern, format_paths};
use indoc::indoc;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> dcbor::CBOR { parse_dcbor_item(s).unwrap() }

/// Helper function to parse pattern text into Pattern objects
fn parse(s: &str) -> Pattern { Pattern::parse(s).unwrap() }

#[test]
fn test_array_pattern_paths_with_captures() {
    // Parse the inner capture pattern directly
    let inner_pattern = parse("ARRAY(@item(NUMBER(42)))");
    let cbor_data = cbor("[42]");

    // Test the inner pattern directly on the array
    let (inner_paths, inner_captures) =
        inner_pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected_inner_paths = indoc! {r#"
        [42]
    "#}.trim();
    assert_actual_expected!(format_paths(&inner_paths), expected_inner_paths);

    assert_eq!(inner_captures.len(), 1);
    assert!(inner_captures.contains_key("item"));
    let captured_paths = &inner_captures["item"];
    assert_eq!(captured_paths.len(), 1);
    assert_eq!(captured_paths[0], vec![cbor("[42]"), cbor("42")]);

    // Test the inner pattern on the array element directly
    let element = cbor("42");
    let element_pattern = parse("@item(NUMBER(42))");
    let (element_paths, element_captures) =
        element_pattern.paths_with_captures(&element);

    #[rustfmt::skip]
    let expected_element_paths = indoc! {r#"
        42
    "#}.trim();
    assert_actual_expected!(
        format_paths(&element_paths),
        expected_element_paths
    );

    assert_eq!(element_captures.len(), 1);
    assert!(element_captures.contains_key("item"));
    let element_captured_paths = &element_captures["item"];
    assert_eq!(element_captured_paths.len(), 1);
    assert_eq!(element_captured_paths[0], vec![element]);

    // Test what happens when we call paths() on the inner pattern with the
    // array
    let pattern_paths = inner_pattern.paths(&cbor_data);
    assert_actual_expected!(format_paths(&pattern_paths), expected_inner_paths);
}

#[test]
fn test_array_element_traversal() {
    let cbor_data = cbor("[42]");

    if let CBORCase::Array(arr) = cbor_data.as_case() {
        assert_eq!(arr.len(), 1, "Array should have one element");

        for (i, element) in arr.iter().enumerate() {
            let pattern = parse("@item(NUMBER(42))");
            let (paths, captures) = pattern.paths_with_captures(element);

            #[rustfmt::skip]
            let expected_paths = indoc! {r#"
                42
            "#}.trim();
            assert_actual_expected!(format_paths(&paths), expected_paths);

            assert_eq!(
                captures.len(),
                1,
                "Should have one capture for element {}",
                i
            );
            assert!(captures.contains_key("item"));
            let captured_paths = &captures["item"];
            assert_eq!(captured_paths.len(), 1);
            assert_eq!(
                captured_paths[0],
                vec![element.clone()],
                "Capture should match element {}",
                i
            );
        }
    } else {
        panic!("CBOR data should be an array");
    }
}

#[test]
fn test_array_pattern_with_multiple_elements() {
    let cbor_data = cbor("[42, 100, 200]");
    let pattern = parse("ARRAY(@item(NUMBER))");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // Array patterns can return multiple paths when they match multiple
    // elements
    assert!(!paths.is_empty(), "Should have at least one path");

    // Note: captures will contain multiple values for the same name when
    // matching multiple elements
    assert!(!captures.is_empty(), "Should have captures");
    assert!(captures.contains_key("item"), "Should have item captures");
}

#[test]
fn test_array_pattern_nested_structure() {
    let cbor_data = cbor(r#"[[42], [100]]"#);
    let pattern = parse("ARRAY(@outer_item(ARRAY(@inner_item(NUMBER))))");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // Should match the outer array
    assert!(!paths.is_empty(), "Should have at least one path");

    // Should have captures for both outer and inner items
    assert!(!captures.is_empty(), "Should have captures");
    assert!(
        captures.contains_key("outer_item"),
        "Should have outer_item captures"
    );
    assert!(
        captures.contains_key("inner_item"),
        "Should have inner_item captures"
    );
}

#[test]
fn test_array_pattern_specific_value_matching() {
    let cbor_data = cbor("[42, 100, 42]");
    let pattern = parse("ARRAY(@specific(NUMBER(42)))");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // Should match when array contains the specific value
    assert!(!paths.is_empty(), "Should have at least one path");

    assert!(
        !captures.is_empty(),
        "Should have captures for matching elements"
    );
    assert!(
        captures.contains_key("specific"),
        "Should have specific captures"
    );
}

#[test]
fn test_array_pattern_no_match() {
    let cbor_data = cbor("[100, 200]");
    let pattern = parse("ARRAY(@item(NUMBER(42)))");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // Should have no paths or captures when no elements match
    assert!(
        paths.is_empty(),
        "No paths should be returned for non-matching pattern"
    );
    assert!(
        captures.is_empty(),
        "No captures should be returned for non-matching pattern"
    );
}

#[test]
fn test_array_pattern_mixed_types() {
    let cbor_data = cbor(r#"[42, "hello", true, [1, 2]]"#);
    let pattern = parse("ARRAY(@any_item(ANY))");

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // Should match the array and its elements
    assert!(!paths.is_empty(), "Should have at least one path");

    assert!(
        !captures.is_empty(),
        "Should have captures for all elements"
    );
    assert!(
        captures.contains_key("any_item"),
        "Should have any_item captures"
    );
}
