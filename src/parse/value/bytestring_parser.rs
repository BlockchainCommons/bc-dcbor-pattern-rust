use crate::{Error, Pattern, Result, parse::Token};

pub(crate) fn parse_bytestring(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();

            // Check if this is a regex pattern by looking for leading /
            if src.trim_start().starts_with('/') {
                let (regex, consumed) = parse_binary_regex(src)?;
                lexer.bump(consumed);
                match lexer.next() {
                    Some(Ok(Token::ParenClose)) => {
                        Ok(Pattern::byte_string_regex(regex))
                    }
                    Some(Ok(t)) => {
                        Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                    }
                    Some(Err(e)) => Err(e),
                    None => Err(Error::ExpectedCloseParen(lexer.span())),
                }
            } else {
                // Parse as hex string literal
                let (bytes, consumed) = parse_hex_string(src)?;
                lexer.bump(consumed);
                match lexer.next() {
                    Some(Ok(Token::ParenClose)) => {
                        Ok(Pattern::byte_string(bytes))
                    }
                    Some(Ok(t)) => {
                        Err(Error::UnexpectedToken(Box::new(t), lexer.span()))
                    }
                    Some(Err(e)) => Err(e),
                    None => Err(Error::ExpectedCloseParen(lexer.span())),
                }
            }
        }
        _ => Ok(Pattern::any_byte_string()),
    }
}

/// Parse a hex string literal enclosed in h'...' syntax.
fn parse_hex_string(src: &str) -> Result<(Vec<u8>, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);
    if !src[pos..].starts_with("h'") {
        return Err(Error::InvalidHexString(pos..pos));
    }
    pos += 2;
    let start = pos;
    while pos < src.len() {
        let b = src.as_bytes()[pos];
        if b == b'\'' {
            let inner = &src[start..pos];
            let bytes = hex::decode(inner)
                .map_err(|_| Error::InvalidHexString(pos..pos))?;
            pos += 1;
            skip_ws(src, &mut pos);
            return Ok((bytes, pos));
        }
        if !(b as char).is_ascii_hexdigit() {
            return Err(Error::InvalidHexString(pos..pos));
        }
        pos += 1;
    }
    Err(Error::InvalidHexString(pos..pos))
}

/// Parse a binary regex from the input string starting with /
fn parse_binary_regex(src: &str) -> Result<(regex::bytes::Regex, usize)> {
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
            let regex = regex::bytes::Regex::new(inner)
                .map_err(|_| Error::InvalidRegex(pos..pos))?;
            skip_ws(src, &mut pos);
            return Ok((regex, pos));
        }
    }
    Err(Error::UnterminatedRegex(pos..pos))
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
    fn test_parse_bytestring_any() {
        let mut lexer = Token::lexer("BSTR");
        // Consume the BSTR token first
        assert_eq!(lexer.next(), Some(Ok(Token::ByteString)));
        let result = parse_bytestring(&mut lexer).unwrap();
        assert_eq!(result, Pattern::any_byte_string());
        assert_eq!(result.to_string(), "BSTR");
    }

    #[test]
    fn test_parse_bytestring_hex() {
        let mut lexer = Token::lexer(r#"BSTR(h'010203')"#);
        // Consume the BSTR token first
        assert_eq!(lexer.next(), Some(Ok(Token::ByteString)));
        let result = parse_bytestring(&mut lexer).unwrap();
        assert_eq!(result, Pattern::byte_string(vec![1, 2, 3]));
        assert_eq!(result.to_string(), r#"BSTR(h'010203')"#);
    }

    #[test]
    fn test_parse_bytestring_hex_with_spaces() {
        let mut lexer = Token::lexer(r#"BSTR( h'deadbeef' )"#);
        // Consume the BSTR token first
        assert_eq!(lexer.next(), Some(Ok(Token::ByteString)));
        let result = parse_bytestring(&mut lexer).unwrap();
        assert_eq!(result, Pattern::byte_string(vec![0xde, 0xad, 0xbe, 0xef]));
        assert_eq!(result.to_string(), r#"BSTR(h'deadbeef')"#);
    }

    #[test]
    fn test_parse_bytestring_regex() {
        let mut lexer = Token::lexer(r"BSTR(/^\d+$/)");
        // Consume the BSTR token first
        assert_eq!(lexer.next(), Some(Ok(Token::ByteString)));
        let result = parse_bytestring(&mut lexer).unwrap();
        let regex = regex::bytes::Regex::new(r"^\d+$").unwrap();
        assert_eq!(result, Pattern::byte_string_regex(regex));
        assert_eq!(result.to_string(), r"BSTR(/^\d+$/)");
    }

    #[test]
    fn test_parse_bytestring_regex_with_spaces() {
        let mut lexer = Token::lexer(r"BSTR( /^[0-9a-f]+$/ )");
        // Consume the BSTR token first
        assert_eq!(lexer.next(), Some(Ok(Token::ByteString)));
        let result = parse_bytestring(&mut lexer).unwrap();
        let regex = regex::bytes::Regex::new(r"^[0-9a-f]+$").unwrap();
        assert_eq!(result, Pattern::byte_string_regex(regex));
        assert_eq!(result.to_string(), r"BSTR(/^[0-9a-f]+$/)");
    }

    #[test]
    fn test_parse_hex_string() {
        let (bytes, consumed) = parse_hex_string("h'010203'").unwrap();
        assert_eq!(bytes, vec![1, 2, 3]);
        assert_eq!(consumed, 9);

        let (bytes, consumed) = parse_hex_string("  h'deadbeef'  ").unwrap();
        assert_eq!(bytes, vec![0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(consumed, 15);

        let (bytes, consumed) = parse_hex_string("h''").unwrap();
        assert_eq!(bytes, vec![]);
        assert_eq!(consumed, 3);
    }

    #[test]
    fn test_parse_hex_string_errors() {
        assert!(parse_hex_string("hello").is_err());
        assert!(parse_hex_string("h'zz'").is_err());
        assert!(parse_hex_string("h'123").is_err());
        assert!(parse_hex_string("").is_err());
    }

    #[test]
    fn test_parse_binary_regex() {
        let (regex, consumed) = parse_binary_regex(r"/^\d+$/").unwrap();
        assert_eq!(regex.as_str(), r"^\d+$");
        assert_eq!(consumed, 7);

        let (regex, consumed) =
            parse_binary_regex(r"  /^[a-f0-9]+$/  ").unwrap();
        assert_eq!(regex.as_str(), r"^[a-f0-9]+$");
        assert_eq!(consumed, 17);
    }

    #[test]
    fn test_parse_binary_regex_errors() {
        assert!(parse_binary_regex("hello").is_err());
        assert!(parse_binary_regex("/unterminated").is_err());
        assert!(parse_binary_regex("").is_err());
    }
}
