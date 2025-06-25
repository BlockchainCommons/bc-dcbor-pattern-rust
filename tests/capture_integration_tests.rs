// Test integration for named capture functionality

mod common;

use dcbor::prelude::*;
use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern, Result, format_paths};
use indoc::indoc;

/// Test basic capture functionality with simple patterns
#[test]
fn test_capture_basic_number() -> Result<()> {
    let pattern = Pattern::parse("@num(NUMBER(42))")?;
    let cbor = parse_dcbor_item("42").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Should match the root
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec![cbor.clone()]);

    // Validate formatted output
    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        42
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    // Should capture the number
    assert_eq!(captures.len(), 1);
    assert!(captures.contains_key("num"));
    let captured_paths = &captures["num"];
    assert_eq!(captured_paths.len(), 1);
    assert_eq!(captured_paths[0], vec![cbor.clone()]);

    Ok(())
}

/// Test capture with text patterns
#[test]
fn test_capture_basic_text() -> Result<()> {
    let pattern = Pattern::parse(r#"@greeting(TEXT("hello"))"#)?;
    let cbor = parse_dcbor_item(r#""hello""#).unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Should match and capture
    assert_eq!(paths.len(), 1);
    assert_eq!(captures.len(), 1);
    assert!(captures.contains_key("greeting"));
    assert_eq!(captures["greeting"][0], vec![cbor.clone()]);

    // Validate formatted output
    #[rustfmt::skip]
    let expected_paths = indoc! {r#"
        "hello"
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected_paths);

    Ok(())
}

/// Test capture with patterns that don't match
#[test]
fn test_capture_no_match() -> Result<()> {
    let pattern = Pattern::parse("@num(NUMBER(42))")?;
    let cbor = parse_dcbor_item("24").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Should not match
    assert_eq!(paths.len(), 0);
    assert_eq!(captures.len(), 0);

    Ok(())
}

/// Test multiple captures in OR pattern
#[test]
fn test_multiple_captures_or() -> Result<()> {
    let pattern =
        Pattern::parse("@first(NUMBER(42)) | @second(TEXT(\"hello\"))")?;

    // Test matching the first alternative
    let cbor1 = parse_dcbor_item("42").unwrap();
    let (paths1, captures1) = pattern.paths_with_captures(&cbor1);

    assert_eq!(paths1.len(), 1);
    assert_eq!(captures1.len(), 1);
    assert!(captures1.contains_key("first"));
    assert!(!captures1.contains_key("second"));

    // Test matching the second alternative
    let cbor2 = parse_dcbor_item(r#""hello""#).unwrap();
    let (paths2, captures2) = pattern.paths_with_captures(&cbor2);

    assert_eq!(paths2.len(), 1);
    assert_eq!(captures2.len(), 1);
    assert!(captures2.contains_key("second"));
    assert!(!captures2.contains_key("first"));

    Ok(())
}

/// Test nested captures
#[test]
fn test_nested_captures() -> Result<()> {
    let pattern = Pattern::parse("@outer(@inner(NUMBER(42)))")?;
    let cbor = parse_dcbor_item("42").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Should match
    assert_eq!(paths.len(), 1);

    // Should have both captures pointing to the same value
    assert_eq!(captures.len(), 2);
    assert!(captures.contains_key("outer"));
    assert!(captures.contains_key("inner"));

    // Both captures should point to the same path
    assert_eq!(captures["outer"][0], vec![cbor.clone()]);
    assert_eq!(captures["inner"][0], vec![cbor.clone()]);

    Ok(())
}

/// Test captures in array patterns
#[test]
fn test_capture_in_array() -> Result<()> {
    let pattern = Pattern::parse("ARRAY(@item(NUMBER(42)))")?;
    let cbor = parse_dcbor_item("[42]").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Should match the array
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], vec![cbor.clone()]);

    // Should capture the number element
    assert_eq!(captures.len(), 1);
    assert!(captures.contains_key("item"));

    let captured = &captures["item"][0];
    // The captured path should be [array, element]
    assert_eq!(captured.len(), 2);
    assert_eq!(captured[0], cbor);
    assert_eq!(captured[1], CBOR::from(42));

    Ok(())
}

/// Test captures in array sequence patterns
#[test]
fn test_capture_in_array_sequence() -> Result<()> {
    let pattern =
        Pattern::parse("ARRAY(@first(TEXT(\"a\")) > @second(NUMBER(42)))")?;
    let cbor = parse_dcbor_item(r#"["a", 42]"#).unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Should match the array
    assert_eq!(paths.len(), 1);

    // Should have both captures
    assert_eq!(captures.len(), 2);
    assert!(captures.contains_key("first"));
    assert!(captures.contains_key("second"));

    // Check first capture points to "a"
    let first_path = &captures["first"][0];
    assert_eq!(first_path.len(), 2);
    assert_eq!(first_path[1], CBOR::from("a"));

    // Check second capture points to 42
    let second_path = &captures["second"][0];
    assert_eq!(second_path.len(), 2);
    assert_eq!(second_path[1], CBOR::from(42));

    Ok(())
}

/// Test captures in map patterns
#[test]
fn test_capture_in_map() -> Result<()> {
    let pattern =
        Pattern::parse(r#"MAP(@key(TEXT("name")): @value(TEXT("Alice")))"#)?;
    let cbor = parse_dcbor_item(r#"{"name": "Alice"}"#).unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Should match the map
    assert_eq!(paths.len(), 1);

    // Should capture both key and value
    assert_eq!(captures.len(), 2);
    assert!(captures.contains_key("key"));
    assert!(captures.contains_key("value"));

    Ok(())
}

/// Test captures with search patterns
#[test]
fn test_capture_with_search() -> Result<()> {
    let pattern = Pattern::parse("SEARCH(@found(NUMBER(42)))")?;
    let cbor = parse_dcbor_item(r#"[1, [2, 42], 3]"#).unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Search should find the nested 42
    assert!(!paths.is_empty());

    // Should capture it
    assert_eq!(captures.len(), 1);
    assert!(captures.contains_key("found"));

    let found_path = &captures["found"][0];
    // Should be path to the nested 42: [root_array, inner_array, 42]
    assert_eq!(found_path.len(), 3);
    assert_eq!(found_path[2], CBOR::from(42));

    Ok(())
}

/// Test captures with tagged patterns
#[test]
fn test_capture_with_tagged() -> Result<()> {
    let pattern = Pattern::parse("TAG(1, @content(NUMBER(42)))")?;
    let cbor = parse_dcbor_item("1(42)").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Should match the tagged value
    assert_eq!(paths.len(), 1);

    // Should capture the content
    assert_eq!(captures.len(), 1);
    assert!(captures.contains_key("content"));

    let content_path = &captures["content"][0];
    // Should be path to the tagged content: [tagged_value, content]
    assert_eq!(content_path.len(), 2);
    assert_eq!(content_path[1], CBOR::from(42));

    Ok(())
}

/// Test capture performance doesn't significantly degrade
#[test]
fn test_capture_performance() -> Result<()> {
    // Create a complex nested structure
    let cbor = parse_dcbor_item(
        r#"[{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]"#,
    )
    .unwrap();

    // Pattern that will search through the structure
    let pattern = Pattern::parse("SEARCH(@nums(NUMBER))")?;

    let start = std::time::Instant::now();
    let (paths, captures) = pattern.paths_with_captures(&cbor);
    let duration = start.elapsed();

    // Should find all the numbers
    assert!(!paths.is_empty());
    assert!(captures.contains_key("nums"));
    assert_eq!(captures["nums"].len(), 9); // Should capture all 9 numbers

    // Should complete reasonably quickly (less than 10ms for this small
    // example)
    assert!(
        duration.as_millis() < 10,
        "Capture processing took too long: {:?}",
        duration
    );

    Ok(())
}

/// Test patterns without captures use the optimized path
#[test]
fn test_no_captures_optimization() -> Result<()> {
    let pattern = Pattern::parse("NUMBER(42)")?;
    let cbor = parse_dcbor_item("42").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Should match
    assert_eq!(paths.len(), 1);

    // Should have no captures
    assert_eq!(captures.len(), 0);

    Ok(())
}

/// Test error handling with invalid capture patterns
#[test]
fn test_capture_parsing_errors() {
    // Missing closing parenthesis
    assert!(Pattern::parse("@name(NUMBER(42)").is_err());

    // Missing pattern inside capture
    assert!(Pattern::parse("@name()").is_err());

    // Invalid capture name (empty)
    assert!(Pattern::parse("@(NUMBER(42))").is_err());
}

/// Test complex nested captures with multiple levels
#[test]
fn test_complex_nested_captures() -> Result<()> {
    let pattern = Pattern::parse(
        r#"
        ARRAY(
            @first_map(MAP(
                @key1(TEXT("type")): @val1(TEXT("person"))
            )) >
            @second_map(MAP(
                @key2(TEXT("name")): @val2(TEXT)
            ))
        )
    "#,
    )?;

    let cbor =
        parse_dcbor_item(r#"[{"type": "person"}, {"name": "Alice"}]"#).unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Should match
    assert_eq!(paths.len(), 1);

    // Should have all captures
    assert_eq!(captures.len(), 6);
    assert!(captures.contains_key("first_map"));
    assert!(captures.contains_key("key1"));
    assert!(captures.contains_key("val1"));
    assert!(captures.contains_key("second_map"));
    assert!(captures.contains_key("key2"));
    assert!(captures.contains_key("val2"));

    // Verify val2 captured "Alice"
    let val2_path = &captures["val2"][0];
    assert_eq!(val2_path[val2_path.len() - 1], CBOR::from("Alice"));

    Ok(())
}
