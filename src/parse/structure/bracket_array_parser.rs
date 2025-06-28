use crate::{ArrayPattern, Error, Pattern, Result, parse::Token};

/// Parse bracket-style array patterns: [pattern] or [*] or [{n}] etc.
///
/// Supports the following syntax:
/// - `[*]` - matches any array (wildcard)
/// - `[{n}]` - matches array with exactly n elements
/// - `[{n,m}]` - matches array with n to m elements (inclusive)
/// - `[{n,}]` - matches array with at least n elements
/// - `[pattern]` - matches array with elements matching the given pattern
pub(crate) fn parse_bracket_array(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    // We expect the opening bracket to already be consumed by the caller

    // Peek at the next token to determine what we're parsing
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::RepeatZeroOrMore)) => {
            // This is [*] - wildcard array pattern
            lexer.next(); // consume the * token

            // Expect closing bracket
            match lexer.next() {
                Some(Ok(Token::BracketClose)) => Ok(Pattern::Structure(
                    crate::pattern::StructurePattern::Array(ArrayPattern::any()),
                )),
                Some(Ok(token)) => {
                    Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseBracket(lexer.span())),
            }
        }
        Some(Ok(Token::Range(res))) => {
            // This is a quantifier syntax: [{n}], [{n,m}], etc.
            let quantifier = res?;
            lexer.next(); // consume the Range token

            // Convert quantifier to appropriate ArrayPattern
            let pattern = if let Some(max) = quantifier.max() {
                if quantifier.min() == max {
                    // Exact count: {n}
                    ArrayPattern::with_length(quantifier.min())
                } else {
                    // Range: {n,m}
                    ArrayPattern::with_length_range(quantifier.min()..=max)
                }
            } else {
                // Open-ended range: {n,}
                ArrayPattern::with_length_range(quantifier.min()..=usize::MAX)
            };

            // Expect closing bracket
            match lexer.next() {
                Some(Ok(Token::BracketClose)) => Ok(Pattern::Structure(
                    crate::pattern::StructurePattern::Array(pattern),
                )),
                Some(Ok(token)) => {
                    Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseBracket(lexer.span())),
            }
        }
        Some(Ok(Token::BracketClose)) => {
            // This is [] - empty array (no elements)
            lexer.next(); // consume the closing bracket
            Ok(Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_length(0),
            )))
        }
        _ => {
            // This is a pattern syntax: [pattern]
            // Parse the inner pattern using array-specific parsing (commas for
            // sequences)
            let element_pattern = super::parse_array_or(lexer)?;
            let pattern = ArrayPattern::with_elements(element_pattern);

            // Expect closing bracket
            match lexer.next() {
                Some(Ok(Token::BracketClose)) => Ok(Pattern::Structure(
                    crate::pattern::StructurePattern::Array(pattern),
                )),
                Some(Ok(token)) => {
                    Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseBracket(lexer.span())),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    #[test]
    fn test_parse_bracket_array_wildcard() {
        let mut lexer = Token::lexer("[*]");
        lexer.next(); // consume the '['
        let pattern = parse_bracket_array(&mut lexer).unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::any()
            ))
        );
    }

    #[test]
    fn test_parse_bracket_array_empty() {
        let mut lexer = Token::lexer("[]");
        lexer.next(); // consume the '['
        let pattern = parse_bracket_array(&mut lexer).unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_length(0)
            ))
        );
    }

    #[test]
    fn test_parse_bracket_array_with_pattern() {
        let mut lexer = Token::lexer("[NUMBER(42)]");
        lexer.next(); // consume the '['
        let pattern = parse_bracket_array(&mut lexer).unwrap();

        // Should be an array with elements pattern
        if let Pattern::Structure(crate::pattern::StructurePattern::Array(
            ArrayPattern::WithElements(_),
        )) = pattern
        {
            // Test passes
        } else {
            panic!("Expected ArrayPattern::WithElements");
        }
    }
}
