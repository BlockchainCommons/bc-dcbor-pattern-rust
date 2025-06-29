mod common;

use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern, format_paths};
use indoc::indoc;

#[test]
fn test_simple_nested_tagged_array() {
    #[rustfmt::skip]
    let pattern = Pattern::parse(r#"
        TAG(100, ["target"])
    "#).unwrap();

    // Should match: 100(["target"])
    let match_case = parse_dcbor_item(r#"100(["target"])"#).unwrap();
    assert!(
        pattern.matches(&match_case),
        "Should match tagged array with text"
    );

    let paths = pattern.paths(&match_case);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        100(["target"])
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    // Should not match: 100([42])
    let no_match_case = parse_dcbor_item(r#"100([42])"#).unwrap();
    assert!(
        !pattern.matches(&no_match_case),
        "Should not match tagged array with number"
    );
    let no_match_paths = pattern.paths(&no_match_case);
    assert!(no_match_paths.is_empty(), "No paths for non-matching case");

    // Should not match: 101(["target"])
    let wrong_tag_case = parse_dcbor_item(r#"101(["target"])"#).unwrap();
    assert!(
        !pattern.matches(&wrong_tag_case),
        "Should not match wrong tag"
    );
    let wrong_tag_paths = pattern.paths(&wrong_tag_case);
    assert!(wrong_tag_paths.is_empty(), "No paths for wrong tag case");
}

#[test]
fn test_complex_nested_tagged_array_with_repeat() {
    #[rustfmt::skip]
    let pattern = Pattern::parse(r#"
        TAG( 100, [(ANY)*, "target", (ANY)*] )
    "#).unwrap();

    // Should match: 100(["target"])
    let case1 = parse_dcbor_item(r#"100(["target"])"#).unwrap();
    assert!(
        pattern.matches(&case1),
        "Should match tagged array with just target"
    );
    let paths1 = pattern.paths(&case1);
    #[rustfmt::skip]
    let expected1 = indoc! {r#"
        100(["target"])
    "#}.trim();
    assert_actual_expected!(format_paths(&paths1), expected1);

    // Should match: 100([1, "target"])
    let case2 = parse_dcbor_item(r#"100([1, "target"])"#).unwrap();
    assert!(
        pattern.matches(&case2),
        "Should match tagged array with prefix"
    );
    let paths2 = pattern.paths(&case2);
    #[rustfmt::skip]
    let expected2 = indoc! {r#"
        100([1, "target"])
    "#}.trim();
    assert_actual_expected!(format_paths(&paths2), expected2);

    // Should match: 100(["target", 2])
    let case3 = parse_dcbor_item(r#"100(["target", 2])"#).unwrap();
    assert!(
        pattern.matches(&case3),
        "Should match tagged array with suffix"
    );
    let paths3 = pattern.paths(&case3);
    #[rustfmt::skip]
    let expected3 = indoc! {r#"
        100(["target", 2])
    "#}.trim();
    assert_actual_expected!(format_paths(&paths3), expected3);

    // Should match: 100([1, "target", 2])
    let case4 = parse_dcbor_item(r#"100([1, "target", 2])"#).unwrap();
    assert!(
        pattern.matches(&case4),
        "Should match tagged array with prefix and suffix"
    );
    let paths4 = pattern.paths(&case4);
    #[rustfmt::skip]
    let expected4 = indoc! {r#"
        100([1, "target", 2])
    "#}.trim();
    assert_actual_expected!(format_paths(&paths4), expected4);

    // Should not match: 100([1, 2])
    let no_match = parse_dcbor_item(r#"100([1, 2])"#).unwrap();
    assert!(
        !pattern.matches(&no_match),
        "Should not match tagged array without target"
    );
    let no_match_paths = pattern.paths(&no_match);
    assert!(no_match_paths.is_empty(), "No paths for non-matching case");
}

#[test]
fn test_map_with_array_constraints() {
    #[rustfmt::skip]
    let pattern = Pattern::parse(r#"
        {"users": [{3,}]}
    "#).unwrap();

    // Should match: {"users": [1, 2, 3]}
    let case1 = parse_dcbor_item(r#"{"users": [1, 2, 3]}"#).unwrap();
    assert!(
        pattern.matches(&case1),
        "Should match map with array of exactly 3 elements"
    );
    let paths1 = pattern.paths(&case1);
    #[rustfmt::skip]
    let expected1 = indoc! {r#"
        {"users": [1, 2, 3]}
    "#}.trim();
    assert_actual_expected!(format_paths(&paths1), expected1);

    // Should match: {"users": [1, 2, 3, 4]}
    let case2 = parse_dcbor_item(r#"{"users": [1, 2, 3, 4]}"#).unwrap();
    assert!(
        pattern.matches(&case2),
        "Should match map with array of 4 elements"
    );
    let paths2 = pattern.paths(&case2);
    #[rustfmt::skip]
    let expected2 = indoc! {r#"
        {"users": [1, 2, 3, 4]}
    "#}.trim();
    assert_actual_expected!(format_paths(&paths2), expected2);

    // Should not match: {"users": [1, 2]}
    let no_match1 = parse_dcbor_item(r#"{"users": [1, 2]}"#).unwrap();
    assert!(
        !pattern.matches(&no_match1),
        "Should not match map with array of only 2 elements"
    );
    let no_match_paths1 = pattern.paths(&no_match1);
    assert!(
        no_match_paths1.is_empty(),
        "No paths for insufficient array size"
    );

    // Should not match: {"items": [1, 2, 3]}
    let no_match2 = parse_dcbor_item(r#"{"items": [1, 2, 3]}"#).unwrap();
    assert!(
        !pattern.matches(&no_match2),
        "Should not match map with wrong key"
    );
    let no_match_paths2 = pattern.paths(&no_match2);
    assert!(no_match_paths2.is_empty(), "No paths for wrong key");
}

#[test]
fn test_array_starting_with_maps() {
    #[rustfmt::skip]
    let pattern = Pattern::parse(r#"
        [{"id": number}, (ANY)*]
    "#).unwrap();

    // Should match: [{"id": 42}]
    let case1 = parse_dcbor_item(r#"[{"id": 42}]"#).unwrap();
    assert!(
        pattern.matches(&case1),
        "Should match array with just the required map"
    );
    let paths1 = pattern.paths(&case1);
    #[rustfmt::skip]
    let expected1 = indoc! {r#"
        [{"id": 42}]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths1), expected1);

    // Should match: [{"id": 42}, "extra"]
    let case2 = parse_dcbor_item(r#"[{"id": 42}, "extra"]"#).unwrap();
    assert!(
        pattern.matches(&case2),
        "Should match array with required map and extra elements"
    );
    let paths2 = pattern.paths(&case2);
    #[rustfmt::skip]
    let expected2 = indoc! {r#"
        [{"id": 42}, "extra"]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths2), expected2);

    // Should match: [{"id": 42}, 123, true]
    let case3 = parse_dcbor_item(r#"[{"id": 42}, 123, true]"#).unwrap();
    assert!(
        pattern.matches(&case3),
        "Should match array with required map and multiple extra elements"
    );
    let paths3 = pattern.paths(&case3);
    #[rustfmt::skip]
    let expected3 = indoc! {r#"
        [{"id": 42}, 123, true]
    "#}.trim();
    assert_actual_expected!(format_paths(&paths3), expected3);

    // Should not match: [{"name": "test"}]
    let no_match1 = parse_dcbor_item(r#"[{"name": "test"}]"#).unwrap();
    assert!(
        !pattern.matches(&no_match1),
        "Should not match array with wrong map structure"
    );

    // Should not match: ["string", {"id": 42}]
    let no_match2 = parse_dcbor_item(r#"["string", {"id": 42}]"#).unwrap();
    assert!(
        !pattern.matches(&no_match2),
        "Should not match array that doesn't start with the required map"
    );
}

#[test]
fn test_deeply_nested_structures() {
    #[rustfmt::skip]
    let pattern = Pattern::parse(r#"
        TAG(200,
            {
                "data":
                [{"value": number}]
            }
        )
    "#).unwrap();

    // Should match: 200({"data": [{"value": 42}]})
    let case1 = parse_dcbor_item(r#"200({"data": [{"value": 42}]})"#).unwrap();
    assert!(
        pattern.matches(&case1),
        "Should match deeply nested structure"
    );
    let paths1 = pattern.paths(&case1);
    #[rustfmt::skip]
    let expected1 = indoc! {r#"
        200({"data": [{"value": 42}]})
    "#}.trim();
    assert_actual_expected!(format_paths(&paths1), expected1);

    // Should not match: 200({"data": [{"name": "test"}]})
    let no_match =
        parse_dcbor_item(r#"200({"data": [{"name": "test"}]})"#).unwrap();
    assert!(
        !pattern.matches(&no_match),
        "Should not match with wrong inner map structure"
    );
}

#[test]
fn test_deeply_nested_structures_with_multiple_maps() {
    // For multiple maps, we need a repeat pattern
    #[rustfmt::skip]
    let pattern = Pattern::parse(r#"
        TAG(200,
            {
                "data": [({"value": number})*]
            }
        )
    "#).unwrap();

    // Should match: 200({"data": []}) - zero maps
    let case0 = parse_dcbor_item(r#"200({"data": []})"#).unwrap();
    assert!(
        pattern.matches(&case0),
        "Should match empty array (zero maps)"
    );
    let paths0 = pattern.paths(&case0);
    #[rustfmt::skip]
    let expected0 = indoc! {r#"
        200({"data": []})
    "#}.trim();
    assert_actual_expected!(format_paths(&paths0), expected0);

    // Should match: 200({"data": [{"value": 42}]}) - one map
    let case1 = parse_dcbor_item(r#"200({"data": [{"value": 42}]})"#).unwrap();
    assert!(pattern.matches(&case1), "Should match single map");
    let paths1 = pattern.paths(&case1);
    #[rustfmt::skip]
    let expected1 = indoc! {r#"
        200({"data": [{"value": 42}]})
    "#}.trim();
    assert_actual_expected!(format_paths(&paths1), expected1);

    // Should match: 200({"data": [{"value": 1}, {"value": 2}]}) - multiple maps
    let case2 =
        parse_dcbor_item(r#"200({"data": [{"value": 1}, {"value": 2}]})"#)
            .unwrap();
    assert!(pattern.matches(&case2), "Should match multiple maps");
    let paths2 = pattern.paths(&case2);
    #[rustfmt::skip]
    let expected2 = indoc! {r#"
        200({"data": [{"value": 1}, {"value": 2}]})
    "#}.trim();
    assert_actual_expected!(format_paths(&paths2), expected2);

    // Should not match: 200({"data": [{"value": 1}, {"name": "test"}]}) - mixed
    // valid/invalid
    let no_match =
        parse_dcbor_item(r#"200({"data": [{"value": 1}, {"name": "test"}]})"#)
            .unwrap();
    assert!(
        !pattern.matches(&no_match),
        "Should not match mixed valid/invalid maps"
    );
}

#[test]
fn test_multiple_levels_of_nesting_with_any() {
    #[rustfmt::skip]
    let pattern = Pattern::parse(r#"
        TAG(300, [{ANY: ANY}, (ANY)*])
    "#).unwrap();

    // Should match: 300([{"key": "value"}])
    let case1 = parse_dcbor_item(r#"300([{"key": "value"}])"#).unwrap();
    assert!(
        pattern.matches(&case1),
        "Should match tagged array starting with any map"
    );
    let paths1 = pattern.paths(&case1);
    #[rustfmt::skip]
    let expected1 = indoc! {r#"
        300([{"key": "value"}])
    "#}.trim();
    assert_actual_expected!(format_paths(&paths1), expected1);

    // Should match: 300([{42: true}, "extra", 123])
    let case2 = parse_dcbor_item(r#"300([{42: true}, "extra", 123])"#).unwrap();
    assert!(
        pattern.matches(&case2),
        "Should match tagged array with number key map and extras"
    );
    let paths2 = pattern.paths(&case2);
    #[rustfmt::skip]
    let expected2 = indoc! {r#"
        300([{42: true}, "extra", 123])
    "#}.trim();
    assert_actual_expected!(format_paths(&paths2), expected2);

    // Should not match: 300(["string"])
    let no_match = parse_dcbor_item(r#"300(["string"])"#).unwrap();
    assert!(
        !pattern.matches(&no_match),
        "Should not match tagged array not starting with map"
    );
}

#[test]
fn test_extreme_nesting_depth() {
    // Test deeply nested structures for performance
    let pattern = Pattern::parse(r#"TAG(400, {"level1": {"level2": {"level3": [42]}}})"#).unwrap();

    let deep_structure =
        parse_dcbor_item(r#"400({"level1": {"level2": {"level3": [42]}}})"#)
            .unwrap();
    assert!(
        pattern.matches(&deep_structure),
        "Should match deeply nested structure"
    );
    let paths = pattern.paths(&deep_structure);
    #[rustfmt::skip]
    let expected = indoc! {r#"
        400({"level1": {"level2": {"level3": [42]}}})
    "#}.trim();
    assert_actual_expected!(format_paths(&paths), expected);

    let wrong_structure =
        parse_dcbor_item(r#"400({"level1": {"level2": {"level3": [43]}}})"#)
            .unwrap();
    assert!(
        !pattern.matches(&wrong_structure),
        "Should not match with wrong final value"
    );
}

#[test]
fn test_complex_combined_patterns() {
    // Combining multiple advanced patterns
    #[rustfmt::skip]
    let pattern = Pattern::parse(r#"
        TAG(500,
            [
                {"type": "user"},
                {"id": number},
                ( {"name": text} | {"email": text} )*
            ]
        )
    "#).unwrap();

    // Minimum valid structure
    let case1 =
        parse_dcbor_item(r#"500([{"type": "user"}, {"id": 123}])"#).unwrap();
    assert!(
        pattern.matches(&case1),
        "Should match minimum required structure"
    );
    let paths1 = pattern.paths(&case1);
    #[rustfmt::skip]
    let expected1 = indoc! {r#"
        500([{"type": "user"}, {"id": 123}])
    "#}.trim();
    assert_actual_expected!(format_paths(&paths1), expected1);

    // With optional name map
    let case2 = parse_dcbor_item(
        r#"500([{"type": "user"}, {"id": 123}, {"name": "John"}])"#,
    )
    .unwrap();
    assert!(
        pattern.matches(&case2),
        "Should match with optional name map"
    );
    let paths2 = pattern.paths(&case2);
    #[rustfmt::skip]
    let expected2 = indoc! {r#"
        500([{"type": "user"}, {"id": 123}, {"name": "John"}])
    "#}.trim();
    assert_actual_expected!(format_paths(&paths2), expected2);

    // With optional email map
    let case3 = parse_dcbor_item(r#"500([{"type": "user"}, {"id": 123}, {"email": "john@example.com"}])"#).unwrap();
    assert!(
        pattern.matches(&case3),
        "Should match with optional email map"
    );
    let paths3 = pattern.paths(&case3);
    #[rustfmt::skip]
    let expected3 = indoc! {r#"
        500([{"type": "user"}, {"id": 123}, {"email": "john@example.com"}])
    "#}.trim();
    assert_actual_expected!(format_paths(&paths3), expected3);

    // With multiple optional maps
    let case4 = parse_dcbor_item(r#"500([{"type": "user"}, {"id": 123}, {"name": "John"}, {"email": "john@example.com"}])"#).unwrap();
    assert!(
        pattern.matches(&case4),
        "Should match with multiple optional maps"
    );
    let paths4 = pattern.paths(&case4);
    #[rustfmt::skip]
    let expected4 = indoc! {r#"
        500([{"type": "user"}, {"id": 123}, {"name": "John"}, {"email": "john@example.com"}])
    "#}.trim();
    assert_actual_expected!(format_paths(&paths4), expected4);
}
