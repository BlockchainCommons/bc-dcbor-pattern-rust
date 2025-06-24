use dcbor::prelude::*;
use dcbor_pattern::{
    ArrayPattern, MapPattern, Matcher, Pattern, TaggedPattern,
};

/// Test that ArrayPattern::Any matches any array
#[test]
fn test_array_pattern_any() {
    let pattern = ArrayPattern::any();

    // Should match empty array
    let empty_array: CBOR = Vec::<CBOR>::new().into();
    let paths = pattern.paths(&empty_array);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec![empty_array.clone()]);

    // Should match non-empty array
    let array: CBOR = vec![1.to_cbor(), 2.to_cbor(), 3.to_cbor()].into();
    let paths = pattern.paths(&array);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec![array.clone()]);

    // Should not match non-array
    let not_array: CBOR = "not an array".into();
    let paths = pattern.paths(&not_array);
    assert_eq!(paths.len(), 0);
}

/// Test that ArrayPattern::WithLength matches arrays with specific length
#[test]
fn test_array_pattern_with_length() {
    let pattern = ArrayPattern::with_length(2);

    // Should match array with length 2
    let array: CBOR = vec![1.to_cbor(), 2.to_cbor()].into();
    let paths = pattern.paths(&array);
    assert_eq!(paths.len(), 1);

    // Should not match array with different length
    let wrong_length: CBOR = vec![1.to_cbor(), 2.to_cbor(), 3.to_cbor()].into();
    let paths = pattern.paths(&wrong_length);
    assert_eq!(paths.len(), 0);

    // Should not match non-array
    let not_array: CBOR = "not an array".into();
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
    let array: CBOR = vec![1.to_cbor(), 42.to_cbor(), 3.to_cbor()].into();
    let paths = pattern.paths(&array);
    assert_eq!(paths.len(), 1);

    // Should not match array without 42
    let no_match: CBOR = vec![1.to_cbor(), 2.to_cbor(), 3.to_cbor()].into();
    let paths = pattern.paths(&no_match);
    assert_eq!(paths.len(), 0);
}

/// Test MapPattern::Any matches any map
#[test]
fn test_map_pattern_any() {
    let pattern = MapPattern::any();

    // Should match empty map
    let empty_map: CBOR = Map::new().into();
    let paths = pattern.paths(&empty_map);
    assert_eq!(paths.len(), 1);

    // Should match non-empty map
    let mut map = Map::new();
    map.insert("key", "value");
    let cbor_map: CBOR = map.into();
    let paths = pattern.paths(&cbor_map);
    assert_eq!(paths.len(), 1);

    // Should not match non-map
    let not_map: CBOR = "not a map".into();
    let paths = pattern.paths(&not_map);
    assert_eq!(paths.len(), 0);
}

/// Test MapPattern::WithKey matches maps containing specific keys
#[test]
fn test_map_pattern_with_key() {
    let text_pattern = Pattern::text("target_key");
    let pattern = MapPattern::with_key(text_pattern);

    // Should match map with target key
    let mut map = Map::new();
    map.insert("target_key", "value");
    map.insert("other_key", "other_value");
    let cbor_map: CBOR = map.into();
    let paths = pattern.paths(&cbor_map);
    assert_eq!(paths.len(), 1);

    // Should not match map without target key
    let mut no_match_map = Map::new();
    no_match_map.insert("wrong_key", "value");
    let no_match: CBOR = no_match_map.into();
    let paths = pattern.paths(&no_match);
    assert_eq!(paths.len(), 0);
}

