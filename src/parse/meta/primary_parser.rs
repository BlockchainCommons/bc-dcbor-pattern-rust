use super::super::{
    Token,
    structure::{parse_array, parse_map, parse_tagged},
    value::{
        parse_bool, parse_bytestring, parse_date, parse_digest,
        parse_known_value, parse_null, parse_number, parse_text,
    },
};
use crate::{Error, Pattern, Result};

/// Parse a primary pattern - the most basic unit of pattern matching.
///
/// This parser handles:
/// - ANY and NONE meta patterns
/// - Parenthesized group patterns
/// - Capture patterns (@name(...))
/// - All atomic value patterns (BOOL, TEXT, NUMBER, etc.)
/// - All structure patterns (ARRAY, MAP, TAG)
pub(crate) fn parse_primary(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let token = match lexer.next() {
        Some(Ok(tok)) => tok,
        Some(Err(e)) => {
            // Convert Unknown errors to UnrecognizedToken with span information
            match e {
                Error::Unknown => {
                    return Err(Error::UnrecognizedToken(lexer.span()));
                }
                _ => return Err(e),
            }
        }
        None => return Err(Error::UnexpectedEndOfInput),
    };

    match token {
        // Meta patterns
        Token::Any => Ok(Pattern::any()),
        Token::None => Ok(Pattern::none()),

        // Parenthesized groups - parse the inner pattern
        Token::ParenOpen => {
            let pattern = super::parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(pattern),
                Some(Ok(token)) => {
                    Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::UnexpectedEndOfInput),
            }
        }

        // Capture patterns (@name(...))
        Token::GroupName(name) => super::parse_capture(lexer, name),

        // Value patterns
        Token::Bool => parse_bool(lexer),
        Token::ByteString => parse_bytestring(lexer),
        Token::Date => parse_date(lexer),
        Token::Digest => parse_digest(lexer),
        Token::Known => parse_known_value(lexer),
        Token::Null => parse_null(lexer),
        Token::Number => parse_number(lexer),
        Token::Text => parse_text(lexer),

        // Structure patterns
        Token::Array => parse_array(lexer),
        Token::Map => parse_map(lexer),
        Token::Tagged => parse_tagged(lexer),

        // Unexpected tokens
        _ => Err(Error::UnexpectedToken(Box::new(token), lexer.span())),
    }
}
