mod common;

use dcbor::prelude::*;
use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{MapPattern, Matcher, Pattern, format_paths};

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> CBOR { parse_dcbor_item(s).unwrap() }

#[test]
fn test_map_patterns_with_real_cbor() {
    // Create test maps using CBOR diagnostic notation
    let empty_map = cbor("{}");
    let single_item = cbor(r#"{"key": "hello"}"#);
    let three_items = cbor(r#"{"a": 1, "b": 2, "c": 3}"#);
    let large_map = cbor(
        r#"{0: "item0", 1: "item1", 2: "item2", 3: "item3", 4: "item4", 5: "item5", 6: "item6", 7: "item7", 8: "item8", 9: "item9"}"#,
    );

    // Test MAP (any map)
    let any_map = Pattern::parse("MAP").unwrap();

    // Should match empty map
    let paths = any_map.paths(&empty_map);
    let expected = "{}";
    assert_actual_expected!(format_paths(&paths), expected);

    // Should match single item map
    let paths = any_map.paths(&single_item);
    let expected = r#"{"key": "hello"}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Should match three items map
    let paths = any_map.paths(&three_items);
    let expected = r#"{"a": 1, "b": 2, "c": 3}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Should match large map
    let paths = any_map.paths(&large_map);
    let expected = r#"{0: "item0", 1: "item1", 2: "item2", 3: "item3", 4: "item4", 5: "item5", 6: "item6", 7: "item7", 8: "item8", 9: "item9"}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match non-map
    assert!(!any_map.matches(&1.to_cbor()));

    // Test MAP({0}) - empty map
    let empty_pattern = Pattern::parse("MAP({0})").unwrap();
    let paths = empty_pattern.paths(&empty_map);
    let expected = "{}";
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match other maps
    assert!(!empty_pattern.matches(&single_item));
    assert!(!empty_pattern.matches(&three_items));

    // Test MAP({1}) - single item map
    let single_pattern = Pattern::parse("MAP({1})").unwrap();
    let paths = single_pattern.paths(&single_item);
    let expected = r#"{"key": "hello"}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match other maps
    assert!(!single_pattern.matches(&empty_map));
    assert!(!single_pattern.matches(&three_items));

    // Test MAP({3}) - three item map
    let three_pattern = Pattern::parse("MAP({3})").unwrap();
    let paths = three_pattern.paths(&three_items);
    let expected = r#"{"a": 1, "b": 2, "c": 3}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match other maps
    assert!(!three_pattern.matches(&empty_map));
    assert!(!three_pattern.matches(&single_item));
    assert!(!three_pattern.matches(&large_map));

    // Test MAP({5,15}) - range pattern
    let range_pattern = Pattern::parse("MAP({5,15})").unwrap();
    let paths = range_pattern.paths(&large_map);
    let expected = r#"{0: "item0", 1: "item1", 2: "item2", 3: "item3", 4: "item4", 5: "item5", 6: "item6", 7: "item7", 8: "item8", 9: "item9"}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match smaller maps
    assert!(!range_pattern.matches(&empty_map));
    assert!(!range_pattern.matches(&single_item));
    assert!(!range_pattern.matches(&three_items));

    // Test MAP({5,}) - at least 5 items
    let min_pattern = Pattern::parse("MAP({5,})").unwrap();
    let paths = min_pattern.paths(&large_map);
    let expected = r#"{0: "item0", 1: "item1", 2: "item2", 3: "item3", 4: "item4", 5: "item5", 6: "item6", 7: "item7", 8: "item8", 9: "item9"}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match smaller maps
    assert!(!min_pattern.matches(&empty_map));
    assert!(!min_pattern.matches(&single_item));
    assert!(!min_pattern.matches(&three_items));
}

#[test]
fn test_map_pattern_display() {
    assert_eq!(MapPattern::any().to_string(), "MAP");
    assert_eq!(MapPattern::with_length(0).to_string(), "MAP({0})");
    assert_eq!(MapPattern::with_length(5).to_string(), "MAP({5})");
    assert_eq!(
        MapPattern::with_length_range(2..=8).to_string(),
        "MAP({2,8})"
    );
    assert_eq!(
        MapPattern::with_length_range(3..=usize::MAX).to_string(),
        "MAP({3,})"
    );
}

#[test]
fn test_map_pattern_round_trip() {
    let patterns = vec![
        "MAP",
        "MAP({0})",
        "MAP({1})",
        "MAP({5})",
        "MAP({2,8})",
        "MAP({3,})",
    ];

    for pattern_str in patterns {
        let pattern = Pattern::parse(pattern_str).unwrap();
        assert_eq!(pattern.to_string(), pattern_str);
    }
}

#[test]
fn test_map_pattern_paths() {
    // Create a test map
    let test_map = cbor(r#"{"key1": "value1", "key2": 42}"#);

    // Test that MAP pattern returns the map itself as a path
    let any_map = MapPattern::any();
    let paths = any_map.paths(&test_map);
    let expected = r#"{"key1": "value1", "key2": 42}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Test with non-map data - should return no paths
    let not_map = cbor(r#""not a map""#);
    let paths = any_map.paths(&not_map);
    assert_eq!(paths.len(), 0);

    // Test exact length match
    let exact_pattern = MapPattern::with_length(2);
    let paths = exact_pattern.paths(&test_map);
    let expected = r#"{"key1": "value1", "key2": 42}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Test length mismatch - should return no paths
    let wrong_length = MapPattern::with_length(3);
    let paths = wrong_length.paths(&test_map);
    assert_eq!(paths.len(), 0);
}

#[test]
fn test_map_key_value_constraints_single() {
    // Test map with single key-value constraint
    let test_map = cbor(r#"{"name": "Alice", "age": 30, "city": "New York"}"#);

    // Single constraint: name must be a text value
    let pattern = MapPattern::with_key_value_constraints(vec![(
        Pattern::text("name"),
        Pattern::any_text(),
    )]);

    let paths = pattern.paths(&test_map);
    let expected = r#"{"age": 30, "city": "New York", "name": "Alice"}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Test non-matching constraint
    let non_matching = MapPattern::with_key_value_constraints(vec![
        (Pattern::text("name"), Pattern::any_number()), /* name is text, not
                                                         * number */
    ]);

    let paths = non_matching.paths(&test_map);
    assert_eq!(paths.len(), 0);
}

#[test]
fn test_map_key_value_constraints_multiple() {
    // Test map with multiple key-value constraints
    let test_map = cbor(r#"{"name": "Bob", "age": 25, "active": true}"#);

    // Multiple constraints: all must be satisfied
    let pattern = MapPattern::with_key_value_constraints(vec![
        (Pattern::text("name"), Pattern::any_text()),
        (Pattern::text("age"), Pattern::any_number()),
        (Pattern::text("active"), Pattern::any_bool()),
    ]);

    let paths = pattern.paths(&test_map);
    let expected = r#"{"age": 25, "name": "Bob", "active": true}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Test with one failing constraint
    let partial_pattern = MapPattern::with_key_value_constraints(vec![
        (Pattern::text("name"), Pattern::any_text()), // matches
        (Pattern::text("age"), Pattern::any_text()),  /* fails: age is
                                                       * number, not text */
        (Pattern::text("active"), Pattern::any_bool()), // matches
    ]);

    let paths = partial_pattern.paths(&test_map);
    assert_eq!(paths.len(), 0);
}

#[test]
fn test_map_key_value_constraints_any_key() {
    // Test constraints with ANY key pattern
    let test_map = cbor(r#"{"key1": "hello", "key2": "world", "key3": 42}"#);

    // Match any key with text value
    let pattern = MapPattern::with_key_value_constraints(vec![(
        Pattern::any(),
        Pattern::any_text(),
    )]);

    let paths = pattern.paths(&test_map);
    let expected = r#"{"key1": "hello", "key2": "world", "key3": 42}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Match any key with number value
    let number_pattern = MapPattern::with_key_value_constraints(vec![(
        Pattern::any(),
        Pattern::any_number(),
    )]);

    let paths = number_pattern.paths(&test_map);
    assert_actual_expected!(format_paths(&paths), expected);
}

#[test]
fn test_map_key_value_constraints_specific_values() {
    // Test constraints with specific values
    let test_map = cbor(r#"{"status": "active", "count": 42, "flag": true}"#);

    // Match specific key-value pairs
    let pattern = MapPattern::with_key_value_constraints(vec![
        (Pattern::text("status"), Pattern::text("active")),
        (Pattern::text("count"), Pattern::number(42.0)),
    ]);

    let paths = pattern.paths(&test_map);
    let expected = r#"{"flag": true, "count": 42, "status": "active"}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Test with non-matching specific values
    let wrong_values = MapPattern::with_key_value_constraints(vec![
        (Pattern::text("status"), Pattern::text("inactive")), // wrong value
        (Pattern::text("count"), Pattern::number(42.0)),      // correct value
    ]);

    let paths = wrong_values.paths(&test_map);
    assert_eq!(paths.len(), 0);
}

#[test]
fn test_map_key_value_constraints_empty_map() {
    // Test constraints against empty map
    let empty_map = cbor("{}");

    // Any constraint should fail on empty map
    let pattern = MapPattern::with_key_value_constraints(vec![(
        Pattern::any(),
        Pattern::any(),
    )]);

    let paths = pattern.paths(&empty_map);
    assert_eq!(paths.len(), 0);

    // Multiple constraints should also fail
    let multi_pattern = MapPattern::with_key_value_constraints(vec![
        (Pattern::text("key1"), Pattern::any()),
        (Pattern::text("key2"), Pattern::any()),
    ]);

    let paths = multi_pattern.paths(&empty_map);
    assert_eq!(paths.len(), 0);
}

#[test]
fn test_map_key_value_constraints_pattern_text_parsing() {
    // Test the unified MAP(pattern:pattern, ...) syntax from text
    let pattern =
        Pattern::parse(r#"MAP(TEXT("name"):TEXT, TEXT("age"):NUMBER)"#)
            .unwrap();

    let matching_map =
        cbor(r#"{"name": "Charlie", "age": 28, "extra": "data"}"#);
    assert!(pattern.matches(&matching_map));

    let non_matching_map = cbor(r#"{"name": 123, "age": 28}"#); // name is number, not text
    assert!(!pattern.matches(&non_matching_map));

    let missing_key_map = cbor(r#"{"name": "Charlie"}"#); // missing age key
    assert!(!pattern.matches(&missing_key_map));

    // Test display format
    assert_eq!(
        pattern.to_string(),
        r#"MAP(TEXT("name"):TEXT, TEXT("age"):NUMBER)"#
    );
}

#[test]
fn test_map_key_value_constraints_complex_patterns() {
    // Test with complex nested patterns
    let pattern =
        Pattern::parse(r#"MAP(ANY:TEXT("target"), NUMBER(42):BOOL(true))"#)
            .unwrap();

    let matching_map =
        cbor(r#"{"somekey": "target", 42: true, "other": "data"}"#);
    assert!(pattern.matches(&matching_map));

    let partial_match = cbor(r#"{"somekey": "target", 42: false}"#); // boolean is wrong
    assert!(!pattern.matches(&partial_match));

    let no_match = cbor(r#"{"somekey": "other", 42: true}"#); // text value is wrong
    assert!(!pattern.matches(&no_match));
}
