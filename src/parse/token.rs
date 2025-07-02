use dcbor::prelude::*;
use dcbor_parse::parse_dcbor_item_partial;
use logos::{Lexer, Logos};

use crate::{DigestPattern, Error, Quantifier, Reluctance, Result};

/// Tokens for the Gordian Envelope pattern syntax.
#[derive(Debug, Clone, Logos, PartialEq)]
#[rustfmt::skip]
#[logos(error = Error)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    #[token("&")]
    And,

    #[token("|")]
    Or,

    #[token("!")]
    Not,

    #[token("*")]
    RepeatZeroOrMore,

    #[token("*?")]
    RepeatZeroOrMoreLazy,

    #[token("*+")]
    RepeatZeroOrMorePossessive,

    #[token("+")]
    RepeatOneOrMore,

    #[token("+?")]
    RepeatOneOrMoreLazy,

    #[token("++")]
    RepeatOneOrMorePossessive,

    #[token("?")]
    RepeatZeroOrOne,

    #[token("??")]
    RepeatZeroOrOneLazy,

    #[token("?+")]
    RepeatZeroOrOnePossessive,

    // Structure Pattern Keywords
    #[token("tagged")]
    Tagged,

    // Value Pattern Keywords
    #[token("bool")]
    Bool,

    #[token("bstr")]
    ByteString,

    #[token("date")]
    Date,

    #[token("date'", parse_date_quoted)]
    DateQuoted(Result<crate::pattern::DatePattern>),

    #[token("known")]
    Known,

    #[token("null")]
    Null,

    #[token("number")]
    Number,

    #[token("text")]
    Text,

    #[token("digest")]
    Digest,

    #[token("digest'", parse_digest_quoted)]
    DigestQuoted(Result<DigestPattern>),

    // Meta Pattern Keywords
    #[token("search")]
    Search,

    // Special literals
    #[token("true")]
    BoolTrue,

    #[token("false")]
    BoolFalse,

    #[token("NaN")]
    NaN,

    #[token("Infinity")]
    Infinity,

    #[token("-Infinity")]
    NegInfinity,

    // Grouping and Range delimiters
    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token("{", parse_brace_open)]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token("...")]
    Ellipsis,

    #[token(">=")]
    GreaterThanOrEqual,

    #[token("<=")]
    LessThanOrEqual,

    #[token(">", priority = 1)]
    GreaterThan,

    #[token("<")]
    LessThan,

    /// Number literal parsed using dcbor-parse for consistency with dCBOR
    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", callback = parse_number)]
    NumberLiteral(Result<f64>),

    #[regex(r"@[a-zA-Z_][a-zA-Z0-9_]*", |lex|
        lex.slice()[1..].to_string()
    )]
    GroupName(String),

    #[token("\"", parse_string)]
    StringLiteral(Result<String>),

    #[token("'", parse_single_quoted)]
    SingleQuoted(Result<String>),

    #[token("/", parse_regex)]
    Regex(Result<String>),

    #[token("h'", parse_hex_string)]
    HexString(Result<Vec<u8>>),

    #[token("h'/", parse_hex_regex)]
    HexRegex(Result<String>),

    Range(Result<Quantifier>),
}

/// Callback to parse numbers using dcbor-parse for consistency with dCBOR
fn parse_number(lex: &mut Lexer<Token>) -> Result<f64> {
    let number_str = lex.slice();
    match parse_dcbor_item_partial(number_str) {
        Ok((cbor, _)) => match f64::try_from_cbor(&cbor) {
            Ok(value) => Ok(value),
            Err(_) => Err(Error::InvalidNumberFormat(lex.span())),
        },
        Err(_) => Err(Error::InvalidNumberFormat(lex.span())),
    }
}

