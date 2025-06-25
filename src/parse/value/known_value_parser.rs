use known_values::KnownValue;

use crate::{Error, Pattern, Result, parse::Token};

pub(crate) fn parse_known_value(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();
            let (pattern, consumed) = parse_known_value_inner(src)?;
            lexer.bump(consumed);
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => Ok(pattern),
                Some(Ok(t)) => {
                    Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => Ok(Pattern::any_known_value()),
    }
}

fn parse_known_value_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);
    if src[pos..].starts_with('/') {
        let (regex, used) = parse_text_regex(&src[pos..])?;
        pos += used;
        return Ok((Pattern::known_value_regex(regex), pos));
    }

    let (inner, used) = parse_single_quoted(&src[pos..])?;
    pos += used;
    if let Ok(value) = inner.parse::<u64>() {
        Ok((Pattern::known_value(KnownValue::new(value)), pos))
    } else {
        Ok((Pattern::known_value_named(inner), pos))
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

/// Parse a single-quoted string literal.
fn parse_single_quoted(src: &str) -> Result<(String, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);
    if pos >= src.len() || src.as_bytes()[pos] != b'\'' {
        return Err(Error::UnexpectedEndOfInput);
    }
    pos += 1;
    let start = pos;
    while pos < src.len() {
        if src.as_bytes()[pos] == b'\'' {
            let value = src[start..pos].to_string();
            pos += 1;
            skip_ws(src, &mut pos);
            return Ok((value, pos));
        }
        pos += 1;
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
    fn test_parse_known_value_any() {
        let mut lexer = Token::lexer("KNOWN");
        // Consume the KNOWN token first
        assert_eq!(lexer.next(), Some(Ok(Token::Known)));
        let result = parse_known_value(&mut lexer).unwrap();
        assert_eq!(result, Pattern::any_known_value());
        assert_eq!(result.to_string(), "KNOWN");
    }

    #[test]
    fn test_parse_known_value_numeric() {
        let mut lexer = Token::lexer("KNOWN('12345')");
        // Consume the KNOWN token first
        assert_eq!(lexer.next(), Some(Ok(Token::Known)));
        let result = parse_known_value(&mut lexer).unwrap();
        assert_eq!(result, Pattern::known_value(KnownValue::new(12345)));
        assert_eq!(result.to_string(), "KNOWN('12345')");
    }

    #[test]
    fn test_parse_known_value_numeric_with_spaces() {
        let mut lexer = Token::lexer("KNOWN( '54321' )");
        // Consume the KNOWN token first
        assert_eq!(lexer.next(), Some(Ok(Token::Known)));
        let result = parse_known_value(&mut lexer).unwrap();
        assert_eq!(result, Pattern::known_value(KnownValue::new(54321)));
        assert_eq!(result.to_string(), "KNOWN('54321')");
    }

    #[test]
    fn test_parse_known_value_named() {
        let mut lexer = Token::lexer("KNOWN('note')");
        // Consume the KNOWN token first
        assert_eq!(lexer.next(), Some(Ok(Token::Known)));
        let result = parse_known_value(&mut lexer).unwrap();
        assert_eq!(result, Pattern::known_value_named("note"));
        assert_eq!(result.to_string(), "KNOWN('note')");
    }

    #[test]
    fn test_parse_known_value_named_with_spaces() {
        let mut lexer = Token::lexer("KNOWN( 'controller' )");
        // Consume the KNOWN token first
        assert_eq!(lexer.next(), Some(Ok(Token::Known)));
        let result = parse_known_value(&mut lexer).unwrap();
        assert_eq!(result, Pattern::known_value_named("controller"));
        assert_eq!(result.to_string(), "KNOWN('controller')");
    }

    #[test]
    fn test_parse_known_value_regex() {
        let mut lexer = Token::lexer("KNOWN(/^note.*/)");
        // Consume the KNOWN token first
        assert_eq!(lexer.next(), Some(Ok(Token::Known)));
        let result = parse_known_value(&mut lexer).unwrap();
        let regex = regex::Regex::new("^note.*").unwrap();
        assert_eq!(result, Pattern::known_value_regex(regex));
        assert_eq!(result.to_string(), "KNOWN(/^note.*/)");
    }

    #[test]
    fn test_parse_known_value_regex_with_spaces() {
        let mut lexer = Token::lexer("KNOWN( /controller|note/ )");
        // Consume the KNOWN token first
        assert_eq!(lexer.next(), Some(Ok(Token::Known)));
        let result = parse_known_value(&mut lexer).unwrap();
        let regex = regex::Regex::new("controller|note").unwrap();
        assert_eq!(result, Pattern::known_value_regex(regex));
        assert_eq!(result.to_string(), "KNOWN(/controller|note/)");
    }

    #[test]
    fn test_parse_single_quoted() {
        let (value, consumed) = parse_single_quoted("'hello'").unwrap();
        assert_eq!(value, "hello");
        assert_eq!(consumed, 7);

        let (value, consumed) =
            parse_single_quoted("  'hello world'  ").unwrap();
        assert_eq!(value, "hello world");
        assert_eq!(consumed, 17);

        let (value, consumed) = parse_single_quoted("'12345'").unwrap();
        assert_eq!(value, "12345");
        assert_eq!(consumed, 7);
    }

    #[test]
    fn test_parse_single_quoted_errors() {
        assert!(parse_single_quoted("hello").is_err());
        assert!(parse_single_quoted("'unterminated").is_err());
        assert!(parse_single_quoted("").is_err());
    }
}
