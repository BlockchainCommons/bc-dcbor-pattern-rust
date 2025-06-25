use crate::{ArrayPattern, Error, Pattern, Result, parse::Token};

/// Parse an ARRAY pattern.
///
/// Supports the following syntax:
/// - `ARRAY` - matches any array
/// - `ARRAY({n})` - matches array with exactly n elements
/// - `ARRAY({n,m})` - matches array with n to m elements (inclusive)
/// - `ARRAY({n,})` - matches array with at least n elements
pub(crate) fn parse_array(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            // Consume the '(' token
            lexer.next();

            match lexer.next() {
                Some(Ok(Token::Range(res))) => {
                    let quantifier = res?;

                    // Convert quantifier to appropriate ArrayPattern
                    let pattern = if let Some(max) = quantifier.max() {
                        if quantifier.min() == max {
                            // Exact count: {n}
                            ArrayPattern::with_length(quantifier.min())
                        } else {
                            // Range: {n,m}
                            ArrayPattern::with_length_range(
                                quantifier.min()..=max,
                            )
                        }
                    } else {
                        // Open-ended range: {n,}
                        ArrayPattern::with_length_range(
                            quantifier.min()..=usize::MAX,
                        )
                    };

                    // Expect closing parenthesis
                    match lexer.next() {
                        Some(Ok(Token::ParenClose)) => Ok(Pattern::Structure(
                            crate::pattern::StructurePattern::Array(pattern),
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
            // No parentheses, just "ARRAY" - matches any array
            Ok(Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::any(),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    #[test]
    fn test_parse_array_any() {
        let pattern = Pattern::parse("ARRAY").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::any()
            ))
        );
        assert_eq!(pattern.to_string(), "ARRAY");
    }

    #[test]
    fn test_parse_array_exact_count() {
        let pattern = Pattern::parse("ARRAY({3})").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_length(3)
            ))
        );
        assert_eq!(pattern.to_string(), "ARRAY({3})");
    }

    #[test]
    fn test_parse_array_range() {
        let pattern = Pattern::parse("ARRAY({2,5})").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_length_range(2..=5)
            ))
        );
        assert_eq!(pattern.to_string(), "ARRAY({2,5})");
    }

    #[test]
    fn test_parse_array_open_range() {
        let pattern = Pattern::parse("ARRAY({2,})").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_length_range(2..=usize::MAX)
            ))
        );
        assert_eq!(pattern.to_string(), "ARRAY({2,})");
    }

    #[test]
    fn test_parse_array_error_missing_close_paren() {
        let result = Pattern::parse("ARRAY({3}");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_array_error_invalid_range() {
        let result = Pattern::parse("ARRAY(invalid)");
        assert!(result.is_err());
    }
}