/// Callback used by the `Regex` variant above.
fn parse_regex(lex: &mut Lexer<Token>) -> Result<String> {
    let src = lex.remainder(); // everything after the first '/'
    let mut escape = false;

    for (i, ch) in src.char_indices() {
        match (ch, escape) {
            ('\\', false) => escape = true, // start of an escape
            ('/', false) => {
                // Found the closing delimiter ------------------
                lex.bump(i + 1); // +1 to also eat the '/'
                let content = src[..i].to_owned();
                match regex::Regex::new(&content) {
                    Ok(_) => return Ok(content),
                    Err(_) => return Err(Error::InvalidRegex(lex.span())),
                }
            }
            _ => escape = false, // any other char ends an escape
        }
    }

    // Unterminated literal – treat as lexing error
    Err(Error::UnterminatedRegex(lex.span()))
}

/// Callback used by the `StringLiteral` variant above.
fn parse_string(lex: &mut Lexer<Token>) -> Result<String> {
    let src = lex.remainder(); // everything after the first '"'
    let mut escape = false;
    let mut result = String::new();

    for (i, ch) in src.char_indices() {
        match (ch, escape) {
            ('\\', false) => escape = true, // start of an escape
            ('"', false) => {
                // Found the closing delimiter
                lex.bump(i + 1); // +1 to also eat the '"'
                return Ok(result);
            }
            (c, true) => {
                // Handle escape sequences
                match c {
                    '"' => result.push('"'),
                    '\\' => result.push('\\'),
                    'n' => result.push('\n'),
                    'r' => result.push('\r'),
                    't' => result.push('\t'),
                    _ => {
                        result.push('\\');
                        result.push(c);
                    }
                }
                escape = false;
            }
            (c, false) => {
                result.push(c);
                escape = false;
            }
        }
    }

    // Unterminated literal – treat as lexing error
    Err(Error::UnterminatedString(lex.span()))
}

/// Callback used by the `HexString` variant above.
fn parse_hex_string(lex: &mut Lexer<Token>) -> Result<Vec<u8>> {
    let src = lex.remainder(); // everything after the first h'

    // Parse as hex string h'...'
    for (i, ch) in src.char_indices() {
        match ch {
            '\'' => {
                // Found the closing delimiter
                let hex_content = &src[..i];
                match hex::decode(hex_content) {
                    Ok(bytes) => {
                        lex.bump(i + 1); // +1 to also eat the '\''
                        return Ok(bytes);
                    }
                    Err(_) => return Err(Error::InvalidHexString(lex.span())),
                }
            }
            c if c.is_ascii_hexdigit() => {
                // Valid hex character, continue
            }
            _ => {
                // Invalid character in hex string
                return Err(Error::InvalidHexString(lex.span()));
            }
        }
    }

    // Unterminated literal – treat as lexing error
    Err(Error::UnterminatedHexString(lex.span()))
}

/// Callback used by the `HexRegex` variant above.
fn parse_hex_regex(lex: &mut Lexer<Token>) -> Result<String> {
    let src = lex.remainder(); // everything after the first h'/
    let mut escape = false;

    for (i, ch) in src.char_indices() {
        match (ch, escape) {
            ('\\', false) => escape = true, // start of an escape
            ('/', false) => {
                // Look for the closing '
                let remainder = &src[i + 1..];
                if remainder.starts_with('\'') {
                    // Found the closing h'/.../'
                    lex.bump(i + 2); // +2 to eat both '/' and '\''
                    let content = src[..i].to_owned();
                    match regex::bytes::Regex::new(&content) {
                        Ok(_) => return Ok(content),
                        Err(_) => return Err(Error::InvalidRegex(lex.span())),
                    }
                }
                // Not the end, continue
                escape = false;
            }
            _ => escape = false, // any other char ends an escape
        }
    }

    // Unterminated literal – treat as lexing error
    Err(Error::UnterminatedRegex(lex.span()))
}

