mod common;

use dcbor::prelude::*;
use dcbor_pattern::{Matcher, Pattern, format_paths};
use indoc::indoc;

#[test]
fn test_sequence_pattern_new_api() {
    // Test the new Pattern::sequence() convenience method
    let sequence = Pattern::sequence(vec![
        Pattern::text("first"),
        Pattern::text("second"),
    ]);

    // Verify display format shows sequence syntax
    let display = sequence.to_string();
    assert!(display.contains("first"));
    assert!(display.contains("second"));
    assert!(display.contains(", "));  // Changed from > to comma

    // Verify sequence is marked as complex
    assert!(sequence.is_complex());
}

#[test]
fn test_structure_convenience_methods() {
    // Test new structure pattern convenience methods
    let array_pattern = Pattern::any_array();
    let map_pattern = Pattern::any_map();
    let tagged_pattern = Pattern::any_tagged();

    assert_eq!(array_pattern.to_string(), "[*]");
    assert_eq!(map_pattern.to_string(), "{*}");
    assert_eq!(tagged_pattern.to_string(), "TAGGED");

    // Test that they work with real CBOR data
    let array_cbor = vec![1, 2, 3].to_cbor();
    let map_cbor = vec![("key", "value")]
        .into_iter()
        .collect::<std::collections::BTreeMap<_, _>>()
        .to_cbor();
    let tagged_cbor = CBOR::to_tagged_value(42, "content".to_cbor());

    // Test array pattern paths
    let array_paths = array_pattern.paths(&array_cbor);
    assert!(!array_paths.is_empty());
    #[rustfmt::skip]
    let expected_array = indoc! {r#"
        [1, 2, 3]
    "#}.trim();
    assert_actual_expected!(format_paths(&array_paths), expected_array);

    // Test map pattern paths
    let map_paths = map_pattern.paths(&map_cbor);
    assert!(!map_paths.is_empty());
    #[rustfmt::skip]
    let expected_map = indoc! {r#"
        {"key": "value"}
    "#}.trim();
    assert_actual_expected!(format_paths(&map_paths), expected_map);

    // Test tagged pattern paths
    let tagged_paths = tagged_pattern.paths(&tagged_cbor);
    assert!(!tagged_paths.is_empty());
    #[rustfmt::skip]
    let expected_tagged = indoc! {r#"
        42("content")
    "#}.trim();
    assert_actual_expected!(format_paths(&tagged_paths), expected_tagged);
}

#[test]
fn test_sequence_pattern_compilation() {
    let sequence = Pattern::sequence(vec![
        Pattern::text("a"),
        Pattern::number(42),
        Pattern::any_bool(),
    ]);

    let mut code = Vec::new();
    let mut literals = Vec::new();
    let mut captures = Vec::new();

    sequence.compile(&mut code, &mut literals, &mut captures);

    // Should generate VM instructions
    assert!(!code.is_empty());
    // Should have literals for each pattern
    assert_eq!(literals.len(), 3);
    // No captures in this test
    assert!(captures.is_empty());
}

#[test]
fn test_sequence_pattern_with_captures() {
    let sequence = Pattern::sequence(vec![
        Pattern::capture("first", Pattern::text("hello")),
        Pattern::capture("second", Pattern::number(42)),
    ]);

    let mut capture_names = Vec::new();
    sequence.collect_capture_names(&mut capture_names);

    assert_eq!(capture_names.len(), 2);
    assert!(capture_names.contains(&"first".to_string()));
    assert!(capture_names.contains(&"second".to_string()));
}

#[test]
fn test_empty_sequence_pattern() {
    let empty_sequence = Pattern::sequence(vec![]);

    // Empty sequence should display as "()"
    assert_eq!(empty_sequence.to_string(), "()");

    // Empty sequence should not be complex
    assert!(!empty_sequence.is_complex());

    // Empty sequence should compile without errors
    let mut code = Vec::new();
    let mut literals = Vec::new();
    let mut captures = Vec::new();

    empty_sequence.compile(&mut code, &mut literals, &mut captures);

    // Empty sequence should not generate any VM instructions
    assert!(code.is_empty());
}
