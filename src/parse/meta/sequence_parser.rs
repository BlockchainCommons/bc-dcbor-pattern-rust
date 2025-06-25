use super::{super::Token, parse_primary};
use crate::{Pattern, Result};

/// Parse a sequence pattern.
///
/// This parser handles the sequence operator (>) with left associativity.
/// It collects all patterns separated by > tokens and creates a single sequence
/// pattern. If only one pattern is found, it returns that pattern directly.
///
/// Examples:
/// - `TEXT("a") > TEXT("b")` - matches "a" followed by "b" in sequence
/// - `NUMBER(1) > NUMBER(2) > NUMBER(3)` - matches 1, 2, 3 in exact sequence
/// - `ANY > NUMBER(42)` - matches any value followed by the number 42
pub(crate) fn parse_sequence(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut patterns = vec![parse_primary(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::Sequence)) => {
                lexer.next(); // consume the sequence token (>)
                patterns.push(parse_primary(lexer)?);
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
