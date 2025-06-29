#[test]
fn test_parsing_with_spaces_produces_canonical_format() {
    use dcbor_pattern::Pattern;

    // Parse patterns with spaces (should work)
    let or_with_spaces = Pattern::parse("bool | text | number").unwrap();
    let and_with_spaces = Pattern::parse("bool & text & number").unwrap();

    // Parse patterns without spaces (should also work)
    let or_no_spaces = Pattern::parse("bool|text|number").unwrap();
    let and_no_spaces = Pattern::parse("bool&text&number").unwrap();

    // Both should produce the same canonical format (no spaces)
    assert_eq!(or_with_spaces.to_string(), or_no_spaces.to_string());
    assert_eq!(and_with_spaces.to_string(), and_no_spaces.to_string());

    // Verify canonical format has no spaces
    assert_eq!(or_with_spaces.to_string(), "bool|text|number");
    assert_eq!(and_with_spaces.to_string(), "bool&text&number");
}
