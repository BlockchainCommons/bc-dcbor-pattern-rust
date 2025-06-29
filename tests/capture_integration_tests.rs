// Test integration for named capture functionality

mod common;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern, Result, format_paths_with_captures};
use indoc::indoc;

/// Test basic capture functionality with simple patterns
#[test]
fn test_capture_basic_number() -> Result<()> {
    let pattern = Pattern::parse("@num(42)")?;
    let cbor = parse_dcbor_item("42").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Validate formatted output with captures
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        @num
            42
        42
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );

    Ok(())
}

/// Test capture with text patterns
#[test]
fn test_capture_basic_text() -> Result<()> {
    let pattern = Pattern::parse(r#"@greeting("hello")"#)?;
    let cbor = parse_dcbor_item(r#""hello""#).unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Validate formatted output with captures
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        @greeting
            "hello"
        "hello"
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );

    Ok(())
}

/// Test capture with patterns that don't match
#[test]
fn test_capture_no_match() -> Result<()> {
    let pattern = Pattern::parse("@num(42)")?;
    let cbor = parse_dcbor_item("24").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Should not match - should be empty output
    let expected_output = "";
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );

    Ok(())
}

/// Test multiple captures in OR pattern
#[test]
fn test_multiple_captures_or() -> Result<()> {
    let pattern =
        Pattern::parse("@first(42) | @second(\"hello\")")?;

    // Test matching the first alternative
    let cbor1 = parse_dcbor_item("42").unwrap();
    let (paths1, captures1) = pattern.paths_with_captures(&cbor1);

    #[rustfmt::skip]
    let expected_output1 = indoc! {r#"
        @first
            42
        42
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths1,
            &captures1,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output1
    );

    // Test matching the second alternative
    let cbor2 = parse_dcbor_item(r#""hello""#).unwrap();
    let (paths2, captures2) = pattern.paths_with_captures(&cbor2);

    #[rustfmt::skip]
    let expected_output2 = indoc! {r#"
        @second
            "hello"
        "hello"
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths2,
            &captures2,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output2
    );

    Ok(())
}

/// Test nested captures
#[test]
fn test_nested_captures() -> Result<()> {
    let pattern = Pattern::parse("@outer(@inner(42))")?;
    let cbor = parse_dcbor_item("42").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Should have both captures pointing to the same value, sorted
    // alphabetically
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        @inner
            42
        @outer
            42
        42
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );

    Ok(())
}

/// Test captures in array patterns
#[test]
fn test_capture_in_array() -> Result<()> {
    let pattern = Pattern::parse("[@item(42)]")?;
    let cbor = parse_dcbor_item("[42]").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Validate the structured output
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        @item
            [42]
                42
        [42]
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );

    Ok(())
}

