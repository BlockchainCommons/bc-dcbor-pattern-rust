use super::super::{
    Token,
    meta::{parse_and, parse_not},
};
use crate::{Pattern, Result};

/// Parse a sequence pattern specifically for array contents.
///
/// This parser handles the comma operator (,) for array element sequences.
/// It follows the same precedence hierarchy as the global pattern parser,
/// but uses commas instead of `>` for sequences:
/// OR -> AND -> NOT -> ARRAY_SEQUENCE -> PRIMARY
///
/// Examples:
/// - `TEXT("a"), TEXT("b")` - matches "a" followed by "b" in sequence
/// - `NUMBER(1), NUMBER(2), NUMBER(3)` - matches 1, 2, 3 in exact sequence
/// - `ANY, NUMBER(42)` - matches any value followed by the number 42
/// - `(ANY)*, NUMBER(42), (ANY)*` - matches 42 anywhere within the array

/// Parse an OR pattern for array contents.
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
    let mut patterns = vec![super::super::meta::parse_primary(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::Comma)) => {
                lexer.next(); // consume the comma token (,)
                patterns.push(super::super::meta::parse_primary(lexer)?);
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
