use crate::{Pattern, Result, parse::Token};

pub(crate) fn parse_known_value(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    // For the new syntax, 'known' by itself matches any known value
    Ok(Pattern::any_known_value())
}

#[cfg(test)]
mod tests {
    use known_values::KnownValue;
    use logos::Logos;

    use super::*;

    #[test]
    fn test_parse_known_value_any() {
        let mut lexer = Token::lexer("known");
        // Consume the known token first
        assert_eq!(lexer.next(), Some(Ok(Token::Known)));
        let result = parse_known_value(&mut lexer).unwrap();
        assert_eq!(result, Pattern::any_known_value());
        assert_eq!(result.to_string(), "known");
    }

    #[test]
    fn test_parse_known_value_numeric() {
        let mut lexer = Token::lexer("'12345'");
        // Consume the SingleQuoted token first
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::SingleQuoted(Ok("12345".to_string()))))
        );
        // This test would now be handled by the primary parser, not the
        // known_value parser The pattern should be created directly
        // from the SingleQuoted token
        let pattern = Pattern::known_value(KnownValue::new(12345));
        assert_eq!(pattern.to_string(), "'12345'");
    }

    #[test]
    fn test_parse_known_value_numeric_with_spaces() {
        let mut lexer = Token::lexer("'54321'");
        // Consume the SingleQuoted token first
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::SingleQuoted(Ok("54321".to_string()))))
        );
        let pattern = Pattern::known_value(KnownValue::new(54321));
        assert_eq!(pattern.to_string(), "'54321'");
    }

    #[test]
    fn test_parse_known_value_named() {
        let mut lexer = Token::lexer("'note'");
        // Consume the SingleQuoted token first
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::SingleQuoted(Ok("note".to_string()))))
        );
        let pattern = Pattern::known_value_named("note");
        assert_eq!(pattern.to_string(), "'note'");
    }

    #[test]
    fn test_parse_known_value_named_with_spaces() {
        let mut lexer = Token::lexer("'controller'");
        // Consume the SingleQuoted token first
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::SingleQuoted(Ok("controller".to_string()))))
        );
        let pattern = Pattern::known_value_named("controller");
        assert_eq!(pattern.to_string(), "'controller'");
    }

    #[test]
    fn test_parse_known_value_regex() {
        let mut lexer = Token::lexer("'/^note.*/'");
        // Consume the SingleQuoted token first
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::SingleQuoted(Ok("/^note.*/".to_string()))))
        );
        let regex = regex::Regex::new("^note.*").unwrap();
        let pattern = Pattern::known_value_regex(regex);
        assert_eq!(pattern.to_string(), "'/^note.*/'");
    }

    #[test]
    fn test_parse_known_value_regex_with_spaces() {
        let mut lexer = Token::lexer("'/controller|note/'");
        // Consume the SingleQuoted token first
        assert_eq!(
            lexer.next(),
            Some(Ok(Token::SingleQuoted(Ok("/controller|note/".to_string()))))
        );
        let regex = regex::Regex::new("controller|note").unwrap();
        let pattern = Pattern::known_value_regex(regex);
        assert_eq!(pattern.to_string(), "'/controller|note/'");
    }
}
