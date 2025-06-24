// NOTE: This parser is a stub implementation pending completion of the parsing infrastructure
// TODO: Implement full capture pattern parsing when or_parser is available

use crate::{Error, Pattern, Result};

/// Parse a capture pattern of the form `@name(pattern)`.
///
/// This function is called when a `GroupName` token is encountered.
/// It expects the next token to be an opening parenthesis, followed by a pattern,
/// followed by a closing parenthesis.
///
/// Currently this is a stub implementation pending completion of the parsing infrastructure.
pub(crate) fn parse_capture(
    _lexer: &mut logos::Lexer<crate::parse::Token>,
    _name: String,
) -> Result<Pattern> {
    // TODO: Implement when or_parser is available
    Err(Error::UnexpectedEndOfInput)
}
