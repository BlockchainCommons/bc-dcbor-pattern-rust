use dcbor::prelude::*;
use dcbor_pattern::{Pattern, MapPattern, Matcher};
use dcbor_parse::parse_dcbor_item;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> CBOR {
    parse_dcbor_item(s).unwrap()
}

#[test]
fn test_map_patterns_with_real_cbor() {
    // Create test maps using CBOR diagnostic notation
    let empty_map = cbor("{}");
    let single_item = cbor(r#"{"key": "hello"}"#);
    let three_items = cbor(r#"{"a": 1, "b": 2, "c": 3}"#);
    let large_map = cbor(r#"{0: "item0", 1: "item1", 2: "item2", 3: "item3", 4: "item4", 5: "item5", 6: "item6", 7: "item7", 8: "item8", 9: "item9"}"#);

    // Test MAP (any map)
    let any_map = Pattern::parse("MAP").unwrap();
    assert!(any_map.matches(&empty_map));
    assert!(any_map.matches(&single_item));
    assert!(any_map.matches(&three_items));
    assert!(any_map.matches(&large_map));
    assert!(!any_map.matches(&1.to_cbor())); // not a map

    // Test MAP({0}) - empty map
    let empty_pattern = Pattern::parse("MAP({0})").unwrap();
    assert!(empty_pattern.matches(&empty_map));
    assert!(!empty_pattern.matches(&single_item));
    assert!(!empty_pattern.matches(&three_items));

    // Test MAP({1}) - single item map
    let single_pattern = Pattern::parse("MAP({1})").unwrap();
    assert!(!single_pattern.matches(&empty_map));
    assert!(single_pattern.matches(&single_item));
    assert!(!single_pattern.matches(&three_items));

    // Test MAP({3}) - three item map
    let three_pattern = Pattern::parse("MAP({3})").unwrap();
    assert!(!three_pattern.matches(&empty_map));
    assert!(!three_pattern.matches(&single_item));
    assert!(three_pattern.matches(&three_items));
    assert!(!three_pattern.matches(&large_map));

    // Test MAP({5,15}) - range pattern
    let range_pattern = Pattern::parse("MAP({5,15})").unwrap();
    assert!(!range_pattern.matches(&empty_map));
    assert!(!range_pattern.matches(&single_item));
    assert!(!range_pattern.matches(&three_items));
    assert!(range_pattern.matches(&large_map)); // 10 items

    // Test MAP({5,}) - at least 5 items
    let min_pattern = Pattern::parse("MAP({5,})").unwrap();
    assert!(!min_pattern.matches(&empty_map));
    assert!(!min_pattern.matches(&single_item));
    assert!(!min_pattern.matches(&three_items));
    assert!(min_pattern.matches(&large_map)); // 10 items
}

#[test]
fn test_map_pattern_display() {
    assert_eq!(MapPattern::any().to_string(), "MAP");
    assert_eq!(MapPattern::with_length(0).to_string(), "MAP({0})");
    assert_eq!(MapPattern::with_length(5).to_string(), "MAP({5})");
    assert_eq!(MapPattern::with_length_range(2..=8).to_string(), "MAP({2,8})");
    assert_eq!(MapPattern::with_length_range(3..=usize::MAX).to_string(), "MAP({3,})");
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
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec![test_map.clone()]);

    // Test with non-map data
    let not_map = cbor(r#""not a map""#);
    let paths = any_map.paths(&not_map);
    assert_eq!(paths.len(), 0);

    // Test exact length match
    let exact_pattern = MapPattern::with_length(2);
    let paths = exact_pattern.paths(&test_map);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec![test_map.clone()]);

    // Test length mismatch
    let wrong_length = MapPattern::with_length(3);
    let paths = wrong_length.paths(&test_map);
    assert_eq!(paths.len(), 0);
}
