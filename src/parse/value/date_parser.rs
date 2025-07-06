use crate::{Pattern, Result};

pub(crate) fn parse_date(
    _lexer: &mut logos::Lexer<crate::parse::Token>,
) -> Result<Pattern> {
    // With the new simplified syntax, "date" without quotes matches any date
    Ok(Pattern::any_date())
}
