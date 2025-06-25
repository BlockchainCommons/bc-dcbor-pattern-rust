use super::{super::Token, parse_or};
use crate::{Error, Pattern, Result};

/// Parse a capture pattern of the form `@name(pattern)`.
///
/// This function is called when a `GroupName` token is encountered.
/// It expects the next token to be an opening parenthesis, followed by a
/// pattern, followed by a closing parenthesis.
///
/// Examples:
/// - `@count(NUMBER)` - captures any number with the name "count"
/// - `@name(TEXT)` - captures any text with the name "name"
/// - `@item(ARRAY | MAP)` - captures any array or map with the name "item"
pub(crate) fn parse_capture(
    lexer: &mut logos::Lexer<Token>,
    name: String,
) -> Result<Pattern> {
    match lexer.next() {
        Some(Ok(Token::ParenOpen)) => {
            let pattern = parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => {
                    Ok(Pattern::capture(name, pattern))
                }
                Some(Ok(token)) => {
                    Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
                }
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
