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
    let expected = r#"{
    0:
    "item0",
    1:
    "item1",
    2:
    "item2",
    3:
    "item3",
    4:
    "item4",
    5:
    "item5",
    6:
    "item6",
    7:
    "item7",
    8:
    "item8",
    9:
    "item9"
}"#;
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
    let expected = r#"{
    0:
    "item0",
    1:
    "item1",
    2:
    "item2",
    3:
    "item3",
    4:
    "item4",
    5:
    "item5",
    6:
    "item6",
    7:
    "item7",
    8:
    "item8",
    9:
    "item9"
}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match smaller maps
    assert!(!range_pattern.matches(&empty_map));
    assert!(!range_pattern.matches(&single_item));
    assert!(!range_pattern.matches(&three_items));

    // Test MAP({5,}) - at least 5 items
    let min_pattern = Pattern::parse("MAP({5,})").unwrap();
    let paths = min_pattern.paths(&large_map);
    let expected = r#"{
    0:
    "item0",
    1:
    "item1",
    2:
    "item2",
    3:
    "item3",
    4:
    "item4",
    5:
    "item5",
    6:
    "item6",
    7:
    "item7",
    8:
    "item8",
    9:
    "item9"
}"#;
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
    let expected = r#"{
    "key1":
    "value1",
    "key2":
    42
}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Test with non-map data - should return no paths
    let not_map = cbor(r#""not a map""#);
    let paths = any_map.paths(&not_map);
    assert_eq!(paths.len(), 0);

    // Test exact length match
    let exact_pattern = MapPattern::with_length(2);
    let paths = exact_pattern.paths(&test_map);
    let expected = r#"{
    "key1":
    "value1",
    "key2":
    42
}"#;
    assert_actual_expected!(format_paths(&paths), expected);

    // Test length mismatch - should return no paths
    let wrong_length = MapPattern::with_length(3);
    let paths = wrong_length.paths(&test_map);
    assert_eq!(paths.len(), 0);
}
