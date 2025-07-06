use dcbor::prelude::*;
use dcbor_pattern::{Pattern, Result};

/// Test the OR parser with various patterns
#[test]
fn test_parse_or_simple() -> Result<()> {
    let pattern = Pattern::parse("bool | text")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting (without spaces around operators)
    assert_eq!(pattern.to_string(), "bool | text");
    Ok(())
}

#[test]
fn test_parse_or_three_patterns() -> Result<()> {
    let pattern = Pattern::parse("bool | text | number")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting
    assert_eq!(pattern.to_string(), "bool | text | number");
    Ok(())
}

#[test]
fn test_parse_or_single_pattern() -> Result<()> {
    let pattern = Pattern::parse("bool")?;
    // Should return the pattern directly, not wrapped in OR
    assert!(matches!(pattern, Pattern::Value(_)));
    Ok(())
}

/// Test the AND parser with various patterns
#[test]
fn test_parse_and_simple() -> Result<()> {
    let pattern = Pattern::parse("bool & text")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting (without spaces around operators)
    assert_eq!(pattern.to_string(), "bool & text");
    Ok(())
}

#[test]
fn test_parse_and_three_patterns() -> Result<()> {
    let pattern = Pattern::parse("bool & text & number")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting
    assert_eq!(pattern.to_string(), "bool & text & number");
    Ok(())
}

/// Test the NOT parser with various patterns
#[test]
fn test_parse_not_simple() -> Result<()> {
    let pattern = Pattern::parse("!bool")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting
    assert_eq!(pattern.to_string(), "!bool");
    Ok(())
}

#[test]
fn test_parse_not_double() -> Result<()> {
    let pattern = Pattern::parse("!!bool")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting (nested NOT patterns use parentheses)
    assert_eq!(pattern.to_string(), "!(!bool)");
    Ok(())
}

/// Test operator precedence parsing (but not necessarily display)
#[test]
fn test_precedence_or_and_parsing() -> Result<()> {
    let pattern = Pattern::parse("bool | text & number")?;
    // Should parse as: bool | (text & number)
    // The exact display format may vary but it should parse correctly
    assert!(matches!(pattern, Pattern::Meta(_)));
    assert!(!pattern.to_string().is_empty());
    Ok(())
}

#[test]
fn test_precedence_and_not_parsing() -> Result<()> {
    let pattern = Pattern::parse("bool & !text")?;
    // Should parse as: bool & (!text)
    assert!(matches!(pattern, Pattern::Meta(_)));
    assert!(!pattern.to_string().is_empty());
    Ok(())
}

#[test]
fn test_precedence_or_not_parsing() -> Result<()> {
    let pattern = Pattern::parse("bool | !text")?;
    // Should parse as: bool | (!text)
    assert!(matches!(pattern, Pattern::Meta(_)));
    assert!(!pattern.to_string().is_empty());
    Ok(())
}

/// Test parentheses grouping
#[test]
fn test_parentheses_grouping_parsing() -> Result<()> {
    let pattern = Pattern::parse("(bool | text) & number")?;
    // Should parse as: (bool | text) & number
    assert!(matches!(pattern, Pattern::Meta(_)));
    // The grouping should affect the parsing structure even if display doesn't
    // show parens
    assert!(!pattern.to_string().is_empty());
    Ok(())
}

#[test]
fn test_nested_parentheses() -> Result<()> {
    let pattern = Pattern::parse("((bool))")?;
    // Should create nested RepeatPatterns with "exactly one" quantifiers
    assert!(matches!(pattern, Pattern::Meta(_)));
    assert_eq!(pattern.to_string(), "((bool){1}){1}");
    Ok(())
}

/// Test * and !* patterns
#[test]
fn test_parse_any() -> Result<()> {
    let pattern = Pattern::parse("*")?;
    assert!(matches!(pattern, Pattern::Meta(_)));
    assert_eq!(pattern.to_string(), "*");
    Ok(())
}

#[test]
fn test_parse_not_any() -> Result<()> {
    let pattern = Pattern::parse("!*")?;
    assert!(matches!(pattern, Pattern::Meta(_)));
    assert_eq!(pattern.to_string(), "!*");
    Ok(())
}

/// Test capture patterns
#[test]
fn test_parse_capture_simple() -> Result<()> {
    let pattern = Pattern::parse("@name(bool)")?;
    assert!(matches!(pattern, Pattern::Meta(_)));
    assert_eq!(pattern.to_string(), "@name(bool)");
    Ok(())
}

#[test]
fn test_parse_capture_complex() -> Result<()> {
    let pattern = Pattern::parse("@item(bool | text)")?;
    assert!(matches!(pattern, Pattern::Meta(_)));
    // Display format may not include spaces
    assert_eq!(pattern.to_string(), "@item(bool | text)");
    Ok(())
}

#[test]
fn test_parse_capture_nested() -> Result<()> {
    let pattern = Pattern::parse("@outer(@inner(bool))")?;
    assert!(matches!(pattern, Pattern::Meta(_)));
    assert_eq!(pattern.to_string(), "@outer(@inner(bool))");
    Ok(())
}

