#[test]
fn test_parsing_with_spaces_produces_canonical_format() {
    use dcbor_pattern::Pattern;

    // Parse patterns with spaces (should work)
    let sequence_with_spaces = Pattern::parse("TEXT > NUMBER > BOOL").unwrap();
    let or_with_spaces = Pattern::parse("BOOL | TEXT | NUMBER").unwrap();
    let and_with_spaces = Pattern::parse("BOOL & TEXT & NUMBER").unwrap();

    // Parse patterns without spaces (should also work)
    let sequence_no_spaces = Pattern::parse("TEXT>NUMBER>BOOL").unwrap();
    let or_no_spaces = Pattern::parse("BOOL|TEXT|NUMBER").unwrap();
    let and_no_spaces = Pattern::parse("BOOL&TEXT&NUMBER").unwrap();

    // Both should produce the same canonical format (no spaces)
    assert_eq!(
        sequence_with_spaces.to_string(),
        sequence_no_spaces.to_string()
    );
    assert_eq!(or_with_spaces.to_string(), or_no_spaces.to_string());
    assert_eq!(and_with_spaces.to_string(), and_no_spaces.to_string());

    // Verify canonical format has no spaces
    assert_eq!(sequence_with_spaces.to_string(), "TEXT>NUMBER>BOOL");
    assert_eq!(or_with_spaces.to_string(), "BOOL|TEXT|NUMBER");
    assert_eq!(and_with_spaces.to_string(), "BOOL&TEXT&NUMBER");
}
