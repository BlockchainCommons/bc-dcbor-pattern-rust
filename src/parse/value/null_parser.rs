use crate::{Error, Pattern, Result, parse::Token};

pub(crate) fn parse_null(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    // For null, there's no parameterization like with bool(true) or number(42).
    // It's just NULL, which always matches the null value.
    Ok(Pattern::null())
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    #[test]
    fn test_parse_null() {
        let input = "NULL";
        let mut lexer = Token::lexer(input);

        // Skip the NULL token since it's already consumed by the main parser
        lexer.next();

        let pattern = parse_null(&mut lexer).unwrap();
        assert_eq!(pattern.to_string(), "NULL");
    }
}