/// Callback used by the `DigestQuoted` variant above.
fn parse_digest_quoted(lex: &mut Lexer<Token>) -> Result<DigestPattern> {
    use bc_components::Digest;
    use bc_ur::{URDecodable, UREncodable};

    let src = lex.remainder(); // everything after "digest'"

    // Find the closing quote
    for (i, ch) in src.char_indices() {
        if ch == '\'' {
            let content = &src[..i];
            lex.bump(i + 1); // +1 to eat the closing quote

            // Check for empty content
            if content.is_empty() {
                return Err(Error::InvalidDigestPattern(
                    "empty content".to_string(),
                    lex.span(),
                ));
            }

            // Check if it's a UR string
            if content.starts_with("ur:") {
                match Digest::from_ur_string(content) {
                    Ok(digest) => return Ok(DigestPattern::digest(digest)),
                    Err(_) => {
                        return Err(Error::InvalidUr(
                            content.to_string(),
                            lex.span(),
                        ));
                    }
                }
            }

            // Check if it's a regex pattern /.../
            if content.starts_with('/')
                && content.ends_with('/')
                && content.len() > 2
            {
                let regex_content = &content[1..content.len() - 1];
                match regex::bytes::Regex::new(regex_content) {
                    Ok(regex) => return Ok(DigestPattern::binary_regex(regex)),
                    Err(_) => return Err(Error::InvalidRegex(lex.span())),
                }
            }

            // Try to parse as hex
            if content.chars().all(|c| c.is_ascii_hexdigit()) {
                if content.len() % 2 == 0 {
                    match hex::decode(content) {
                        Ok(bytes) => {
                            if bytes.len() <= Digest::DIGEST_SIZE {
                                return Ok(DigestPattern::prefix(bytes));
                            } else {
                                return Err(Error::InvalidHexString(
                                    lex.span(),
                                ));
                            }
                        }
                        Err(_) => {
                            return Err(Error::InvalidHexString(lex.span()));
                        }
                    }
                } else {
                    return Err(Error::InvalidHexString(lex.span()));
                }
            }

            // If it's not UR, regex, or hex, it's an error
            return Err(Error::InvalidDigestPattern(
                content.to_string(),
                lex.span(),
            ));
        }
    }

    // Unterminated literal
    Err(Error::UnterminatedDigestQuoted(lex.span()))
}

