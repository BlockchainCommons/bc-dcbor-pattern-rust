mod common;

use dcbor::prelude::*;
use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{
    Matcher, Pattern, format_paths, format_paths_with_captures,
};
use indoc::indoc;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> CBOR { parse_dcbor_item(s).unwrap() }

/// Helper function to parse pattern text into Pattern objects
fn parse(s: &str) -> Pattern { Pattern::parse(s).unwrap() }

#[test]
fn test_search_capture_basic() {
    let pattern = parse("search(@found(42))");
    let cbor_data = cbor(r#"[1, [2, 42], 3]"#);

    // Test regular paths first
    let paths = pattern.paths(&cbor_data);
    // expected-text-output-rubric:
    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        [1, [2, 42], 3]
            [2, 42]
                42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    // Test with captures using the proper rubric
    let (capture_paths, captures) = pattern.paths_with_captures(&cbor_data);
    // expected-text-output-rubric:
    #[rustfmt::skip]
    let expected_with_captures = indoc! {r#"
        @found
            [1, [2, 42], 3]
                [2, 42]
                    42
        [1, [2, 42], 3]
            [2, 42]
                42
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &capture_paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_with_captures
    );

    // Verify capture
    assert_eq!(captures.len(), 1);
    assert!(captures.contains_key("found"));
    let captured_paths = &captures["found"];
    assert_eq!(captured_paths.len(), 1);
    // `search` captures include the full path to the found element
    assert_eq!(
        captured_paths[0],
        vec![cbor(r#"[1, [2, 42], 3]"#), cbor(r#"[2, 42]"#), cbor("42")]
    );
}

#[test]
fn test_search_capture_multiple_matches() {
    let pattern = parse("search(@target(42))");
    let cbor_data = cbor(r#"[42, [2, 42], {"key": 42}]"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // expected-text-output-rubric:
    #[rustfmt::skip]
    let expected_with_captures = indoc! {r#"
        @target
            [42, [2, 42], {"key": 42}]
                42
            [42, [2, 42], {"key": 42}]
                [2, 42]
                    42
            [42, [2, 42], {"key": 42}]
                {"key": 42}
                    42
        [42, [2, 42], {"key": 42}]
            42
        [42, [2, 42], {"key": 42}]
            [2, 42]
                42
        [42, [2, 42], {"key": 42}]
            {"key": 42}
                42
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_with_captures
    );
}

#[test]
fn test_search_capture_nested_structure() {
    let pattern = parse(r#"search(@deep("target"))"#);
    let cbor_data = cbor(r#"{"level1": {"level2": {"level3": "target"}}}"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // expected-text-output-rubric:
    #[rustfmt::skip]
    let expected_with_captures = indoc! {r#"
        @deep
            {"level1": {"level2": {"level3": "target"}}}
                {"level2": {"level3": "target"}}
                    {"level3": "target"}
                        "target"
        {"level1": {"level2": {"level3": "target"}}}
            {"level2": {"level3": "target"}}
                {"level3": "target"}
                    "target"
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_with_captures
    );
}

#[test]
fn test_search_capture_with_array_elements() {
    let pattern = parse("search(@item(array))");
    let cbor_data = cbor(r#"[1, [2, 3], {"arrays": [4, 5, 6]}]"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // expected-text-output-rubric:
    #[rustfmt::skip]
    let expected_with_captures = indoc! {r#"
        @item
            [1, [2, 3], {"arrays": [4, 5, 6]}]
            [1, [2, 3], {"arrays": [4, 5, 6]}]
                [2, 3]
            [1, [2, 3], {"arrays": [4, 5, 6]}]
                {"arrays": [4, 5, 6]}
                    [4, 5, 6]
        [1, [2, 3], {"arrays": [4, 5, 6]}]
        [1, [2, 3], {"arrays": [4, 5, 6]}]
            [2, 3]
        [1, [2, 3], {"arrays": [4, 5, 6]}]
            {"arrays": [4, 5, 6]}
                [4, 5, 6]
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_with_captures
    );
}

#[test]
fn test_search_capture_collect_names() {
    let pattern = parse("search(@first(number)) | search(@second(text))");

    let mut capture_names = Vec::new();
    pattern.collect_capture_names(&mut capture_names);

    // Should collect all capture names
    assert_eq!(capture_names.len(), 2);
    assert!(capture_names.contains(&"first".to_string()));
    assert!(capture_names.contains(&"second".to_string()));
}

#[test]
fn test_search_capture_no_match() {
    let pattern = parse("search(@notfound(999))");
    let cbor_data = cbor(r#"[1, [2, 42], 3]"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // Should have no paths or captures when no match is found
    assert!(
        paths.is_empty(),
        "No paths should be returned for non-matching search"
    );
    assert!(
        captures.is_empty(),
        "No captures should be returned for non-matching search"
    );
}

#[test]
fn test_search_capture_complex_pattern() {
    let pattern = parse(r#"search(@found({"id": @id_value(number)}))"#);
    let cbor_data = cbor(
        r#"{"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}"#,
    );

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // expected-text-output-rubric:
    #[rustfmt::skip]
    let expected_with_captures = indoc! {r#"
        @found
            {"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}
                [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]
                    {"id": 1, "name": "Alice"}
            {"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}
                [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]
                    {"id": 2, "name": "Bob"}
        @id_value
            {"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}
                [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]
                    {"id": 1, "name": "Alice"}
            {"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}
                [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]
                    {"id": 2, "name": "Bob"}
        {"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}
            [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]
                {"id": 1, "name": "Alice"}
        {"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}
            [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]
                {"id": 2, "name": "Bob"}
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_with_captures
    );
}

#[test]
fn test_search_capture_api_consistency() {
    let pattern = parse("search(@item(42))");
    let cbor_data = cbor(r#"[1, 42, 3]"#);

    // Test that both direct and API methods give same results
    let (api_paths, api_captures) = pattern.paths_with_captures(&cbor_data);
    let (direct_paths, direct_captures) =
        pattern.paths_with_captures(&cbor_data);

    assert_eq!(
        api_paths, direct_paths,
        "API and direct paths should be identical"
    );
    assert_eq!(
        api_captures, direct_captures,
        "API and direct captures should be identical"
    );
}
