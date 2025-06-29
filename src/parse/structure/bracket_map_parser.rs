use crate::{
    Error, MapPattern, Pattern, Result,
    parse::{Token, meta::parse_primary},
};

/// Parse a bracket map pattern: { ... }
///
/// Supports the following syntax:
/// - `{*}` - matches any map
/// - `{{3}}` - matches map with exactly 3 key-value pairs (interval inside map)
/// - `{{3,5}}` - matches map with 3 to 5 key-value pairs (interval inside map)
/// - `{{3,}}` - matches map with at least 3 key-value pairs (interval inside
///   map)
/// - `{pattern:pattern, pattern:pattern, ...}` - matches map with specified
///   key-value constraints
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
            // This is {*} - matches any map
            lexer.next(); // consume *
            match lexer.next() {
                Some(Ok(Token::BraceClose)) => Ok(Pattern::Structure(
                    crate::pattern::StructurePattern::Map(MapPattern::any()),
                )),
                Some(Ok(token)) => {
                    Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseBrace(lexer.span())),
            }
        }
        Some(Ok(Token::Range(quantifier_result))) => {
            // This is {interval} - map length constraint
            lexer.next(); // consume the Range token

            let quantifier = quantifier_result?;

            // Expect closing brace for the map
            match lexer.next() {
                Some(Ok(Token::BraceClose)) => {
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

                    Ok(Pattern::Structure(
                        crate::pattern::StructurePattern::Map(pattern),
                    ))
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
        let key_pattern = parse_primary(lexer)?;

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
        let value_pattern = parse_primary(lexer)?;

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

    Ok(Pattern::Structure(crate::pattern::StructurePattern::Map(
        MapPattern::with_key_value_constraints(constraints),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bracket_map_any() {
        let pattern = Pattern::parse("{*}").unwrap();
        assert!(matches!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::Any
            ))
        ));
    }

    #[test]
    fn test_parse_bracket_map_exact_count() {
        let pattern = Pattern::parse("{{3}}").unwrap();
        assert!(matches!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::WithLength(3)
            ))
        ));
    }

    #[test]
    fn test_parse_bracket_map_length_range() {
        let pattern = Pattern::parse("{{2,5}}").unwrap();
        assert!(matches!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::WithLengthRange(_)
            ))
        ));

        if let Pattern::Structure(crate::pattern::StructurePattern::Map(
            MapPattern::WithLengthRange(range),
        )) = pattern
        {
            assert_eq!(range, 2..=5);
        }
    }

    #[test]
    fn test_parse_bracket_map_open_range() {
        let pattern = Pattern::parse("{{3,}}").unwrap();
        assert!(matches!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::WithLengthRange(_)
            ))
        ));

        if let Pattern::Structure(crate::pattern::StructurePattern::Map(
            MapPattern::WithLengthRange(range),
        )) = pattern
        {
            assert_eq!(range, 3..=usize::MAX);
        }
    }

    #[test]
    fn test_parse_bracket_map_key_value_constraints() {
        let pattern =
            Pattern::parse(r#"{TEXT("key"): TEXT, NUMBER: TEXT("value")}"#)
                .unwrap();
        assert!(matches!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Map(
                MapPattern::WithKeyValueConstraints(_)
            ))
        ));
    }
}
