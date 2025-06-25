mod common;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern, format_paths};
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

    // Test with captures
    let (capture_paths, captures) = pattern.paths_with_captures(&cbor_data);
    assert_actual_expected!(format_paths(&capture_paths), expected_paths);

    // Verify captures
    assert_eq!(captures.len(), 2);
    assert!(captures.contains_key("key"));
    assert!(captures.contains_key("value"));
    let key_captured = &captures["key"];
    let value_captured = &captures["value"];
    assert_eq!(key_captured.len(), 1);
    assert_eq!(value_captured.len(), 1);
    // Map captures include path from map to element
    assert_eq!(
        key_captured[0],
        vec![cbor(r#"{"name": "Alice"}"#), cbor(r#""name""#)]
    );
    assert_eq!(
        value_captured[0],
        vec![cbor(r#"{"name": "Alice"}"#), cbor(r#""Alice""#)]
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
    let expected_paths = indoc! {r#"
        {"age": 30, "name": "Bob"}
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    // Verify captures
    assert_eq!(captures.len(), 4);
    assert!(captures.contains_key("name_key"));
    assert!(captures.contains_key("name_val"));
    assert!(captures.contains_key("age_key"));
    assert!(captures.contains_key("age_val"));
    let map_data = cbor(r#"{"name": "Bob", "age": 30}"#);
    assert_eq!(
        captures["name_key"][0],
        vec![map_data.clone(), cbor(r#""name""#)]
    );
    assert_eq!(
        captures["name_val"][0],
        vec![map_data.clone(), cbor(r#""Bob""#)]
    );
    assert_eq!(
        captures["age_key"][0],
        vec![map_data.clone(), cbor(r#""age""#)]
    );
    assert_eq!(captures["age_val"][0], vec![map_data, cbor("30")]);
}

#[test]
fn test_map_capture_value_only() {
    let pattern = parse(r#"MAP(TEXT("status"): @status(TEXT))"#);
    let cbor_data = cbor(r#"{"status": "active"}"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        {"status": "active"}
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    // Verify single capture
    assert_eq!(captures.len(), 1);
    assert!(captures.contains_key("status"));
    let captured_paths = &captures["status"];
    assert_eq!(captured_paths.len(), 1);
    assert_eq!(
        captured_paths[0],
        vec![cbor(r#"{"status": "active"}"#), cbor(r#""active""#)]
    );
}

#[test]
fn test_map_capture_with_any_pattern() {
    let pattern = parse(r#"MAP(@any_key(TEXT): @any_value(ANY))"#);
    let cbor_data = cbor(r#"{"hello": [1, 2, 3]}"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        {"hello": [1, 2, 3]}
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    // Verify captures
    assert_eq!(captures.len(), 2);
    assert!(captures.contains_key("any_key"));
    assert!(captures.contains_key("any_value"));
    let map_data = cbor(r#"{"hello": [1, 2, 3]}"#);
    assert_eq!(
        captures["any_key"][0],
        vec![map_data.clone(), cbor(r#""hello""#)]
    );
    assert_eq!(captures["any_value"][0], vec![map_data, cbor("[1, 2, 3]")]);
}

#[test]
fn test_map_capture_nested() {
    let pattern = parse(r#"MAP(TEXT("data"): @inner(ARRAY))"#);
    let cbor_data = cbor(r#"{"data": [42, 100]}"#);

    let (paths, captures) = pattern.paths_with_captures(&cbor_data);

    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        {"data": [42, 100]}
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    // Verify captures
    assert_eq!(captures.len(), 1);

    assert!(captures.contains_key("inner"));
    let inner_captured = &captures["inner"];
    assert_eq!(inner_captured.len(), 1);
    assert_eq!(
        inner_captured[0],
        vec![cbor(r#"{"data": [42, 100]}"#), cbor("[42, 100]")]
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
