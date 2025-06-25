mod common;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{
    Matcher, Pattern, format_paths, format_paths_with_captures,
};
use indoc::indoc;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> dcbor::CBOR { parse_dcbor_item(s).unwrap() }

/// Helper function to parse pattern text into Pattern objects
fn parse(s: &str) -> Pattern { Pattern::parse(s).unwrap() }

#[test]
fn test_map_capture_key_value() {
    let pattern = parse(r#"MAP(@key(TEXT("name")): @value(TEXT("Alice")))"#);
    let cbor_data = cbor(r#"{"name": "Alice"}"#);

    // Test regular paths first
    let paths = pattern.paths(&cbor_data);
    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        {"name": "Alice"}
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    // Test with captures using the proper rubric
    let (capture_paths, captures) = pattern.paths_with_captures(&cbor_data);
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
        r#"MAP(@name_key(TEXT("name")): @name_val(TEXT), @age_key(TEXT("age")): @age_val(NUMBER))"#,
    );
    let cbor_data = cbor(r#"{"name": "Bob", "age": 30}"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

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
    let pattern = parse(r#"MAP(TEXT("status"): @status(TEXT))"#);
    let cbor_data = cbor(r#"{"status": "active"}"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

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
    let pattern = parse(r#"MAP(@any_key(TEXT): @any_value(ANY))"#);
    let cbor_data = cbor(r#"{"hello": [1, 2, 3]}"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

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
    let pattern = parse(r#"MAP(TEXT("data"): @inner(ARRAY))"#);
    let cbor_data = cbor(r#"{"data": [42, 100]}"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

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
        parse(r#"MAP(@key1(TEXT): @val1(NUMBER), @key2(TEXT): @val2(TEXT))"#);

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
    let pattern = parse(r#"MAP(@key(TEXT("name")): @value(TEXT("Alice")))"#);
    let cbor_data = cbor(r#"{"name": "Bob"}"#); // Different value

    // Should not match
    assert!(!pattern.matches(&cbor_data));

    let paths = pattern.paths(&cbor_data);
    assert!(
        paths.is_empty(),
        "No paths should be returned for non-matching pattern"
    );
}