/// Test MapPattern::WithValue matches maps containing specific values
#[test]
fn test_map_pattern_with_value() {
    let text_pattern = Pattern::text("target_value");
    let pattern = MapPattern::with_value(text_pattern);

    // Should match map with target value
    let mut map = Map::new();
    map.insert("key", "target_value");
    map.insert("other_key", "other_value");
    let cbor_map: CBOR = map.into();
    let paths = pattern.paths(&cbor_map);
    assert_eq!(paths.len(), 1);

    // Should not match map without target value
    let mut no_match_map = Map::new();
    no_match_map.insert("key", "wrong_value");
    let no_match: CBOR = no_match_map.into();
    let paths = pattern.paths(&no_match);
    assert_eq!(paths.len(), 0);
}

/// Test TaggedPattern::Any matches any tagged value
#[test]
fn test_tagged_pattern_any() {
    let pattern = TaggedPattern::any();

    // Should match any tagged value
    let tag = Tag::new(1234, "test_tag");
    let tagged: CBOR = CBORCase::Tagged(tag, "content".into()).into();
    let paths = pattern.paths(&tagged);
    assert_eq!(paths.len(), 1);

    // Should not match non-tagged value
    let not_tagged: CBOR = "not tagged".into();
    let paths = pattern.paths(&not_tagged);
    assert_eq!(paths.len(), 0);
}

/// Test TaggedPattern::WithTag matches tagged values with specific tag
#[test]
fn test_tagged_pattern_with_tag() {
    let target_tag = Tag::new(1234, "test_tag");
    let pattern = TaggedPattern::with_tag(target_tag.clone());

    // Should match tagged value with correct tag
    let tagged: CBOR = CBORCase::Tagged(target_tag, "content".into()).into();
    let paths = pattern.paths(&tagged);
    assert_eq!(paths.len(), 1);

    // Should not match tagged value with different tag
    let wrong_tag = Tag::new(5678, "wrong_tag");
    let wrong_tagged: CBOR =
        CBORCase::Tagged(wrong_tag, "content".into()).into();
    let paths = pattern.paths(&wrong_tagged);
    assert_eq!(paths.len(), 0);

    // Should not match non-tagged value
    let not_tagged: CBOR = "not tagged".into();
    let paths = pattern.paths(&not_tagged);
    assert_eq!(paths.len(), 0);
}

/// Test TaggedPattern::WithContent matches tagged values with matching content
#[test]
fn test_tagged_pattern_with_content() {
    let text_pattern = Pattern::text("target_content");
    let pattern = TaggedPattern::with_content(text_pattern);

    // Should match tagged value with matching content
    let tag = Tag::new(1234, "test_tag");
    let tagged: CBOR = CBORCase::Tagged(tag, "target_content".into()).into();
    let paths = pattern.paths(&tagged);
    assert_eq!(paths.len(), 1);

    // Should not match tagged value with different content
    let tag2 = Tag::new(1234, "test_tag");
    let wrong_content: CBOR =
        CBORCase::Tagged(tag2, "wrong_content".into()).into();
    let paths = pattern.paths(&wrong_content);
    assert_eq!(paths.len(), 0);
}

/// Test structure pattern display formatting
#[test]
fn test_structure_pattern_display() {
    // Array patterns
    assert_eq!(format!("{}", ArrayPattern::any()), "ARRAY");
    assert_eq!(format!("{}", ArrayPattern::with_length(5)), "ARRAY_LEN(5)");
    assert_eq!(
        format!("{}", ArrayPattern::with_length_range(1..=10)),
        "ARRAY_LEN_RANGE(1..=10)"
    );

    // Map patterns
    assert_eq!(format!("{}", MapPattern::any()), "MAP");
    assert_eq!(format!("{}", MapPattern::with_length(3)), "MAP_LEN(3)");
    assert_eq!(
        format!("{}", MapPattern::with_length_range(2..=8)),
        "MAP_LEN_RANGE(2..=8)"
    );

    // Tagged patterns
    assert_eq!(format!("{}", TaggedPattern::any()), "TAGGED");
    let tag = Tag::new(1234, "test");
    assert_eq!(
        format!("{}", TaggedPattern::with_tag(tag)),
        "TAGGED_TAG(1234)"
    );
}
