use super::{super::Token, parse_and};
use crate::{Pattern, Result};

/// Parse an OR pattern - the top-level pattern parser.
///
/// This parser handles the OR operator (|) with left associativity.
/// It collects all patterns separated by | tokens and creates a single OR pattern.
/// If only one pattern is found, it returns that pattern directly.
///
/// This is the entry point for the pattern parsing hierarchy:
/// OR -> AND -> NOT -> PRIMARY (atomic patterns)
///
/// Examples:
/// - `BOOL | TEXT` - matches values that are either boolean OR text
/// - `NUMBER | NULL` - matches values that are either numbers OR null
/// - `ARRAY | MAP` - matches values that are either arrays OR maps
pub(crate) fn parse_or(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut patterns = vec![parse_and(lexer)?];

    loop {
        let mut lookahead = lexer.clone();
        match lookahead.next() {
            Some(Ok(Token::Or)) => {
                lexer.next(); // consume the OR token
                patterns.push(parse_and(lexer)?);
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
