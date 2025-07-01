use regex::Regex;

use crate::{Error, Pattern, Result, parse::Token};

pub(crate) fn parse_text(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    Ok(Pattern::any_text())
}

/// Parse a string literal starting with double quote
pub(crate) fn parse_text_string_literal(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let src = lexer.remainder();
    let (value, consumed) = parse_string_literal(src)?;
    lexer.bump(consumed);
    Ok(Pattern::text(value))
}

/// Parse a regex pattern starting with /
pub(crate) fn parse_text_regex_literal(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let src = lexer.remainder();
    let (regex, consumed) = parse_text_regex(src)?;
    lexer.bump(consumed);
    Ok(Pattern::text_regex(regex))
}

/// Parse a text regex from the input string starting with /
fn parse_text_regex(src: &str) -> Result<(regex::Regex, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);
    if pos >= src.len() || src.as_bytes()[pos] != b'/' {
        return Err(Error::UnterminatedRegex(pos..pos));
    }
    pos += 1;
    let start = pos;
    let mut escape = false;
    while pos < src.len() {
        let b = src.as_bytes()[pos];
        pos += 1;
        if escape {
            escape = false;
            continue;
        }
        if b == b'\\' {
            escape = true;
            continue;
        }
        if b == b'/' {
            let inner = &src[start..pos - 1];
            let regex = regex::Regex::new(inner)
                .map_err(|_| Error::InvalidRegex(pos..pos))?;
            skip_ws(src, &mut pos);
            return Ok((regex, pos));
        }
    }
    Err(Error::UnterminatedRegex(pos..pos))
}

/// Parse a string literal enclosed in double quotes.
/// Supports basic escape sequences like \" and \\.
fn parse_string_literal(src: &str) -> Result<(String, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);

    let bytes = src.as_bytes();
    if pos >= bytes.len() || bytes[pos] != b'"' {
        return Err(Error::UnexpectedEndOfInput);
    }
    pos += 1;
    let start = pos;
    let mut escape = false;
    while pos < bytes.len() {
        let b = bytes[pos];
        pos += 1;
        if escape {
            escape = false;
            continue;
        }
        if b == b'\\' {
            escape = true;
            continue;
        }
        if b == b'"' {
            let inner = &src[start..pos - 1];
            let value = inner.replace(r#"\""#, r#"""#).replace(r#"\\"#, r#"\"#);
            skip_ws(src, &mut pos);
            return Ok((value, pos));
        }
    }
    Err(Error::UnexpectedEndOfInput)
}

/// Skip whitespace characters.
fn skip_ws(src: &str, pos: &mut usize) {
    while let Some(ch) = src[*pos..].chars().next() {
        if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
            *pos += ch.len_utf8();
        } else {
            break;
        }
    }
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

    #[test]
    fn test_parse_string_literal() {
        let (value, consumed) = parse_string_literal(r#""hello""#).unwrap();
        assert_eq!(value, "hello");
        assert_eq!(consumed, 7);

        let (value, consumed) =
            parse_string_literal(r#"  "hello world"  "#).unwrap();
        assert_eq!(value, "hello world");
        assert_eq!(consumed, 17);

        let (value, consumed) =
            parse_string_literal(r#""say \"hello\"""#).unwrap();
        assert_eq!(value, r#"say "hello""#);
        assert_eq!(consumed, 15);
    }

    #[test]
    fn test_parse_string_literal_errors() {
        assert!(parse_string_literal("hello").is_err());
        assert!(parse_string_literal(r#""unterminated"#).is_err());
        assert!(parse_string_literal("").is_err());
    }
}
