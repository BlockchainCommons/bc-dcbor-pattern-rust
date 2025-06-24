use regex::Regex;

use crate::{Error, Pattern, Result, parse::Token};

pub(crate) fn parse_text(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();

            // Check if this is a regex pattern by looking for leading /
            if src.trim_start().starts_with('/') {
                let (regex, consumed) = parse_text_regex(src)?;
                lexer.bump(consumed);
                match lexer.next() {
                    Some(Ok(Token::ParenClose)) => {
                        Ok(Pattern::text_regex(regex))
                    }
                    Some(Ok(t)) => Err(Error::UnexpectedToken(
                        Box::new(t),
                        lexer.span(),
                    )),
                    Some(Err(e)) => Err(e),
                    None => {
                        Err(Error::ExpectedCloseParen(lexer.span()))
                    }
                }
            } else {
                // Parse as string literal
                let (value, consumed) = parse_string_literal(src)?;
                lexer.bump(consumed);
                match lexer.next() {
                    Some(Ok(Token::ParenClose)) => Ok(Pattern::text(value)),
                    Some(Ok(t)) => Err(Error::UnexpectedToken(
                        Box::new(t),
                        lexer.span(),
                    )),
                    Some(Err(e)) => Err(e),
                    None => Err(Error::ExpectedCloseParen(lexer.span())),
                }
            }
        }
        _ => Ok(Pattern::any_text()),
    }
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
        let mut lexer = Token::lexer("TEXT");
        // Consume the TEXT token first
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        let result = parse_text(&mut lexer).unwrap();
        assert_eq!(result, Pattern::any_text());
        assert_eq!(result.to_string(), "TEXT");
    }

    #[test]
    fn test_parse_text_literal() {
        let mut lexer = Token::lexer(r#"TEXT("hello")"#);
        // Consume the TEXT token first
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        let result = parse_text(&mut lexer).unwrap();
        assert_eq!(result, Pattern::text("hello"));
        assert_eq!(result.to_string(), r#"TEXT("hello")"#);
    }

    #[test]
    fn test_parse_text_literal_with_spaces() {
        let mut lexer = Token::lexer(r#"TEXT ( "hello world" )"#);
        // Consume the TEXT token first
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        let result = parse_text(&mut lexer).unwrap();
        assert_eq!(result, Pattern::text("hello world"));
        assert_eq!(result.to_string(), r#"TEXT("hello world")"#);
    }

    #[test]
    fn test_parse_text_literal_with_escapes() {
        let mut lexer = Token::lexer(r#"TEXT("say \"hello\"")"#);
        // Consume the TEXT token first
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        let result = parse_text(&mut lexer).unwrap();
        assert_eq!(result, Pattern::text(r#"say "hello""#));
        assert_eq!(result.to_string(), r#"TEXT("say \"hello\"")"#);
    }

    #[test]
    fn test_parse_text_regex() {
        let mut lexer = Token::lexer(r"TEXT(/h.*o/)");
        // Consume the TEXT token first
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        let result = parse_text(&mut lexer).unwrap();
        let regex = regex::Regex::new("h.*o").unwrap();
        assert_eq!(result, Pattern::text_regex(regex));
        assert_eq!(result.to_string(), "TEXT(/h.*o/)");
    }

    #[test]
    fn test_parse_text_regex_with_spaces() {
        let mut lexer = Token::lexer(r"TEXT( /^\d+$/ )");
        // Consume the TEXT token first
        assert_eq!(lexer.next(), Some(Ok(Token::Text)));
        let result = parse_text(&mut lexer).unwrap();
        let regex = regex::Regex::new(r"^\d+$").unwrap();
        assert_eq!(result, Pattern::text_regex(regex));
        assert_eq!(result.to_string(), r"TEXT(/^\d+$/)");
    }

    #[test]
    fn test_parse_string_literal() {
        let (value, consumed) = parse_string_literal(r#""hello""#).unwrap();
        assert_eq!(value, "hello");
        assert_eq!(consumed, 7);

        let (value, consumed) = parse_string_literal(r#"  "hello world"  "#).unwrap();
        assert_eq!(value, "hello world");
        assert_eq!(consumed, 17);

        let (value, consumed) = parse_string_literal(r#""say \"hello\"""#).unwrap();
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