/// Test captures in array sequence patterns
#[test]
fn test_capture_in_array_sequence() -> Result<()> {
    let pattern =
        Pattern::parse(r#"[@first("a"), @second(42)]"#)?;
    let cbor = parse_dcbor_item(r#"["a", 42]"#).unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Should capture both elements, sorted alphabetically
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        @first
            ["a", 42]
                "a"
        @second
            ["a", 42]
                42
        ["a", 42]
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );

    Ok(())
}

/// Test captures in map patterns
#[test]
fn test_capture_in_map() -> Result<()> {
    let pattern =
        Pattern::parse(r#"{@key("name"): @value("Alice")}"#)?;
    let cbor = parse_dcbor_item(r#"{"name": "Alice"}"#).unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Validate formatted output with captures
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
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
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );

    Ok(())
}

/// Test captures with search patterns
#[test]
fn test_capture_with_search() -> Result<()> {
    let pattern = Pattern::parse("SEARCH(@found(42))")?;
    let cbor = parse_dcbor_item(r#"[1, [2, 42], 3]"#).unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Validate formatted output with captures
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        @found
            [1, [2, 42], 3]
                [2, 42]
                    42
        [1, [2, 42], 3]
            [2, 42]
                42
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );

    Ok(())
}

/// Test captures with tagged patterns
#[test]
fn test_capture_with_tagged() -> Result<()> {
    let pattern = Pattern::parse("TAG(1, @content(42))")?;
    let cbor = parse_dcbor_item("1(42)").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Validate formatted output with captures
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        @content
            1(42)
                42
        1(42)
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );

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
    let pattern = Pattern::parse("SEARCH(@nums(number))")?;

    let start = std::time::Instant::now();
    let (paths, captures) = pattern.paths_with_captures(&cbor);
    let duration = start.elapsed();

    // Validate formatted output with all captured numbers
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        @nums
            [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
                {"a": [1, 2, 3]}
                    [1, 2, 3]
                        1
            [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
                {"a": [1, 2, 3]}
                    [1, 2, 3]
                        2
            [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
                {"a": [1, 2, 3]}
                    [1, 2, 3]
                        3
            [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
                {"b": [4, 5, 6]}
                    [4, 5, 6]
                        4
            [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
                {"b": [4, 5, 6]}
                    [4, 5, 6]
                        5
            [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
                {"b": [4, 5, 6]}
                    [4, 5, 6]
                        6
            [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
                {"c": [7, 8, 9]}
                    [7, 8, 9]
                        7
            [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
                {"c": [7, 8, 9]}
                    [7, 8, 9]
                        8
            [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
                {"c": [7, 8, 9]}
                    [7, 8, 9]
                        9
        [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
            {"a": [1, 2, 3]}
                [1, 2, 3]
                    1
        [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
            {"a": [1, 2, 3]}
                [1, 2, 3]
                    2
        [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
            {"a": [1, 2, 3]}
                [1, 2, 3]
                    3
        [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
            {"b": [4, 5, 6]}
                [4, 5, 6]
                    4
        [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
            {"b": [4, 5, 6]}
                [4, 5, 6]
                    5
        [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
            {"b": [4, 5, 6]}
                [4, 5, 6]
                    6
        [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
            {"c": [7, 8, 9]}
                [7, 8, 9]
                    7
        [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
            {"c": [7, 8, 9]}
                [7, 8, 9]
                    8
        [{"a": [1, 2, 3]}, {"b": [4, 5, 6]}, {"c": [7, 8, 9]}]
            {"c": [7, 8, 9]}
                [7, 8, 9]
                    9
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );

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
    let pattern = Pattern::parse("42")?;
    let cbor = parse_dcbor_item("42").unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Validate formatted output with no captures
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        42
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );

    Ok(())
}

/// Test error handling with invalid capture patterns
#[test]
fn test_capture_parsing_errors() {
    // Missing closing parenthesis
    assert!(Pattern::parse("@name(42").is_err());

    // Missing pattern inside capture
    assert!(Pattern::parse("@name()").is_err());

    // Invalid capture name (empty)
    assert!(Pattern::parse("@(42)").is_err());
}

/// Test complex nested captures with multiple levels
#[test]
fn test_complex_nested_captures() -> Result<()> {
    #[rustfmt::skip]
    let pattern = Pattern::parse(r#"
        [
            @first_map({
                @key1("type"): @val1("person")
            }),
            @second_map({
                @key2("name"): @val2(text)
            })
        ]
    "#)?;

    let cbor =
        parse_dcbor_item(r#"[{"type": "person"}, {"name": "Alice"}]"#).unwrap();

    let (paths, captures) = pattern.paths_with_captures(&cbor);

    // Validate formatted output with all captures
    #[rustfmt::skip]
    let expected_output = indoc! {r#"
        @first_map
            [{"type": "person"}, {"name": "Alice"}]
                {"type": "person"}
        @key1
            [{"type": "person"}, {"name": "Alice"}]
                {"type": "person"}
                    "type"
        @key2
            [{"type": "person"}, {"name": "Alice"}]
                {"name": "Alice"}
                    "name"
        @second_map
            [{"type": "person"}, {"name": "Alice"}]
                {"name": "Alice"}
        @val1
            [{"type": "person"}, {"name": "Alice"}]
                {"type": "person"}
                    "person"
        @val2
            [{"type": "person"}, {"name": "Alice"}]
                {"name": "Alice"}
                    "Alice"
        [{"type": "person"}, {"name": "Alice"}]
    "#}.trim();
    assert_actual_expected!(
        format_paths_with_captures(
            &paths,
            &captures,
            dcbor_pattern::FormatPathsOpts::default()
        ),
        expected_output
    );

    Ok(())
}
