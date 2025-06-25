use crate::{Error, MapPattern, Pattern, Result, parse::Token};

/// Parse a MAP pattern.
///
/// Supports the following syntax:
/// - `MAP` - matches any map
/// - `MAP({n})` - matches map with exactly n key-value pairs
/// - `MAP({n,m})` - matches map with n to m key-value pairs (inclusive)
/// - `MAP({n,})` - matches map with at least n key-value pairs
pub(crate) fn parse_map(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            // Consume the '(' token
            lexer.next();

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
            // No parentheses, just "MAP" - matches any map
            Ok(Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::any(),
            )))
        }
    }
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
}
