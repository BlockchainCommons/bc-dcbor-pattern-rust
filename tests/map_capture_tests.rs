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
fn test_map_capture_key_value() {
    let pattern = parse(r#"{@key("name"): @value("Alice")}"#);
    let cbor_data = cbor(r#"{"name": "Alice"}"#);

    // Test regular paths first
    let paths = pattern.paths(&cbor_data);
    // expected-text-output-rubric:
    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        {"name": "Alice"}
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    // Test with captures using the proper rubric
    let (capture_paths, captures) = pattern.paths_with_captures(&cbor_data);
    // expected-text-output-rubric:
    #[rustfmt::skip]
    let expected_with_captures = indoc! {r#"
        @key
            {"name": "Alice"}
                "name"
        @value
            {"name": "Alice"}
                "Alice"
        {"name": "Alice"}
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &capture_paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_with_captures
    );
}

#[test]
fn test_map_capture_multiple_entries() {
    let pattern = parse(
        r#"{@name_key("name"): @name_val(text), @age_key("age"): @age_val(number)}"#,
    );
    let cbor_data = cbor(r#"{"name": "Bob", "age": 30}"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // expected-text-output-rubric:
    #[rustfmt::skip]
    let expected_with_captures = indoc! {r#"
        @age_key
            {"age": 30, "name": "Bob"}
                "age"
        @age_val
            {"age": 30, "name": "Bob"}
                30
        @name_key
            {"age": 30, "name": "Bob"}
                "name"
        @name_val
            {"age": 30, "name": "Bob"}
                "Bob"
        {"age": 30, "name": "Bob"}
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
fn test_map_capture_value_only() {
    let pattern = parse(r#"{"status": @status(text)}"#);
    let cbor_data = cbor(r#"{"status": "active"}"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // expected-text-output-rubric:
    #[rustfmt::skip]
    let expected_with_captures = indoc! {r#"
        @status
            {"status": "active"}
                "active"
        {"status": "active"}
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
fn test_map_capture_with_any_pattern() {
    let pattern = parse(r#"{@any_key(text): @any_value(*)}"#);
    let cbor_data = cbor(r#"{"hello": [1, 2, 3]}"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // expected-text-output-rubric:
    #[rustfmt::skip]
    let expected_with_captures = indoc! {r#"
        @any_key
            {"hello": [1, 2, 3]}
                "hello"
        @any_value
            {"hello": [1, 2, 3]}
                [1, 2, 3]
        {"hello": [1, 2, 3]}
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
fn test_map_capture_nested() {
    let pattern = parse(r#"{"data": @inner(array)}"#);
    let cbor_data = cbor(r#"{"data": [42, 100]}"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    // expected-text-output-rubric:
    #[rustfmt::skip]
    let expected_with_captures = indoc! {r#"
        @inner
            {"data": [42, 100]}
                [42, 100]
        {"data": [42, 100]}
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
fn test_map_capture_collect_names() {
    let pattern =
        parse(r#"{@key1(text): @val1(number), @key2(text): @val2(text)}"#);

    let mut capture_names = Vec::new();
    pattern.collect_capture_names(&mut capture_names);

    // Should collect all capture names
    assert_eq!(capture_names.len(), 4);
    assert!(capture_names.contains(&"key1".to_string()));
    assert!(capture_names.contains(&"val1".to_string()));
    assert!(capture_names.contains(&"key2".to_string()));
    assert!(capture_names.contains(&"val2".to_string()));
}

#[test]
fn test_map_capture_non_matching() {
    let pattern = parse(r#"{@key("name"): @value("Alice")}"#);
    let cbor_data = cbor(r#"{"name": "Bob"}"#); // Different value

    // Should not match
    assert!(!pattern.matches(&cbor_data));

    let paths = pattern.paths(&cbor_data);
    assert!(
        paths.is_empty(),
        "No paths should be returned for non-matching pattern"
    );
}