/// Callback used by the `DateQuoted` variant above.
fn parse_date_quoted(
    lex: &mut Lexer<Token>,
) -> Result<crate::pattern::DatePattern> {
    use dcbor_parse::parse_dcbor_item;

    let src = lex.remainder(); // everything after "date'"

    // Find the closing quote
    for (i, ch) in src.char_indices() {
        if ch == '\'' {
            let content = &src[..i];
            lex.bump(i + 1); // +1 to eat the closing quote

            // Check for empty content
            if content.is_empty() {
                return Err(Error::InvalidDateFormat(lex.span()));
            }

            // Check if it's a regex pattern /.../
            if content.starts_with('/')
                && content.ends_with('/')
                && content.len() > 2
            {
                let regex_content = &content[1..content.len() - 1];
                match regex::Regex::new(regex_content) {
                    Ok(regex) => {
                        return Ok(crate::pattern::DatePattern::regex(regex));
                    }
                    Err(_) => return Err(Error::InvalidRegex(lex.span())),
                }
            }

            // Check for range patterns
            if content.contains("...") {
                if let Some(iso_str) = content.strip_prefix("...") {
                    // Latest pattern: "...iso-8601"
                    match parse_dcbor_item(iso_str) {
                        Ok(cbor) => match Date::try_from(cbor) {
                            Ok(date) => {
                                return Ok(
                                    crate::pattern::DatePattern::latest(date),
                                );
                            }
                            Err(_) => {
                                return Err(Error::InvalidDateFormat(
                                    lex.span(),
                                ));
                            }
                        },
                        Err(_) => {
                            return Err(Error::InvalidDateFormat(lex.span()));
                        }
                    }
                } else if let Some(iso_str) = content.strip_suffix("...") {
                    // Earliest pattern: "iso-8601..."
                    match parse_dcbor_item(iso_str) {
                        Ok(cbor) => match Date::try_from(cbor) {
                            Ok(date) => {
                                return Ok(
                                    crate::pattern::DatePattern::earliest(date),
                                );
                            }
                            Err(_) => {
                                return Err(Error::InvalidDateFormat(
                                    lex.span(),
                                ));
                            }
                        },
                        Err(_) => {
                            return Err(Error::InvalidDateFormat(lex.span()));
                        }
                    }
                } else {
                    // Range pattern: "iso-8601...iso-8601"
                    let parts: Vec<&str> = content.split("...").collect();
                    if parts.len() == 2 {
                        let start_date = match parse_dcbor_item(parts[0]) {
                            Ok(cbor) => match Date::try_from(cbor) {
                                Ok(date) => date,
                                Err(_) => {
                                    return Err(Error::InvalidDateFormat(
                                        lex.span(),
                                    ));
                                }
                            },
                            Err(_) => {
                                return Err(Error::InvalidDateFormat(
                                    lex.span(),
                                ));
                            }
                        };
                        let end_date = match parse_dcbor_item(parts[1]) {
                            Ok(cbor) => match Date::try_from(cbor) {
                                Ok(date) => date,
                                Err(_) => {
                                    return Err(Error::InvalidDateFormat(
                                        lex.span(),
                                    ));
                                }
                            },
                            Err(_) => {
                                return Err(Error::InvalidDateFormat(
                                    lex.span(),
                                ));
                            }
                        };
                        return Ok(crate::pattern::DatePattern::range(
                            start_date..=end_date,
                        ));
                    } else {
                        return Err(Error::InvalidDateFormat(lex.span()));
                    }
                }
            }

            // Try to parse as single ISO-8601 date
            match parse_dcbor_item(content) {
                Ok(cbor) => match Date::try_from(cbor) {
                    Ok(date) => {
                        return Ok(crate::pattern::DatePattern::value(date));
                    }
                    Err(_) => return Err(Error::InvalidDateFormat(lex.span())),
                },
                Err(_) => return Err(Error::InvalidDateFormat(lex.span())),
            }
        }
    }

    // Unterminated literal
    Err(Error::UnterminatedDateQuoted(lex.span()))
}

/// Callback to handle `{` token - determines if it's a Range or BraceOpen
fn parse_brace_open(lex: &mut Lexer<Token>) -> Token {
    let remainder = lex.remainder();

    // Skip whitespace and see if we have a digit pattern
    let mut chars = remainder.chars();
    let mut pos = 0;

    // Skip whitespace
    while let Some(ch) = chars.next() {
        if !matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
            // If the first non-whitespace character is a digit, we need to look ahead further
            // to determine if this is really a range pattern or a map key-value constraint
            if ch.is_ascii_digit() {
                // Look ahead to see if this looks like a range pattern
                if looks_like_range_pattern(&remainder[pos..]) {
                    let quantifier_result = parse_range_from_remainder(lex);
                    return Token::Range(quantifier_result);
                }
            }
            // Otherwise, it's just a regular BraceOpen
            break;
        }
        pos += ch.len_utf8();
    }

    Token::BraceOpen
}

/// Helper function to determine if the content after `{` looks like a range pattern
fn looks_like_range_pattern(content: &str) -> bool {
    let mut chars = content.chars();
    let mut has_digit = false;
    
    // Skip whitespace
    while let Some(ch) = chars.next() {
        if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
            continue;
        } else if ch.is_ascii_digit() {
            has_digit = true;
            break;
        } else {
            return false;
        }
    }
    
    if !has_digit {
        return false;
    }
    
    // Skip remaining digits
    while let Some(ch) = chars.next() {
        if ch.is_ascii_digit() {
            continue;
        } else {
            // After digits, we should see whitespace, comma, or closing brace for a range
            // If we see a colon, it's definitely a map key-value constraint
            if ch == ':' {
                return false;
            }
            // Skip whitespace
            if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
                // Continue to look for comma or closing brace
                while let Some(next_ch) = chars.next() {
                    if matches!(next_ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
                        continue;
                    } else if next_ch == ',' || next_ch == '}' {
                        return true;
                    } else if next_ch == ':' {
                        return false;
                    } else {
                        return false;
                    }
                }
            }
            // First non-digit, non-whitespace char should be comma or closing brace
            return ch == ',' || ch == '}';
        }
    }
    
    false
}

