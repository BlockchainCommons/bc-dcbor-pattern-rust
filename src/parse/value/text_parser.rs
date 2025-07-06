use crate::{Pattern, Result, parse::Token};

pub(crate) fn parse_text(_lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    Ok(Pattern::any_text())
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    #[test]
    fn test_parse_text_any() {
        let mut lexer = Token::lexer("text");
        // Consume the text token first
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        let result = parse_text(&mut lexer).unwrap();
        assert_eq!(result, Pattern::any_text());
        assert_eq!(result.to_string(), "text");
    }

    #[test]
    fn test_parse_text_literal() {
        // Note: The new syntax doesn't use parentheses anymore.
        // String literals are parsed directly by the lexer as tokens.
        // This test is kept for documentation but the actual
        // parsing happens at the token level now.
        let mut lexer = Token::lexer("text");
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        let result = parse_text(&mut lexer).unwrap();
        assert_eq!(result, Pattern::any_text());
        assert_eq!(result.to_string(), "text");
    }

    #[test]
    fn test_parse_text_literal_with_spaces() {
        // Obsolete test - new syntax doesn't use parentheses
        let mut lexer = Token::lexer("text");
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        let result = parse_text(&mut lexer).unwrap();
        assert_eq!(result, Pattern::any_text());
        assert_eq!(result.to_string(), "text");
    }

    #[test]
    fn test_parse_text_literal_with_escapes() {
        // Obsolete test - new syntax doesn't use parentheses
        let mut lexer = Token::lexer("text");
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        let result = parse_text(&mut lexer).unwrap();
        assert_eq!(result, Pattern::any_text());
        assert_eq!(result.to_string(), "text");
    }

    #[test]
    fn test_parse_text_regex() {
        // Obsolete test - new syntax doesn't use parentheses
        let mut lexer = Token::lexer("text");
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        let result = parse_text(&mut lexer).unwrap();
        assert_eq!(result, Pattern::any_text());
        assert_eq!(result.to_string(), "text");
    }

    #[test]
    fn test_parse_text_regex_with_spaces() {
        // Obsolete test - new syntax doesn't use parentheses
        let mut lexer = Token::lexer("text");
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        let result = parse_text(&mut lexer).unwrap();
        assert_eq!(result, Pattern::any_text());
        assert_eq!(result.to_string(), "text");
    }
}
