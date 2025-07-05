use crate::{ArrayPattern, Error, Interval, Pattern, Result, parse::Token};

/// Parse bracket-style array patterns: [pattern] or [{n}] etc.
///
/// Supports the following syntax:
/// - `[*]` - matches array with exactly one element of any type
/// - `[{0}]` - matches empty array (no elements)
/// - `[{n}]` - matches array with exactly n elements
/// - `[{n,m}]` - matches array with n to m elements (inclusive)
/// - `[{n,}]` - matches array with at least n elements
/// - `[pattern, pattern, ...]` - matches array with elements matching the given
///   patterns in order
///
/// `[]` is not a valid array pattern and will return an error.
/// Use `array` keyword for "any array" pattern.
pub(crate) fn parse_bracket_array(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    // We expect the opening bracket to already be consumed by the caller

    // Peek at the next token to determine what we're parsing
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::Range(res))) => {
            // This is a quantifier syntax: [{n}], [{n,m}], etc.
            let quantifier = res?;
            lexer.next(); // consume the Range token

            let pattern = ArrayPattern::with_length_interval(quantifier.into());

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
                ArrayPattern::with_length_interval(Interval::default()),
            )))
        }
        _ => {
            // This is a pattern syntax: [pattern] including [*]
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

/// Parse a sequence pattern specifically for array contents.
///
/// This parser handles the comma operator (,) for array element sequences.
/// It follows the same precedence hierarchy as the global pattern parser,
/// but uses commas instead of `>` for sequences:
/// OR -> AND -> NOT -> ARRAY_SEQUENCE -> PRIMARY
///
/// Examples:
/// - `"a", "b"` - matches "a" followed by "b" in sequence
/// - `1, 2, 3` - matches 1, 2, 3 in exact sequence
/// - `*, 42` - matches any value followed by the number 42
/// - `(*), 42, (*)` - matches 42 anywhere within the array
pub(crate) fn parse_array_or(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut patterns = vec![parse_array_and(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::Or)) => {
                lexer.next(); // consume the OR token
                patterns.push(parse_array_and(lexer)?);
            }
            _ => break,
        }
    }

    if patterns.len() == 1 {
        Ok(patterns.remove(0))
    } else {
        Ok(Pattern::or(patterns))
    }
}

/// Parse an AND pattern for array contents.
pub(crate) fn parse_array_and(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut patterns = vec![parse_array_not(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::And)) => {
                lexer.next(); // consume the AND token
                patterns.push(parse_array_not(lexer)?);
            }
            _ => break,
        }
    }

    if patterns.len() == 1 {
        Ok(patterns.remove(0))
    } else {
        Ok(Pattern::and(patterns))
    }
}

/// Parse a NOT pattern for array contents.
pub(crate) fn parse_array_not(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::Not)) => {
            lexer.next(); // consume the NOT token
            let pattern = parse_array_not(lexer)?; // right associative recursion
            Ok(Pattern::not_matching(pattern))
        }
        _ => parse_array_sequence(lexer),
    }
}

/// Parse a sequence pattern specifically for array contents using commas.
pub(crate) fn parse_array_sequence(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut patterns = vec![super::super::meta::parse_or(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::Comma)) => {
                lexer.next(); // consume the comma token (,)
                patterns.push(super::super::meta::parse_or(lexer)?);
            }
            _ => break,
        }
    }

    if patterns.len() == 1 {
        Ok(patterns.remove(0))
    } else {
        Ok(Pattern::sequence(patterns))
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    #[test]
    fn test_parse_bracket_array_single_any_element() {
        let mut lexer = Token::lexer("[*]");
        lexer.next(); // consume the '['
        let pattern = parse_bracket_array(&mut lexer).unwrap();

        // Should parse as array with elements pattern containing Any
        if let Pattern::Structure(crate::pattern::StructurePattern::Array(
            ArrayPattern::Elements(inner_pattern),
        )) = pattern
        {
            // Verify the inner pattern is Any
            if let Pattern::Meta(crate::pattern::MetaPattern::Any(_)) =
                inner_pattern.as_ref()
            {
                // Test passes
            } else {
                panic!(
                    "Expected inner pattern to be Any, got: {:?}",
                    inner_pattern
                );
            }
        } else {
            panic!("Expected ArrayPattern::Elements, got: {:?}", pattern);
        }
    }

    #[test]
    fn test_parse_bracket_array_empty() {
        let mut lexer = Token::lexer("[]");
        lexer.next(); // consume the '['
        let pattern = parse_bracket_array(&mut lexer).unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Array(
                ArrayPattern::with_length_interval(Interval::default())
            ))
        );
    }

    #[test]
    fn test_parse_bracket_array_with_pattern() {
        let mut lexer = Token::lexer("[42]");
        lexer.next(); // consume the '['
        let pattern = parse_bracket_array(&mut lexer).unwrap();

        // Should be an array with elements pattern
        if let Pattern::Structure(crate::pattern::StructurePattern::Array(
            ArrayPattern::Elements(_),
        )) = pattern
        {
            // Test passes
        } else {
            panic!("Expected ArrayPattern::WithElements");
        }
    }
}
