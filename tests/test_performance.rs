mod common;

use std::time::Instant;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern, format_paths};
use indoc::indoc;

#[test]
fn test_deeply_nested_performance() {
    // Test performance with very deeply nested structures
    let start = Instant::now();

    // Create a deeply nested pattern: 5 levels deep
    let pattern = Pattern::parse(r#"TAG(100, MAP(TEXT("a"):MAP(TEXT("b"):MAP(TEXT("c"):MAP(TEXT("d"):[NUMBER(42)])))))"#).unwrap();
    let pattern_creation_time = start.elapsed();

    // Create matching deeply nested data
    let data =
        parse_dcbor_item(r#"100({"a": {"b": {"c": {"d": [42]}}}})"#).unwrap();

    // Test matching performance
    let match_start = Instant::now();
    let result = pattern.matches(&data);
    let match_time = match_start.elapsed();

    assert!(result, "Should match deeply nested structure");

    // Test paths generation and validate result
    let paths_start = Instant::now();
    let paths = pattern.paths(&data);
    let paths_time = paths_start.elapsed();

    #[rustfmt::skip]
    let expected = indoc! {r#"
        100({"a": {"b": {"c": {"d": [42]}}}})
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Performance should be reasonable (under 10ms for this level of nesting)
    assert!(
        pattern_creation_time.as_millis() < 10,
        "Pattern creation should be fast"
    );
    assert!(
        match_time.as_millis() < 10,
        "Pattern matching should be fast"
    );
    assert!(
        paths_time.as_millis() < 10,
        "Path generation should be fast"
    );

    println!(
        "Deep nesting performance - Pattern creation: {:?}, Matching: {:?}, Paths: {:?}",
        pattern_creation_time, match_time, paths_time
    );
}

#[test]
fn test_complex_repeat_pattern_performance() {
    let start = Instant::now();

    // Complex pattern with multiple repeat patterns
    let pattern = Pattern::parse(
        r#"[(MAP(TEXT("id"):NUMBER])*, (ANY)*, (MAP(TEXT("name"):TEXT))*)"#,
    )
    .unwrap();
    let pattern_creation_time = start.elapsed();

    // Create test data with many elements to test backtracking performance
    let data = parse_dcbor_item(r#"[{"id": 1}, {"id": 2}, 42, "test", true, {"name": "Alice"}, {"name": "Bob"}]"#).unwrap();

    let match_start = Instant::now();
    let result = pattern.matches(&data);
    let match_time = match_start.elapsed();

    assert!(result, "Should match complex pattern with multiple repeats");

    // Test paths generation and validate result
    let paths_start = Instant::now();
    let paths = pattern.paths(&data);
    let paths_time = paths_start.elapsed();

    #[rustfmt::skip]
    let expected = indoc! {r#"
        [{"id": 1}, {"id": 2}, 42, "test", true, {"name": "Alice"}, {"name": "Bob"}]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Performance should be reasonable even with backtracking
    assert!(
        pattern_creation_time.as_millis() < 10,
        "Complex pattern creation should be fast"
    );
    assert!(
        match_time.as_millis() < 10,
        "Complex pattern matching should be fast"
    );
    assert!(
        paths_time.as_millis() < 10,
        "Complex path generation should be fast"
    );

    println!(
        "Complex repeat performance - Pattern creation: {:?}, Matching: {:?}, Paths: {:?}",
        pattern_creation_time, match_time, paths_time
    );
}

#[test]
fn test_large_array_with_search_performance() {
    let start = Instant::now();

    // Search pattern that needs to traverse a large structure
    let pattern = Pattern::parse(r#"SEARCH(TEXT("needle"))"#).unwrap();
    let pattern_creation_time = start.elapsed();

    // Create a large array with the needle somewhere in the middle
    let large_data = parse_dcbor_item(
        r#"[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        {"a": 1}, {"b": 2}, {"c": 3}, {"d": 4}, {"e": 5},
        [1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12],
        "needle",
        {"final": true}
    ]"#,
    )
    .unwrap();

    let match_start = Instant::now();
    let result = pattern.matches(&large_data);
    let match_time = match_start.elapsed();

    assert!(result, "Should find needle in large structure");

    // Test paths generation and validate result
    let paths_start = Instant::now();
    let paths = pattern.paths(&large_data);
    let paths_time = paths_start.elapsed();

    #[rustfmt::skip]
    let expected = indoc! {r#"
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, {"a": 1}, {"b": 2}, {"c": 3}, {"d": 4}, {"e": 5}, [1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12], "needle", {"final": true}]
            "needle"
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Search performance should scale reasonably
    assert!(
        pattern_creation_time.as_millis() < 10,
        "Search pattern creation should be fast"
    );
    assert!(
        match_time.as_millis() < 20,
        "Search through large structure should be reasonably fast"
    );
    assert!(
        paths_time.as_millis() < 10,
        "Path generation should be fast"
    );

    println!(
        "Large structure search performance - Pattern creation: {:?}, Matching: {:?}, Paths: {:?}",
        pattern_creation_time, match_time, paths_time
    );
}

#[test]
fn test_complex_or_pattern_performance() {
    let start = Instant::now();

    // Complex OR pattern with many alternatives
    let pattern = Pattern::parse(
        r#"
        TAG(1, NUMBER) |
        TAG(2, TEXT) |
        TAG(3, [NUMBER]) |
        TAG(4, MAP(TEXT:ANY)) |
        TAG(5, BOOL) |
        MAP(TEXT("type"):TEXT("user")) |
        MAP(TEXT("type"):TEXT("admin")) |
        [TEXT("start")] |
        [NUMBER, TEXT, BOOL]
    "#,
    )
    .unwrap();
    let pattern_creation_time = start.elapsed();

    // Test with a structure that matches one of the later alternatives
    let data = parse_dcbor_item(r#"[42, "test", true]"#).unwrap();

    let match_start = Instant::now();
    let result = pattern.matches(&data);
    let match_time = match_start.elapsed();

    assert!(result, "Should match complex OR pattern");

    // Test paths generation and validate result
    let paths_start = Instant::now();
    let paths = pattern.paths(&data);
    let paths_time = paths_start.elapsed();

    #[rustfmt::skip]
    let expected = indoc! {r#"
        [42, "test", true]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // OR pattern performance should be reasonable
    assert!(
        pattern_creation_time.as_millis() < 10,
        "Complex OR pattern creation should be fast"
    );
    assert!(
        match_time.as_millis() < 10,
        "Complex OR pattern matching should be fast"
    );
    assert!(
        paths_time.as_millis() < 10,
        "Path generation should be fast"
    );

    println!(
        "Complex OR performance - Pattern creation: {:?}, Matching: {:?}, Paths: {:?}",
        pattern_creation_time, match_time, paths_time
    );
}

#[test]
fn test_vm_instruction_optimization() {
    // Test that complex patterns compile to efficient VM instructions
    let pattern = Pattern::parse(r#"TAG(100, [(MAP(TEXT("key"):NUMBER])*, TEXT("separator"), (MAP(TEXT("value"):TEXT))*))"#).unwrap();

    // Test multiple matches to ensure VM optimization is effective
    let test_cases = vec![
        r#"100(["separator"])"#,
        r#"100([{"key": 1}, "separator"])"#,
        r#"100(["separator", {"value": "test"}])"#,
        r#"100([{"key": 1}, {"key": 2}, "separator", {"value": "a"}, {"value": "b"}])"#,
    ];

    let total_start = Instant::now();
    for test_case in test_cases {
        let data = parse_dcbor_item(test_case).unwrap();
        let result = pattern.matches(&data);
        assert!(result, "Should match test case: {}", test_case);

        // Also validate paths for correctness
        let paths = pattern.paths(&data);
        assert!(!paths.is_empty(), "Should generate paths for test case: {}", test_case);
    }
    let total_time = total_start.elapsed();

    // Multiple complex matches should complete quickly
    assert!(
        total_time.as_millis() < 20,
        "VM optimization should enable fast repeated matching"
    );

    println!(
        "VM optimization performance - Total time for 4 complex matches: {:?}",
        total_time
    );
}

#[test]
fn test_edge_case_performance() {
    // Test performance with edge cases that could cause exponential behavior

    // Simpler pattern with repeats that should match the test data
    let pattern = Pattern::parse(r#"[(ANY)*]"#).unwrap();

    // Large array that the pattern should definitely match
    let large_array = parse_dcbor_item(
        r#"[
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        true, false, null,
        "more", "strings", "here"
    ]"#,
    )
    .unwrap();

    let start = Instant::now();
    let result = pattern.matches(&large_array);
    let elapsed = start.elapsed();

    assert!(result, "Should match large array with ANY repeat pattern");

    // Test paths generation and validate result
    let paths_start = Instant::now();
    let paths = pattern.paths(&large_array);
    let paths_time = paths_start.elapsed();

    #[rustfmt::skip]
    let expected = indoc! {r#"
        ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, true, false, null, "more", "strings", "here"]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not exhibit exponential behavior
    assert!(
        elapsed.as_millis() < 50,
        "ANY repeat patterns should not cause exponential behavior"
    );
    assert!(
        paths_time.as_millis() < 50,
        "Path generation should not cause exponential behavior"
    );

    println!(
        "Edge case performance - ANY repeats on large array: {:?}, Paths: {:?}",
        elapsed, paths_time
    );
}
