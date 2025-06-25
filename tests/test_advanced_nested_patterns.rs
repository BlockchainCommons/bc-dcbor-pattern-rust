use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern};

#[test]
fn test_simple_nested_tagged_array() {
    // TAG(100, ARRAY(TEXT("target")))
    let pattern = Pattern::parse(r#"TAG(100, ARRAY(TEXT("target")))"#).unwrap();

    // Should match: 100(["target"])
    let match_case = parse_dcbor_item(r#"100(["target"])"#).unwrap();
    assert!(
        pattern.matches(&match_case),
        "Should match tagged array with text"
    );

    // Should not match: 100([42])
    let no_match_case = parse_dcbor_item(r#"100([42])"#).unwrap();
    assert!(
        !pattern.matches(&no_match_case),
        "Should not match tagged array with number"
    );

    // Should not match: 101(["target"])
    let wrong_tag_case = parse_dcbor_item(r#"101(["target"])"#).unwrap();
    assert!(
        !pattern.matches(&wrong_tag_case),
        "Should not match wrong tag"
    );
}

#[test]
fn test_complex_nested_tagged_array_with_repeat() {
    // TAG(100, ARRAY((ANY)*>TEXT("target")>(ANY)*))
    let pattern =
        Pattern::parse(r#"TAG(100, ARRAY((ANY)*>TEXT("target")>(ANY)*))"#)
            .unwrap();

    // Should match: 100(["target"])
    let case1 = parse_dcbor_item(r#"100(["target"])"#).unwrap();
    assert!(
        pattern.matches(&case1),
        "Should match tagged array with just target"
    );

    // Should match: 100([1, "target"])
    let case2 = parse_dcbor_item(r#"100([1, "target"])"#).unwrap();
    assert!(
        pattern.matches(&case2),
        "Should match tagged array with prefix"
    );

    // Should match: 100(["target", 2])
    let case3 = parse_dcbor_item(r#"100(["target", 2])"#).unwrap();
    assert!(
        pattern.matches(&case3),
        "Should match tagged array with suffix"
    );

    // Should match: 100([1, "target", 2])
    let case4 = parse_dcbor_item(r#"100([1, "target", 2])"#).unwrap();
    assert!(
        pattern.matches(&case4),
        "Should match tagged array with prefix and suffix"
    );

    // Should not match: 100([1, 2])
    let no_match = parse_dcbor_item(r#"100([1, 2])"#).unwrap();
    assert!(
        !pattern.matches(&no_match),
        "Should not match tagged array without target"
    );
}

#[test]
fn test_map_with_array_constraints() {
    // MAP(TEXT("users"):ARRAY({3,}))
    let pattern = Pattern::parse(r#"MAP(TEXT("users"):ARRAY({3,}))"#).unwrap();

    // Should match: {"users": [1, 2, 3]}
    let case1 = parse_dcbor_item(r#"{"users": [1, 2, 3]}"#).unwrap();
    assert!(
        pattern.matches(&case1),
        "Should match map with array of exactly 3 elements"
    );

    // Should match: {"users": [1, 2, 3, 4]}
    let case2 = parse_dcbor_item(r#"{"users": [1, 2, 3, 4]}"#).unwrap();
    assert!(
        pattern.matches(&case2),
        "Should match map with array of 4 elements"
    );

    // Should not match: {"users": [1, 2]}
    let no_match1 = parse_dcbor_item(r#"{"users": [1, 2]}"#).unwrap();
    assert!(
        !pattern.matches(&no_match1),
        "Should not match map with array of only 2 elements"
    );

    // Should not match: {"items": [1, 2, 3]}
    let no_match2 = parse_dcbor_item(r#"{"items": [1, 2, 3]}"#).unwrap();
    assert!(
        !pattern.matches(&no_match2),
        "Should not match map with wrong key"
    );
}

#[test]
fn test_array_starting_with_maps() {
    // ARRAY(MAP(TEXT("id"):NUMBER) > (ANY)*)
    let pattern =
        Pattern::parse(r#"ARRAY(MAP(TEXT("id"):NUMBER) > (ANY)*)"#).unwrap();

    // Should match: [{"id": 42}]
    let case1 = parse_dcbor_item(r#"[{"id": 42}]"#).unwrap();
    assert!(
        pattern.matches(&case1),
        "Should match array with just the required map"
    );

    // Should match: [{"id": 42}, "extra"]
    let case2 = parse_dcbor_item(r#"[{"id": 42}, "extra"]"#).unwrap();
    assert!(
        pattern.matches(&case2),
        "Should match array with required map and extra elements"
    );

    // Should match: [{"id": 42}, 123, true]
    let case3 = parse_dcbor_item(r#"[{"id": 42}, 123, true]"#).unwrap();
    assert!(
        pattern.matches(&case3),
        "Should match array with required map and multiple extra elements"
    );

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
    // TAG(200, MAP(TEXT("data"):ARRAY(MAP(TEXT("value"):NUMBER))))
    let pattern = Pattern::parse(
        r#"TAG(200, MAP(TEXT("data"):ARRAY(MAP(TEXT("value"):NUMBER))))"#,
    )
    .unwrap();

    // Should match: 200({"data": [{"value": 42}]})
    let case1 = parse_dcbor_item(r#"200({"data": [{"value": 42}]})"#).unwrap();
    assert!(
        pattern.matches(&case1),
        "Should match deeply nested structure"
    );

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
    // TAG(200, MAP(TEXT("data"):ARRAY((MAP(TEXT("value"):NUMBER))*)))
    let pattern = Pattern::parse(
        r#"TAG(200, MAP(TEXT("data"):ARRAY((MAP(TEXT("value"):NUMBER))*)))"#,
    )
    .unwrap();

    // Should match: 200({"data": []}) - zero maps
    let case0 = parse_dcbor_item(r#"200({"data": []})"#).unwrap();
    assert!(
        pattern.matches(&case0),
        "Should match empty array (zero maps)"
    );

    // Should match: 200({"data": [{"value": 42}]}) - one map
    let case1 = parse_dcbor_item(r#"200({"data": [{"value": 42}]})"#).unwrap();
    assert!(pattern.matches(&case1), "Should match single map");

    // Should match: 200({"data": [{"value": 1}, {"value": 2}]}) - multiple maps
    let case2 =
        parse_dcbor_item(r#"200({"data": [{"value": 1}, {"value": 2}]})"#)
            .unwrap();
    assert!(pattern.matches(&case2), "Should match multiple maps");

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
    // TAG(300, ARRAY(MAP(ANY:ANY) > (ANY)*))
    let pattern =
        Pattern::parse(r#"TAG(300, ARRAY(MAP(ANY:ANY) > (ANY)*))"#).unwrap();

    // Should match: 300([{"key": "value"}])
    let case1 = parse_dcbor_item(r#"300([{"key": "value"}])"#).unwrap();
    assert!(
        pattern.matches(&case1),
        "Should match tagged array starting with any map"
    );

    // Should match: 300([{42: true}, "extra", 123])
    let case2 = parse_dcbor_item(r#"300([{42: true}, "extra", 123])"#).unwrap();
    assert!(
        pattern.matches(&case2),
        "Should match tagged array with number key map and extras"
    );

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
    // TAG(400, MAP(TEXT("level1"):MAP(TEXT("level2"):MAP(TEXT("level3"):
    // ARRAY(NUMBER(42))))))
    let pattern = Pattern::parse(r#"TAG(400, MAP(TEXT("level1"):MAP(TEXT("level2"):MAP(TEXT("level3"):ARRAY(NUMBER(42))))))"#).unwrap();

    let deep_structure =
        parse_dcbor_item(r#"400({"level1": {"level2": {"level3": [42]}}})"#)
            .unwrap();
    assert!(
        pattern.matches(&deep_structure),
        "Should match deeply nested structure"
    );

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
    // TAG(500, ARRAY(MAP(TEXT("type"):TEXT("user")) > MAP(TEXT("id"):NUMBER) >
    // (MAP(TEXT("name"):TEXT) | MAP(TEXT("email"):TEXT))*))
    let pattern = Pattern::parse(r#"TAG(500, ARRAY(MAP(TEXT("type"):TEXT("user")) > MAP(TEXT("id"):NUMBER) > (MAP(TEXT("name"):TEXT) | MAP(TEXT("email"):TEXT))*))"#).unwrap();

    // Minimum valid structure
    let case1 =
        parse_dcbor_item(r#"500([{"type": "user"}, {"id": 123}])"#).unwrap();
    assert!(
        pattern.matches(&case1),
        "Should match minimum required structure"
    );

    // With optional name map
    let case2 = parse_dcbor_item(
        r#"500([{"type": "user"}, {"id": 123}, {"name": "John"}])"#,
    )
    .unwrap();
    assert!(
        pattern.matches(&case2),
        "Should match with optional name map"
    );

    // With optional email map
    let case3 = parse_dcbor_item(r#"500([{"type": "user"}, {"id": 123}, {"email": "john@example.com"}])"#).unwrap();
    assert!(
        pattern.matches(&case3),
        "Should match with optional email map"
    );

    // With multiple optional maps
    let case4 = parse_dcbor_item(r#"500([{"type": "user"}, {"id": 123}, {"name": "John"}, {"email": "john@example.com"}])"#).unwrap();
    assert!(
        pattern.matches(&case4),
        "Should match with multiple optional maps"
    );
}
