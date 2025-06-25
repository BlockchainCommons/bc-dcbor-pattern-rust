use dcbor::prelude::*;
use dcbor_pattern::{
    ArrayPattern, MapPattern, Matcher, Pattern, TaggedPattern,
};
use dcbor_parse::parse_dcbor_item;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> CBOR {
    parse_dcbor_item(s).unwrap()
}

/// Test that ArrayPattern::Any matches any array
#[test]
fn test_array_pattern_any() {
    let pattern = ArrayPattern::any();

    // Should match empty array
    let empty_array = cbor("[]");
    let paths = pattern.paths(&empty_array);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec![empty_array.clone()]);

    // Should match non-empty array
    let array = cbor("[1, 2, 3]");
    let paths = pattern.paths(&array);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec![array.clone()]);

    // Should not match non-array
    let not_array = cbor(r#""not an array""#);
    let paths = pattern.paths(&not_array);
    assert_eq!(paths.len(), 0);
}

/// Test that ArrayPattern::WithLength matches arrays with specific length
#[test]
fn test_array_pattern_with_length() {
    let pattern = ArrayPattern::with_length(2);

    // Should match array with length 2
    let array = cbor("[1, 2]");
    let paths = pattern.paths(&array);
    assert_eq!(paths.len(), 1);

    // Should not match array with different length
    let wrong_length = cbor("[1, 2, 3]");
    let paths = pattern.paths(&wrong_length);
    assert_eq!(paths.len(), 0);

    // Should not match non-array
    let not_array = cbor(r#""not an array""#);
    let paths = pattern.paths(&not_array);
    assert_eq!(paths.len(), 0);
}

/// Test that ArrayPattern::WithElements matches arrays containing matching
/// elements
#[test]
fn test_array_pattern_with_elements() {
    let number_pattern = Pattern::number(42);
    let pattern = ArrayPattern::with_elements(number_pattern);

    // Should match array containing 42
    let array = cbor("[1, 42, 3]");
    let paths = pattern.paths(&array);
    assert_eq!(paths.len(), 1);

    // Should not match array without 42
    let no_match = cbor("[1, 2, 3]");
    let paths = pattern.paths(&no_match);
    assert_eq!(paths.len(), 0);
}

/// Test MapPattern::Any matches any map
#[test]
fn test_map_pattern_any() {
    let pattern = MapPattern::any();

    // Should match empty map
    let empty_map = cbor("{}");
    let paths = pattern.paths(&empty_map);
    assert_eq!(paths.len(), 1);

    // Should match non-empty map
    let cbor_map = cbor(r#"{"key": "value"}"#);
    let paths = pattern.paths(&cbor_map);
    assert_eq!(paths.len(), 1);

    // Should not match non-map
    let not_map = cbor(r#""not a map""#);
    let paths = pattern.paths(&not_map);
    assert_eq!(paths.len(), 0);
}

/// Test MapPattern::WithKey matches maps containing specific keys
#[test]
fn test_map_pattern_with_key() {
    let text_pattern = Pattern::text("target_key");
    let pattern = MapPattern::with_key(text_pattern);

    // Should match map with target key
    let cbor_map = cbor(r#"{"target_key": "value", "other_key": "other_value"}"#);
    let paths = pattern.paths(&cbor_map);
    assert_eq!(paths.len(), 1);

    // Should not match map without target key
    let no_match = cbor(r#"{"wrong_key": "value"}"#);
    let paths = pattern.paths(&no_match);
    assert_eq!(paths.len(), 0);
}

/// Test MapPattern::WithValue matches maps containing specific values
#[test]
fn test_map_pattern_with_value() {
    let text_pattern = Pattern::text("target_value");
    let pattern = MapPattern::with_value(text_pattern);

    // Should match map with target value
    let cbor_map = cbor(r#"{"key": "target_value", "other_key": "other_value"}"#);
    let paths = pattern.paths(&cbor_map);
    assert_eq!(paths.len(), 1);

    // Should not match map without target value
    let no_match = cbor(r#"{"key": "wrong_value"}"#);
    let paths = pattern.paths(&no_match);
    assert_eq!(paths.len(), 0);
}

/// Test TaggedPattern::Any matches any tagged value
#[test]
fn test_tagged_pattern_any() {
    let pattern = TaggedPattern::any();

    // Should match any tagged value
    let tagged = cbor(r#"1234("content")"#);
    let paths = pattern.paths(&tagged);
    assert_eq!(paths.len(), 1);

    // Should not match non-tagged value
    let not_tagged = cbor(r#""not tagged""#);
    let paths = pattern.paths(&not_tagged);
    assert_eq!(paths.len(), 0);
}

/// Test TaggedPattern::WithTag matches tagged values with specific tag
#[test]
fn test_tagged_pattern_with_tag() {
    let target_tag = Tag::new(1234, "test_tag");
    let pattern = TaggedPattern::with_tag(target_tag.clone());

    // Should match tagged value with correct tag
    let tagged = cbor(r#"1234("content")"#);
    let paths = pattern.paths(&tagged);
    assert_eq!(paths.len(), 1);

    // Should not match tagged value with different tag
    let wrong_tagged = cbor(r#"5678("content")"#);
    let paths = pattern.paths(&wrong_tagged);
    assert_eq!(paths.len(), 0);

    // Should not match non-tagged value
    let not_tagged = cbor(r#""not tagged""#);
    let paths = pattern.paths(&not_tagged);
    assert_eq!(paths.len(), 0);
}

/// Test TaggedPattern::WithContent matches tagged values with matching content
#[test]
fn test_tagged_pattern_with_content() {
    let text_pattern = Pattern::text("target_content");
    let pattern = TaggedPattern::with_content(text_pattern);

    // Should match tagged value with matching content
    let tagged = cbor(r#"1234("target_content")"#);
    let paths = pattern.paths(&tagged);
    assert_eq!(paths.len(), 1);

    // Should not match tagged value with different content
    let wrong_content = cbor(r#"1234("wrong_content")"#);
    let paths = pattern.paths(&wrong_content);
    assert_eq!(paths.len(), 0);
}

/// Test structure pattern display formatting
#[test]
fn test_structure_pattern_display() {
    // Array patterns
    assert_eq!(format!("{}", ArrayPattern::any()), "ARRAY");
    assert_eq!(format!("{}", ArrayPattern::with_length(5)), "ARRAY({5})");
    assert_eq!(
        format!("{}", ArrayPattern::with_length_range(1..=10)),
        "ARRAY({1,10})"
    );

    // Map patterns
    assert_eq!(format!("{}", MapPattern::any()), "MAP");
    assert_eq!(format!("{}", MapPattern::with_length(3)), "MAP({3})");
    assert_eq!(
        format!("{}", MapPattern::with_length_range(2..=8)),
        "MAP({2,8})"
    );

    // Tagged patterns
    assert_eq!(format!("{}", TaggedPattern::any()), "TAGGED");
    let tag = Tag::new(1234, "test");
    assert_eq!(
        format!("{}", TaggedPattern::with_tag(tag)),
        "TAGGED_TAG(1234)"
    );
}
