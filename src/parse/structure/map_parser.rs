use crate::{
    Error, MapPattern, Pattern, Result, StructurePattern,
    parse::{
        Token,
        meta::{parse_or, parse_primary},
    },
};

/// Parse a bracket map pattern: { ... }
///
/// Supports the following syntax:
/// - `{*}` - matches any map
/// - `{{0}}` - matches map with exactly 0 key-value pairs (empty map)
/// - `{{n}}` - matches map with exactly n key-value pairs
/// - `{{n,m}}` - matches map with n to m key-value pairs
/// - `{{n,}}` - matches map with at least n key-value pairs
/// - `{pattern:pattern, pattern:pattern, ...}` - matches map with specified
///   key-value constraints
///
/// `{}` is not a valid map pattern and will return an error.
pub(crate) fn parse_bracket_map(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    // We expect the opening brace to already be consumed by the caller

    // We need to look ahead to distinguish between:
    // 1. {*} - map wildcard
    // 2. {interval} - length constraints (interval {n}, {n,m}, {n,})
    // 3. {pattern:pattern} - key-value constraints

    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::RepeatZeroOrMore)) => {
            // Check if this is {*} or {*:...}
            let mut lookahead2 = lookahead.clone();
            match lookahead2.next() {
                Some(Ok(Token::BraceClose)) => {
                    // This is {*} - matches any map
                    lexer.next(); // consume *
                    lexer.next(); // consume }
                    Ok(Pattern::Structure(StructurePattern::Map(
                        MapPattern::any(),
                    )))
                }
                Some(Ok(Token::Colon)) => {
                    // This is {*:pattern} - key-value constraint with * as key
                    parse_key_value_constraints(lexer)
                }
                Some(Ok(token)) => Err(Error::UnexpectedToken(
                    Box::new(token),
                    lookahead2.span(),
                )),
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseBrace(lookahead2.span())),
            }
        }
        Some(Ok(Token::Range(quantifier_result))) => {
            // This is {interval} - map length constraint
            lexer.next(); // consume the Range token

            let quantifier = quantifier_result?;

            // Expect closing brace for the map
            match lexer.next() {
                Some(Ok(Token::BraceClose)) => {
                    let pattern =
                        MapPattern::with_length_interval(quantifier.interval());

                    Ok(Pattern::Structure(StructurePattern::Map(pattern)))
                }
                Some(Ok(token)) => {
                    Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseBrace(lexer.span())),
            }
        }
        _ => {
            // This should be key-value constraints: {pattern:pattern, ...}
            parse_key_value_constraints(lexer)
        }
    }
}

/// Parse key-value constraints for bracket map patterns.
fn parse_key_value_constraints(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut constraints = Vec::new();

    loop {
        // Parse the key pattern
        let key_pattern = parse_or(lexer)?;

        // Expect colon
        match lexer.next() {
            Some(Ok(Token::Colon)) => {}
            Some(Ok(token)) => {
                return Err(Error::UnexpectedToken(
                    Box::new(token),
                    lexer.span(),
                ));
            }
            Some(Err(e)) => return Err(e),
            None => return Err(Error::ExpectedColon(lexer.span())),
        }

        // Parse the value pattern
        let value_pattern = parse_or(lexer)?;

        constraints.push((key_pattern, value_pattern));

        // Check what comes next
        match lexer.next() {
            Some(Ok(Token::Comma)) => {
                // Continue parsing more constraints
                continue;
            }
            Some(Ok(Token::BraceClose)) => {
                // End of map pattern
                break;
            }
            Some(Ok(token)) => {
                return Err(Error::UnexpectedToken(
                    Box::new(token),
                    lexer.span(),
                ));
            }
            Some(Err(e)) => return Err(e),
            None => return Err(Error::ExpectedCloseBrace(lexer.span())),
        }
    }

    Ok(Pattern::Structure(StructurePattern::Map(
        MapPattern::with_key_value_constraints(constraints),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Interval;

    #[test]
    fn test_parse_bracket_map_any() {
        let pattern = Pattern::parse("{*}").unwrap();
        assert!(matches!(
            pattern,
            Pattern::Structure(StructurePattern::Map(MapPattern::Any))
        ));
    }

    #[test]
    fn test_parse_bracket_map_exact_count() {
        let pattern = Pattern::parse("{{3}}").unwrap();
        let interval = Interval::new(3..=3);
        assert!(matches!(
            pattern,
            Pattern::Structure(StructurePattern::Map(MapPattern::Length(i)))
        ));
    }

    #[test]
    fn test_parse_bracket_map_length_range() {
        let pattern = Pattern::parse("{{2,5}}").unwrap();
        assert!(matches!(
            pattern,
            Pattern::Structure(StructurePattern::Map(MapPattern::Length(_)))
        ));

        if let Pattern::Structure(StructurePattern::Map(MapPattern::Length(
            interval,
        ))) = pattern
        {
            assert_eq!(interval, Interval::new(2..=5));
        }
    }

    #[test]
    fn test_parse_bracket_map_open_range() {
        let pattern = Pattern::parse("{{3,}}").unwrap();
        assert!(matches!(
            pattern,
            Pattern::Structure(StructurePattern::Map(MapPattern::Length(_)))
        ));

        if let Pattern::Structure(StructurePattern::Map(MapPattern::Length(
            interval,
        ))) = pattern
        {
            assert_eq!(interval, Interval::new(3..));
        }
    }

    #[test]
    fn test_parse_bracket_map_key_value_constraints() {
        let pattern =
            Pattern::parse(r#"{"key": text, number: "value"}"#).unwrap();
        assert!(matches!(
            pattern,
            Pattern::Structure(StructurePattern::Map(MapPattern::Constraints(
                _
            )))
        ));
    }
}
