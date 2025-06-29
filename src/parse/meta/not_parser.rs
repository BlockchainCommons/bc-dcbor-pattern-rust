use super::{super::Token, parse_primary};
use crate::{Pattern, Result};

/// Parse a NOT pattern or delegate to primary parser.
///
/// This parser handles the NOT operator (!) with right associativity.
/// If no NOT token is found, it delegates to the primary parser.
///
/// Examples:
/// - `!BOOL` - matches anything that is not a boolean
/// - `!!TEXT` - matches anything that is not (not text), i.e., matches text
/// - `![*]` - matches anything that is not an array
pub(crate) fn parse_not(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::Not)) => {
            lexer.next(); // consume the NOT token
            let pattern = parse_not(lexer)?; // right associative recursion
            Ok(Pattern::not_matching(pattern))
        }
        _ => parse_primary(lexer),
    }
}
