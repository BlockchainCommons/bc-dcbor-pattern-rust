mod common;

use dcbor::prelude::*;
use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{
    format_paths, ArrayPattern, MapPattern, Matcher, Pattern, TaggedPattern
};
use indoc::indoc;

/// Helper function to parse CBOR diagnostic notation into CBOR objects
fn cbor(s: &str) -> CBOR { parse_dcbor_item(s).unwrap() }

/// Helper function to parse pattern text into Pattern objects
fn parse(s: &str) -> Pattern { Pattern::parse(s).unwrap() }

/// Test that ArrayPattern::Any matches any array
#[test]
fn test_array_pattern_any() {
    let pattern = parse("[*]");

    // Should match empty array
    let empty_array = cbor("[]");
    let paths = pattern.paths(&empty_array);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        []
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should match non-empty array
    let array = cbor("[1, 2, 3]");
    let paths = pattern.paths(&array);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        [1, 2, 3]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match non-array
    let not_array = cbor(r#""not an array""#);
    assert!(!pattern.matches(&not_array));
}

/// Test that ArrayPattern::WithLength matches arrays with specific length
#[test]
fn test_array_pattern_with_length() {
    let pattern = parse("[{2}]");

    // Should match array with length 2
    let array = cbor("[1, 2]");
    let paths = pattern.paths(&array);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        [1, 2]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match array with different length
    let wrong_length = cbor("[1, 2, 3]");
    assert!(!pattern.matches(&wrong_length));

    // Should not match non-array
    let not_array = cbor(r#""not an array""#);
    assert!(!pattern.matches(&not_array));
}

/// Test that ArrayPattern::WithElements matches arrays with exactly the
/// specified pattern This implements the unified syntax: [pattern] matches
/// the array as a sequence
#[test]
fn test_array_pattern_with_elements() {
    let number_pattern = parse("42");
    let pattern = ArrayPattern::with_elements(number_pattern);

    // Should match array with exactly one element: 42
    let single_element = cbor("[42]");
    let paths = pattern.paths(&single_element);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        [42]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should NOT match array containing 42 among other elements (unified
    // syntax)
    let multi_element = cbor("[1, 42, 3]");
    assert!(
        !pattern.matches(&multi_element),
        "[42] should only match [42], not [1, 42, 3]"
    );

    // Should not match array without 42
    let no_match = cbor("[1, 2, 3]");
    assert!(!pattern.matches(&no_match));

    // Should not match empty array
    let empty = cbor("[]");
    assert!(!pattern.matches(&empty));
}

/// Test MapPattern::Any matches any map
#[test]
fn test_map_pattern_any() {
    let pattern = parse("{*}");

    // Should match empty map
    let empty_map = cbor("{}");
    let paths = pattern.paths(&empty_map);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        {}
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should match non-empty map
    let cbor_map = cbor(r#"{"key": "value"}"#);
    let paths = pattern.paths(&cbor_map);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        {"key": "value"}
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match non-map
    let not_map = cbor(r#""not a map""#);
    assert!(!pattern.matches(&not_map));
}

/// Test TaggedPattern::Any matches any tagged value
#[test]
fn test_tagged_pattern_any() {
    let pattern = TaggedPattern::any();

    // Should match any tagged value
    let tagged = cbor(r#"1234("content")"#);
    let paths = pattern.paths(&tagged);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1234("content")
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match non-tagged value
    let not_tagged = cbor(r#""not tagged""#);
    assert!(!pattern.matches(&not_tagged));
}

/// Test TaggedPattern::WithTag matches tagged values with specific tag
#[test]
fn test_tagged_pattern_with_tag_and_any() {
    let target_tag = Tag::new(1234, "test_tag");
    let pattern = TaggedPattern::with_tag(target_tag.clone(), Pattern::any());

    // Should match tagged value with correct tag
    let tagged = cbor(r#"1234("content")"#);
    let paths = pattern.paths(&tagged);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        1234("content")
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match tagged value with different tag
    let wrong_tagged = cbor(r#"5678("content")"#);
    assert!(!pattern.matches(&wrong_tagged));

    // Should not match non-tagged value
    let not_tagged = cbor(r#""not tagged""#);
    assert!(!pattern.matches(&not_tagged));
}

/// Test structure pattern display formatting
#[test]
fn test_structure_pattern_display() {
    // Array patterns
    assert_eq!(parse("[*]").to_string(), "[*]");
    assert_eq!(parse("[{5}]").to_string(), "[{5}]");
    assert_eq!(
        format!("{}", ArrayPattern::with_length_range(1..=10)),
        "[{1,10}]"
    );

    // Map patterns
    assert_eq!(parse("{*}").to_string(), "{*}");
    assert_eq!(parse("{{3}}").to_string(), "{{3}}");
    assert_eq!(
        format!("{}", MapPattern::with_length_range(2..=8)),
        "{{2,8}}"
    );

    // Tagged patterns
    assert_eq!(format!("{}", TaggedPattern::any()), "tagged");
    let tag = Tag::new(1234, "test");
    assert_eq!(
        format!("{}", TaggedPattern::with_tag(tag, Pattern::any())),
        "tagged(1234, *)"
    );
}