/// Test error cases
#[test]
fn test_parse_capture_missing_parens() {
    let result = Pattern::parse("@name bool");
    assert!(result.is_err());
}

#[test]
fn test_parse_capture_unclosed_parens() {
    let result = Pattern::parse("@name(bool");
    assert!(result.is_err());
}

#[test]
fn test_parse_parentheses_unclosed() {
    let result = Pattern::parse("(bool");
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_input() {
    let result = Pattern::parse("");
    assert!(result.is_err());
}

/// Test integration with other pattern types
#[test]
fn test_integration_with_structure_patterns() -> Result<()> {
    let pattern = Pattern::parse("[*] | map")?;
    assert!(matches!(pattern, Pattern::Meta(_)));
    assert_eq!(pattern.to_string(), "[*] | map");
    Ok(())
}

#[test]
fn test_integration_with_value_patterns() -> Result<()> {
    let pattern = Pattern::parse(r#""hello" | 42"#)?;
    assert!(matches!(pattern, Pattern::Meta(_)));
    assert_eq!(pattern.to_string(), r#""hello" | 42"#);
    Ok(())
}

#[test]
fn test_complex_mixed_pattern() -> Result<()> {
    let pattern =
        Pattern::parse("@result(bool | (text & !null)) | @number(number)")?;
    assert!(matches!(pattern, Pattern::Meta(_)));
    // The exact formatting might vary, just check it parses successfully
    assert!(!pattern.to_string().is_empty());
    Ok(())
}

/// Test functional correctness of precedence (not just parsing)
#[test]
fn test_precedence_functionality() -> Result<()> {
    use CBOR;
    use dcbor_pattern::Matcher;

    // Test that "bool | text & number" is parsed as "bool | (text & number)"
    // This means a boolean should match, but for the right side, both text and
    // number would need to match (which is impossible, so only bool can
    // match)
    let pattern = Pattern::parse("bool | text & number")?;

    let bool_value = CBOR::from(true);
    let text_value = CBOR::from("hello");
    let number_value = CBOR::from(42);

    // Boolean should match because of the OR
    assert!(pattern.matches(&bool_value));

    // Text should NOT match because "text & number" can never be true
    assert!(!pattern.matches(&text_value));

    // Number should NOT match because "text & number" can never be true
    assert!(!pattern.matches(&number_value));

    Ok(())
}

#[test]
fn test_grouping_functionality() -> Result<()> {
    use CBOR;
    use dcbor_pattern::Matcher;

    // Test that "(bool | text) & number" groups correctly
    // This should never match anything since no value can be both (bool or
    // text) AND number
    let pattern = Pattern::parse("(bool | text) & number")?;

    let bool_value = CBOR::from(true);
    let text_value = CBOR::from("hello");
    let number_value = CBOR::from(42);

    // Nothing should match because no value can be in two different types
    // simultaneously
    assert!(!pattern.matches(&bool_value));
    assert!(!pattern.matches(&text_value));
    assert!(!pattern.matches(&number_value));

    Ok(())
}

/// Test `search` pattern parsing
#[test]
fn test_parse_search_simple() -> Result<()> {
    let pattern = Pattern::parse("search(42)")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting
    assert_eq!(pattern.to_string(), "search(42)");
    Ok(())
}

#[test]
fn test_parse_search_with_text() -> Result<()> {
    let pattern = Pattern::parse(r#"search("hello")"#)?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting
    assert_eq!(pattern.to_string(), r#"search("hello")"#);
    Ok(())
}

#[test]
fn test_parse_search_with_any() -> Result<()> {
    let pattern = Pattern::parse("search(*)")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting
    assert_eq!(pattern.to_string(), "search(*)");
    Ok(())
}

#[test]
fn test_parse_search_with_complex_pattern() -> Result<()> {
    let pattern = Pattern::parse("search(bool | text)")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting
    assert_eq!(pattern.to_string(), "search(bool | text)");
    Ok(())
}

#[test]
fn test_parse_search_with_capture() -> Result<()> {
    let pattern = Pattern::parse("search(@found(42))")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting
    assert_eq!(pattern.to_string(), "search(@found(42))");
    Ok(())
}

#[test]
fn test_parse_search_with_nested_structure() -> Result<()> {
    let pattern = Pattern::parse("search([*])")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // Test display formatting
    assert_eq!(pattern.to_string(), "search([*])");
    Ok(())
}

#[test]
fn test_parse_search_errors() {
    // Missing opening parenthesis
    assert!(Pattern::parse("search 42").is_err());

    // Missing closing parenthesis
    assert!(Pattern::parse("search(42").is_err());

    // Empty search pattern
    assert!(Pattern::parse("search()").is_err());
}

/// Test combinations with search patterns
#[test]
fn test_parse_search_in_combinations() -> Result<()> {
    // Search within OR pattern
    let pattern = Pattern::parse("search(42) | text")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    // AND with search
    let pattern = Pattern::parse("search(42) & search(text)")?;
    assert!(matches!(pattern, Pattern::Meta(_)));

    Ok(())
}
