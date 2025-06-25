use dcbor_pattern::{Pattern, Result};

#[test]
fn test_parse_sequence_simple() -> Result<()> {
    let pattern = Pattern::parse(r#"TEXT("hello") > NUMBER(42)"#)?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting includes sequence operator
    let display = pattern.to_string();
    assert!(display.contains(">"));
    assert!(display.contains(r#"TEXT("hello")"#));
    assert!(display.contains("NUMBER(42)"));
    Ok(())
}

#[test]
fn test_parse_sequence_three_patterns() -> Result<()> {
    let pattern = Pattern::parse("BOOL > TEXT > NUMBER")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting
    let display = pattern.to_string();
    assert!(display.contains("BOOL>TEXT>NUMBER"));
    Ok(())
}

#[test]
fn test_parse_sequence_single_pattern() -> Result<()> {
    let pattern = Pattern::parse("BOOL")?;
    // Should return the pattern directly, not wrapped in sequence
    assert!(matches!(pattern, Pattern::Value(_)));
    Ok(())
}

#[test]
fn test_parse_sequence_with_parentheses() -> Result<()> {
    let pattern = Pattern::parse("(TEXT > NUMBER) | BOOL")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Should parse as OR of (sequence) and BOOL
    let display = pattern.to_string();
    assert!(display.contains("|"));
    assert!(display.contains(">"));
    Ok(())
}

#[test]
fn test_parse_sequence_precedence() -> Result<()> {
    // Test precedence: OR > AND > NOT > SEQUENCE > PRIMARY
    let pattern = Pattern::parse("BOOL | TEXT > NUMBER")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Should parse as: BOOL | (TEXT > NUMBER)
    // Because sequence has higher precedence than OR
    let display = pattern.to_string();
    println!("Precedence test result: {}", display);
    Ok(())
}

#[test]
fn test_final_format_verification() {
    let sequence =
        Pattern::parse(r#"TEXT("hello") > NUMBER(42) > BOOL(true)"#).unwrap();
    let or_pattern = Pattern::parse("BOOL | TEXT").unwrap();
    let and_pattern = Pattern::parse("BOOL & TEXT").unwrap();

    println!("Final sequence display: '{}'", sequence);
    println!("Final OR display: '{}'", or_pattern);
    println!("Final AND display: '{}'", and_pattern);

    // Verify no spaces around operators
    assert_eq!(
        sequence.to_string(),
        r#"TEXT("hello")>NUMBER(42)>BOOL(true)"#
    );
    assert_eq!(or_pattern.to_string(), "BOOL|TEXT");
    assert_eq!(and_pattern.to_string(), "BOOL&TEXT");
}
