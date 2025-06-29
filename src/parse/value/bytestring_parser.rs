use crate::{Error, Pattern, Result, parse::Token};

pub(crate) fn parse_bytestring(
    _lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    // The bstr token only matches any byte string
    Ok(Pattern::any_byte_string())
}

pub(crate) fn parse_hex_string_token(
    token: Result<Vec<u8>>,
) -> Result<Pattern> {
    match token {
        Ok(bytes) => Ok(Pattern::byte_string(bytes)),
        Err(e) => Err(e),
    }
}

pub(crate) fn parse_hex_regex_token(token: Result<String>) -> Result<Pattern> {
    match token {
        Ok(regex_str) => {
            let regex = regex::bytes::Regex::new(&regex_str)
                .map_err(|_| Error::InvalidRegex(0..0))?;
            Ok(Pattern::byte_string_regex(regex))
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::*;

    #[test]
    fn test_parse_bytestring_any() {
        let mut lexer = Token::lexer("bstr");
        // Consume the bstr token first
        assert_eq!(lexer.next(), Some(Ok(Token::ByteString)));
        let result = parse_bytestring(&mut lexer).unwrap();
        assert_eq!(result, Pattern::any_byte_string());
        assert_eq!(result.to_string(), "bstr");
    }
}