/// Helper function to parse a range pattern from the current position
fn parse_range_from_remainder(lex: &mut Lexer<Token>) -> Result<Quantifier> {
    let remainder = lex.remainder(); // everything after the '{'

    // Helper to skip whitespace inside the range specification
    fn skip_ws(s: &str, pos: &mut usize) {
        while let Some(ch) = s[*pos..].chars().next() {
            if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
                *pos += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    let mut pos = 0;

    // Skip initial whitespace
    skip_ws(remainder, &mut pos);

    // Parse the first number
    if !remainder[pos..]
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_digit())
    {
        return Err(Error::InvalidRange(lex.span()));
    }

    let start = pos;
    while let Some(ch) = remainder[pos..].chars().next() {
        if ch.is_ascii_digit() {
            pos += ch.len_utf8();
        } else {
            break;
        }
    }

    let min: usize = remainder[start..pos]
        .parse()
        .map_err(|_| Error::InvalidRange(lex.span()))?;

    skip_ws(remainder, &mut pos);

    // Parse optional comma and maximum value
    let max: Option<usize>;

    match remainder[pos..].chars().next() {
        Some(',') => {
            pos += 1;
            skip_ws(remainder, &mut pos);

            // If the next non-space char is '}', the range is open ended
            match remainder[pos..].chars().next() {
                Some('}') => {
                    pos += 1;
                    max = None;
                }
                Some(ch) if ch.is_ascii_digit() => {
                    let start = pos;
                    while let Some(ch) = remainder[pos..].chars().next() {
                        if ch.is_ascii_digit() {
                            pos += ch.len_utf8();
                        } else {
                            break;
                        }
                    }
                    if start == pos {
                        return Err(Error::InvalidRange(lex.span()));
                    }
                    let m: usize = remainder[start..pos]
                        .parse()
                        .map_err(|_| Error::InvalidRange(lex.span()))?;
                    skip_ws(remainder, &mut pos);
                    if !matches!(remainder[pos..].chars().next(), Some('}')) {
                        return Err(Error::InvalidRange(lex.span()));
                    }
                    pos += 1;
                    max = Some(m);
                }
                _ => return Err(Error::InvalidRange(lex.span())),
            }
        }
        Some('}') => {
            pos += 1;
            max = Some(min);
        }
        _ => return Err(Error::InvalidRange(lex.span())),
    }

    // Determine greediness
    let mode = match remainder[pos..].chars().next() {
        Some('?') => {
            pos += 1;
            Reluctance::Lazy
        }
        Some('+') => {
            pos += 1;
            Reluctance::Possessive
        }
        _ => Reluctance::Greedy,
    };

    // Consume parsed characters
    lex.bump(pos);

    if let Some(max) = max {
        if min > max {
            return Err(Error::InvalidRange(lex.span()));
        }
        Ok(Quantifier::new(min..=max, mode))
    } else {
        Ok(Quantifier::new(min.., mode))
    }
}

/// Callback used by the `SingleQuoted` variant above.
fn parse_single_quoted(lex: &mut Lexer<Token>) -> Result<String> {
    let src = lex.remainder(); // everything after the first '\''
    let mut escape = false;
    let mut result = String::new();

    for (i, ch) in src.char_indices() {
        match (ch, escape) {
            ('\\', false) => escape = true, // start of an escape
            ('\'', false) => {
                // Found the closing delimiter
                lex.bump(i + 1); // +1 to also eat the '\''
                return Ok(result);
            }
            (c, true) => {
                // Handle escape sequences
                match c {
                    '\'' => result.push('\''),
                    '\\' => result.push('\\'),
                    'n' => result.push('\n'),
                    'r' => result.push('\r'),
                    't' => result.push('\t'),
                    _ => {
                        result.push('\\');
                        result.push(c);
                    }
                }
                escape = false;
            }
            (c, false) => {
                result.push(c);
                escape = false;
            }
        }
    }

    // Unterminated literal – treat as lexing error
    Err(Error::UnterminatedString(lex.span()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic_tokens() {
        // Test meta pattern operators
        assert_eq!(Token::lexer("&").next(), Some(Ok(Token::And)));
        assert_eq!(Token::lexer("|").next(), Some(Ok(Token::Or)));
        assert_eq!(Token::lexer("!").next(), Some(Ok(Token::Not)));
        assert_eq!(Token::lexer("*").next(), Some(Ok(Token::RepeatZeroOrMore)));
        assert_eq!(Token::lexer("+").next(), Some(Ok(Token::RepeatOneOrMore)));
        assert_eq!(Token::lexer("?").next(), Some(Ok(Token::RepeatZeroOrOne)));

        // Test structure pattern keywords
        assert_eq!(Token::lexer("tagged").next(), Some(Ok(Token::Tagged)));

        // Test leaf pattern keywords
        assert_eq!(Token::lexer("bool").next(), Some(Ok(Token::Bool)));
        assert_eq!(Token::lexer("bstr").next(), Some(Ok(Token::ByteString)));
        assert_eq!(Token::lexer("text").next(), Some(Ok(Token::Text)));
        assert_eq!(Token::lexer("number").next(), Some(Ok(Token::Number)));

        // Test literals
        assert_eq!(Token::lexer("true").next(), Some(Ok(Token::BoolTrue)));
        assert_eq!(Token::lexer("false").next(), Some(Ok(Token::BoolFalse)));
        assert_eq!(Token::lexer("NaN").next(), Some(Ok(Token::NaN)));
    }

    #[test]
    fn test_complex_tokens() {
        // Group name
        let mut lexer = Token::lexer("@name");
        if let Some(Ok(Token::GroupName(name))) = lexer.next() {
            assert_eq!(name, "name");
        } else {
            panic!("Failed to parse group name");
        }

        // Test regex
        let mut lexer = Token::lexer("/[a-z]+/");
        if let Some(Ok(Token::Regex(Ok(regex)))) = lexer.next() {
            assert_eq!(regex, "[a-z]+");
        } else {
            panic!("Failed to parse regex");
        }

        let mut lx = Token::lexer(r"/abc\/def/  / /  //  /a\//");
        assert_eq!(
            lx.next(),
            Some(Ok(Token::Regex(Ok("abc\\/def".to_string()))))
        );
        assert_eq!(lx.next(), Some(Ok(Token::Regex(Ok(" ".to_string())))));
        assert_eq!(lx.next(), Some(Ok(Token::Regex(Ok("".to_string())))));
        assert_eq!(lx.next(), Some(Ok(Token::Regex(Ok("a\\/".to_string())))));
        assert_eq!(lx.next(), None);
    }

    #[test]
    fn test_hex_tokens() {
        // Test hex string
        let mut lexer = Token::lexer("h'deadbeef'");
        if let Some(Ok(Token::HexString(Ok(bytes)))) = lexer.next() {
            assert_eq!(bytes, vec![0xde, 0xad, 0xbe, 0xef]);
        } else {
            panic!("Failed to parse hex string");
        }

        // Test empty hex string
        let mut lexer = Token::lexer("h''");
        if let Some(Ok(Token::HexString(Ok(bytes)))) = lexer.next() {
            assert_eq!(bytes, vec![]);
        } else {
            panic!("Failed to parse empty hex string");
        }

        // Test hex regex
        let mut lexer = Token::lexer("h'/^[0-9]+$/'");
        if let Some(Ok(Token::HexRegex(Ok(regex)))) = lexer.next() {
            assert_eq!(regex, "^[0-9]+$");
        } else {
            panic!("Failed to parse hex regex");
        }

        // Test hex regex with escaped slash
        let mut lexer = Token::lexer(r"h'/a\/b/'");
        if let Some(Ok(Token::HexRegex(Ok(regex)))) = lexer.next() {
            assert_eq!(regex, r"a\/b");
        } else {
            panic!("Failed to parse hex regex with escaped slash");
        }
    }

    #[test]
    fn test_number_literals() {
        let mut lexer = Token::lexer("42");
        let token = lexer.next();
        println!("Token for '42': {:?}", token);
        if let Some(Ok(Token::NumberLiteral(Ok(value)))) = token {
            assert_eq!(value, 42.0);
        } else {
            panic!("Failed to parse integer literal");
        }

        // Test zero
        let mut lexer = Token::lexer("0");
        if let Some(Ok(Token::NumberLiteral(Ok(value)))) = lexer.next() {
            assert_eq!(value, 0.0);
        } else {
            panic!("Failed to parse zero literal");
        }

        // Test negative number
        let mut lexer = Token::lexer("-10");
        if let Some(Ok(Token::NumberLiteral(Ok(value)))) = lexer.next() {
            assert_eq!(value, -10.0);
        } else {
            panic!("Failed to parse negative literal");
        }

        // Test floating point
        let mut lexer = Token::lexer("3.2222");
        if let Some(Ok(Token::NumberLiteral(Ok(value)))) = lexer.next() {
            assert_eq!(value, 3.2222);
        } else {
            panic!("Failed to parse float literal");
        }

        // Test scientific notation
        let mut lexer = Token::lexer("1e5");
        if let Some(Ok(Token::NumberLiteral(Ok(value)))) = lexer.next() {
            assert_eq!(value, 100000.0);
        } else {
            panic!("Failed to parse scientific notation literal");
        }
    }

    #[test]
    fn test_range() {
        struct RangeTestCase {
            input: &'static str,
            expected: Quantifier,
        }
        let test_cases = vec![
            RangeTestCase {
                input: "{1, 5}",
                expected: Quantifier::new(1..=5, Reluctance::default()),
            },
            RangeTestCase {
                input: "{ 3 , }",
                expected: Quantifier::new(3.., Reluctance::default()),
            },
            RangeTestCase {
                input: "{ 5 }",
                expected: Quantifier::new(5..=5, Reluctance::default()),
            },
            RangeTestCase {
                input: "{1, 5 }?",
                expected: Quantifier::new(1..=5, Reluctance::Lazy),
            },
            RangeTestCase {
                input: "{ 3 , }?",
                expected: Quantifier::new(3.., Reluctance::Lazy),
            },
            RangeTestCase {
                input: "{5}?",
                expected: Quantifier::new(5..=5, Reluctance::Lazy),
            },
            RangeTestCase {
                input: "{ 1,5}+",
                expected: Quantifier::new(1..=5, Reluctance::Possessive),
            },
            RangeTestCase {
                input: "{ 3 , }+",
                expected: Quantifier::new(3.., Reluctance::Possessive),
            },
            RangeTestCase {
                input: "{5}+",
                expected: Quantifier::new(5..=5, Reluctance::Possessive),
            },
        ];

        let mut failed_cases = vec![];

        for test_case in test_cases {
            let mut lexer = Token::lexer(test_case.input);
            if let Some(Ok(Token::Range(Ok(range)))) = lexer.next() {
                assert_eq!(range, test_case.expected);
            } else {
                failed_cases.push(test_case.input);
            }
        }

        if !failed_cases.is_empty() {
            panic!("Failed to parse ranges: {:?}", failed_cases);
        }
    }
}
