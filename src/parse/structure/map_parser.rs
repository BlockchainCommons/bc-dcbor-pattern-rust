use crate::{Error, MapPattern, Pattern, Result, parse::Token};
use crate::parse::meta::parse_primary;

/// Parse a MAP pattern.
///
/// Supports the following syntax:
/// - `MAP` - matches any map
/// - `MAP({n})` - matches map with exactly n key-value pairs
/// - `MAP({n,m})` - matches map with n to m key-value pairs (inclusive)
/// - `MAP({n,})` - matches map with at least n key-value pairs
/// - `MAP(pattern:pattern, pattern:pattern, ...)` - matches map with specified key-value constraints
pub(crate) fn parse_map(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            // Consume the '(' token
            lexer.next();

            // Check if this is a range pattern or key-value constraints
            let mut lookahead2 = lexer.clone();
            match lookahead2.next() {
                Some(Ok(Token::Range(_))) => {
                    // This is a range pattern: MAP({n}) or MAP({n,m})
                    match lexer.next() {
                        Some(Ok(Token::Range(res))) => {
                            let quantifier = res?;

                            // Convert quantifier to appropriate MapPattern
                            let pattern = if let Some(max) = quantifier.max() {
                                if quantifier.min() == max {
                                    // Exact count: {n}
                                    MapPattern::with_length(quantifier.min())
                                } else {
                                    // Range: {n,m}
                                    MapPattern::with_length_range(
                                        quantifier.min()..=max,
                                    )
                                }
                            } else {
                                // Open-ended range: {n,}
                                MapPattern::with_length_range(
                                    quantifier.min()..=usize::MAX,
                                )
                            };

                            // Expect closing parenthesis
                            match lexer.next() {
                                Some(Ok(Token::ParenClose)) => Ok(Pattern::Structure(
                                    crate::pattern::StructurePattern::Map(pattern),
                                )),
                                Some(Ok(token)) => Err(Error::UnexpectedToken(
                                    Box::new(token),
                                    lexer.span(),
                                )),
                                Some(Err(e)) => Err(e),
                                None => Err(Error::ExpectedCloseParen(lexer.span())),
                            }
                        }
                        Some(Ok(token)) => {
                            Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
                        }
                        Some(Err(e)) => Err(e),
                        None => Err(Error::UnexpectedEndOfInput),
                    }
                }
                _ => {
                    // This should be key-value constraints: MAP(pattern:pattern, ...)
                    parse_key_value_constraints(lexer)
                }
            }
        }
        _ => {
            // No parentheses, just "MAP" - matches any map
            Ok(Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::any(),
            )))
        }
    }
}

