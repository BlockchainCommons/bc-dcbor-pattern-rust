use super::{super::Token, parse_not};
use crate::{Pattern, Result};

/// Parse an AND pattern.
///
/// This parser handles the AND operator (&) with left associativity.
/// It collects all patterns separated by & tokens and creates a single AND
/// pattern. If only one pattern is found, it returns that pattern directly.
///
/// Examples:
/// - `bool & text` - matches values that are both boolean AND text (impossible,
///   always fails)
/// - `number & (>= 0)` - matches numbers that are also >= 0
/// - `[*] & map` - matches values that are both arrays AND maps (impossible,
///   always fails)
pub(crate) fn parse_and(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut patterns = vec![parse_not(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::And)) => {
                lexer.next(); // consume the AND token
                patterns.push(parse_not(lexer)?);
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
