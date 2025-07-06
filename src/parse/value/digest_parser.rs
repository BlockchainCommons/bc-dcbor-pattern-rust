use crate::{Pattern, Result, parse::Token};

pub(crate) fn parse_digest(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    // The new syntax only supports bare "digest" keyword for any digest
    Ok(Pattern::any_digest())
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    #[test]
    fn test_parse_digest_any() {
        let mut lexer = Token::lexer("digest");
        // Consume the digest token first
        assert_eq!(lexer.next(), Some(Ok(Token::Digest)));
        let result = parse_digest(&mut lexer).unwrap();
        assert_eq!(result, Pattern::any_digest());
        assert_eq!(result.to_string(), "digest");
    }
}
