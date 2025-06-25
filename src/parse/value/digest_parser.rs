use bc_components::Digest;
use bc_ur::{URDecodable, UREncodable};

use crate::{Error, Pattern, Result, parse::Token};

pub(crate) fn parse_digest(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();
            let (pattern, consumed) = parse_digest_inner(src)?;
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
        _ => Ok(Pattern::any_digest()),
    }
}

fn parse_digest_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);

    // Check if it's a UR string
    if src[pos..].starts_with("ur:") {
        let start = pos;
        while let Some(ch) = src[pos..].chars().next() {
            if ch == ')' || ch.is_whitespace() {
                break;
            }
            pos += ch.len_utf8();
        }
        let ur_str = &src[start..pos];
        let digest = Digest::from_ur_string(ur_str)
            .map_err(|_| Error::InvalidUr(ur_str.to_string(), start..pos))?;
        skip_ws(src, &mut pos);
        return Ok((Pattern::digest(digest), pos));
    }

    // Parse hex prefix
    let start = pos;
    while let Some(ch) = src[pos..].chars().next() {
        if ch.is_ascii_hexdigit() {
            pos += ch.len_utf8();
        } else {
            break;
        }
    }
    if start == pos {
        return Err(Error::InvalidHexString(pos..pos));
    }
    let hex_str = &src[start..pos];
    if hex_str.len() % 2 != 0 {
        return Err(Error::InvalidHexString(start..pos));
    }
    let bytes = hex::decode(hex_str)
        .map_err(|_| Error::InvalidHexString(start..pos))?;
    if bytes.len() > Digest::DIGEST_SIZE {
        return Err(Error::InvalidHexString(start..pos));
    }
    skip_ws(src, &mut pos);
    Ok((Pattern::digest_prefix(bytes), pos))
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
    fn test_parse_digest_any() {
        let mut lexer = Token::lexer("DIGEST");
        // Consume the DIGEST token first
        assert_eq!(lexer.next(), Some(Ok(Token::Digest)));
        let result = parse_digest(&mut lexer).unwrap();
        assert_eq!(result, Pattern::any_digest());
        assert_eq!(result.to_string(), "DIGEST");
    }

    #[test]
    fn test_parse_digest_hex_prefix() {
        let mut lexer = Token::lexer("DIGEST(a1b2c3)");
        // Consume the DIGEST token first
        assert_eq!(lexer.next(), Some(Ok(Token::Digest)));
        let result = parse_digest(&mut lexer).unwrap();
        let expected_bytes = hex::decode("a1b2c3").unwrap();
        assert_eq!(result, Pattern::digest_prefix(expected_bytes));
        assert_eq!(result.to_string(), "DIGEST(a1b2c3)");
    }

    #[test]
    fn test_parse_digest_hex_prefix_with_spaces() {
        let mut lexer = Token::lexer("DIGEST( a1b2c3 )");
        // Consume the DIGEST token first
        assert_eq!(lexer.next(), Some(Ok(Token::Digest)));
        let result = parse_digest(&mut lexer).unwrap();
        let expected_bytes = hex::decode("a1b2c3").unwrap();
        assert_eq!(result, Pattern::digest_prefix(expected_bytes));
        assert_eq!(result.to_string(), "DIGEST(a1b2c3)");
    }

    #[test]
    fn test_parse_digest_full_hex() {
        let full_digest_hex =
            "4d303dac9eed63573f6190e9c4191be619e03a7b3c21e9bb3d27ac1a55971e6b";
        let input = format!("DIGEST({})", full_digest_hex);
        let mut lexer = Token::lexer(&input);
        // Consume the DIGEST token first
        assert_eq!(lexer.next(), Some(Ok(Token::Digest)));
        let result = parse_digest(&mut lexer).unwrap();
        let expected_bytes = hex::decode(full_digest_hex).unwrap();
        assert_eq!(result, Pattern::digest_prefix(expected_bytes));
        assert_eq!(result.to_string(), format!("DIGEST({})", full_digest_hex));
    }

    #[test]
    fn test_parse_digest_ur_string() {
        bc_components::register_tags();
        let digest = Digest::from_image(b"hello world");
        let ur_string = digest.ur_string();
        let input = format!("DIGEST({})", ur_string);
        let mut lexer = Token::lexer(&input);
        // Consume the DIGEST token first
        assert_eq!(lexer.next(), Some(Ok(Token::Digest)));
        let result = parse_digest(&mut lexer).unwrap();
        assert_eq!(result, Pattern::digest(digest.clone()));
        assert_eq!(result.to_string(), format!("DIGEST({})", digest));
    }

    #[test]
    fn test_parse_digest_errors() {
        // Test invalid hex (odd number of characters)
        let mut lexer = Token::lexer("DIGEST(a1b2c)");
        assert_eq!(lexer.next(), Some(Ok(Token::Digest)));
        assert!(parse_digest(&mut lexer).is_err());

        // Test empty parentheses
        let mut lexer = Token::lexer("DIGEST()");
        assert_eq!(lexer.next(), Some(Ok(Token::Digest)));
        assert!(parse_digest(&mut lexer).is_err());

        // Test hex too long (more than 32 bytes)
        let too_long_hex = "a".repeat(66); // 33 bytes
        let input = format!("DIGEST({})", too_long_hex);
        let mut lexer = Token::lexer(&input);
        assert_eq!(lexer.next(), Some(Ok(Token::Digest)));
        assert!(parse_digest(&mut lexer).is_err());

        // Test invalid hex characters
        let mut lexer = Token::lexer("DIGEST(g1h2)");
        assert_eq!(lexer.next(), Some(Ok(Token::Digest)));
        assert!(parse_digest(&mut lexer).is_err());

        // Test invalid UR string
        let mut lexer = Token::lexer("DIGEST(ur:invalid/baddata)");
        assert_eq!(lexer.next(), Some(Ok(Token::Digest)));
        assert!(parse_digest(&mut lexer).is_err());
    }

    #[test]
    fn test_parse_hex_string() {
        let (bytes, consumed) = parse_digest_inner("a1b2c3").unwrap();
        assert_eq!(
            bytes,
            Pattern::digest_prefix(hex::decode("a1b2c3").unwrap())
        );
        assert_eq!(consumed, 6);

        let (bytes, consumed) = parse_digest_inner("  1234  ").unwrap();
        assert_eq!(bytes, Pattern::digest_prefix(hex::decode("1234").unwrap()));
        assert_eq!(consumed, 8);
    }

    #[test]
    fn test_parse_hex_string_errors() {
        assert!(parse_digest_inner("").is_err());
        assert!(parse_digest_inner("a1b2c").is_err()); // Odd length
        assert!(parse_digest_inner("xyz").is_err()); // Invalid hex
    }
}