/// Parse key-value constraints for MAP patterns.
fn parse_key_value_constraints(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut constraints = Vec::new();

    loop {
        // Parse the key pattern
        let key_pattern = parse_primary(lexer)?;

        // Expect colon
        match lexer.next() {
            Some(Ok(Token::Colon)) => {}
            Some(Ok(token)) => {
                return Err(Error::UnexpectedToken(Box::new(token), lexer.span()));
            }
            Some(Err(e)) => return Err(e),
            None => return Err(Error::UnexpectedEndOfInput),
        }

        // Parse the value pattern
        let value_pattern = parse_primary(lexer)?;

        constraints.push((key_pattern, value_pattern));

        // Check if there's a comma for more constraints
        match lexer.next() {
            Some(Ok(Token::Comma)) => {
                // Continue parsing more constraints
                continue;
            }
            Some(Ok(Token::ParenClose)) => {
                // End of constraints
                break;
            }
            Some(Ok(token)) => {
                return Err(Error::UnexpectedToken(Box::new(token), lexer.span()));
            }
            Some(Err(e)) => return Err(e),
            None => return Err(Error::ExpectedCloseParen(lexer.span())),
        }
    }

    let pattern = MapPattern::with_key_value_constraints(constraints);
    Ok(Pattern::Structure(crate::pattern::StructurePattern::Map(pattern)))
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    #[test]
    fn test_parse_map_any() {
        let pattern = Pattern::parse("MAP").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::any()
            ))
        );
        assert_eq!(pattern.to_string(), "MAP");
    }

    #[test]
    fn test_parse_map_exact_count() {
        let pattern = Pattern::parse("MAP({3})").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::with_length(3)
            ))
        );
        assert_eq!(pattern.to_string(), "MAP({3})");
    }

    #[test]
    fn test_parse_map_range() {
        let pattern = Pattern::parse("MAP({2,5})").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::with_length_range(2..=5)
            ))
        );
        assert_eq!(pattern.to_string(), "MAP({2,5})");
    }

    #[test]
    fn test_parse_map_open_range() {
        let pattern = Pattern::parse("MAP({3,})").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::with_length_range(3..=usize::MAX)
            ))
        );
        assert_eq!(pattern.to_string(), "MAP({3,})");
    }

    #[test]
    fn test_parse_map_zero_exact() {
        let pattern = Pattern::parse("MAP({0})").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::with_length(0)
            ))
        );
        assert_eq!(pattern.to_string(), "MAP({0})");
    }

    #[test]
    fn test_parse_map_zero_range() {
        let pattern = Pattern::parse("MAP({0,3})").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::with_length_range(0..=3)
            ))
        );
        assert_eq!(pattern.to_string(), "MAP({0,3})");
    }

    #[test]
    fn test_parse_map_invalid_token() {
        let result = Pattern::parse("MAP(invalid)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_map_missing_close_paren() {
        let result = Pattern::parse("MAP({3}");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_map_empty_parens() {
        let result = Pattern::parse("MAP()");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_map_single_key_value_constraint() {
        let pattern = Pattern::parse(r#"MAP(TEXT("key"):NUMBER)"#).unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::with_key_value_constraints(vec![
                    (Pattern::text("key"), Pattern::any_number())
                ])
            ))
        );
        assert_eq!(pattern.to_string(), r#"MAP(TEXT("key"):NUMBER)"#);
    }

    #[test]
    fn test_parse_map_multiple_key_value_constraints() {
        let pattern = Pattern::parse(r#"MAP(TEXT("name"):TEXT, TEXT("age"):NUMBER)"#).unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::with_key_value_constraints(vec![
                    (Pattern::text("name"), Pattern::any_text()),
                    (Pattern::text("age"), Pattern::any_number())
                ])
            ))
        );
        assert_eq!(pattern.to_string(), r#"MAP(TEXT("name"):TEXT, TEXT("age"):NUMBER)"#);
    }

    #[test]
    fn test_parse_map_any_key_specific_value() {
        let pattern = Pattern::parse(r#"MAP(ANY:TEXT("value"))"#).unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::with_key_value_constraints(vec![
                    (Pattern::any(), Pattern::text("value"))
                ])
            ))
        );
        assert_eq!(pattern.to_string(), r#"MAP(ANY:TEXT("value"))"#);
    }

    #[test]
    fn test_parse_map_specific_key_any_value() {
        let pattern = Pattern::parse(r#"MAP(TEXT("key"):ANY)"#).unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::with_key_value_constraints(vec![
                    (Pattern::text("key"), Pattern::any())
                ])
            ))
        );
        assert_eq!(pattern.to_string(), r#"MAP(TEXT("key"):ANY)"#);
    }

    #[test]
    fn test_parse_map_complex_patterns() {
        let pattern = Pattern::parse(r#"MAP(NUMBER(42):BOOL(true), TEXT("test"):NULL)"#).unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::with_key_value_constraints(vec![
                    (Pattern::number(42.0), Pattern::bool(true)),
                    (Pattern::text("test"), Pattern::null())
                ])
            ))
        );
        assert_eq!(pattern.to_string(), r#"MAP(NUMBER(42):BOOL(true), TEXT("test"):NULL)"#);
    }

    #[test]
    fn test_parse_map_missing_colon() {
        let result = Pattern::parse(r#"MAP(TEXT("key") NUMBER)"#);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_map_missing_value_pattern() {
        let result = Pattern::parse(r#"MAP(TEXT("key"):)"#);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_map_trailing_comma() {
        let result = Pattern::parse(r#"MAP(TEXT("key"):NUMBER,)"#);
        assert!(result.is_err());
    }
}
