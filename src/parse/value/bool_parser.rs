use crate::{Pattern, Result, parse::Token};

pub(crate) fn parse_bool(_lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    // Just return the pattern for any boolean
    Ok(Pattern::any_bool())
}

pub(crate) fn parse_bool_true(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    // Return pattern for the specific boolean value true
    Ok(Pattern::bool(true))
}

pub(crate) fn parse_bool_false(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    // Return pattern for the specific boolean value false
    Ok(Pattern::bool(false))
}
