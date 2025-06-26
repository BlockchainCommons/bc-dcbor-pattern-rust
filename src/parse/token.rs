use dcbor::prelude::*;
use dcbor_parse::parse_dcbor_item_partial;
use logos::{Lexer, Logos};

use crate::{Error, Quantifier, Reluctance, Result};

/// Tokens for the Gordian Envelope pattern syntax.
#[derive(Debug, Clone, Logos, PartialEq)]
#[rustfmt::skip]
#[logos(error = Error)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    // Meta Pattern Operators
    #[token("&")]
    And,

    #[token("|")]
    Or,

    #[token("!")]
    Not,

    #[token(">", priority = 2)]
    Sequence,

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
    #[token("TAG")]
    Tagged,

    #[token("MAP")]
    Map,

    #[token("ARRAY")]
    Array,

    // Value Pattern Keywords
    #[token("BOOL")]
    Bool,

    #[token("BSTR")]
    ByteString,

    #[token("DATE")]
    Date,

    #[token("KNOWN")]
    Known,

    #[token("NULL")]
    Null,

    #[token("NUMBER")]
    Number,

    #[token("TEXT")]
    Text,

    #[token("DIGEST")]
    Digest,

    // Meta Pattern Keywords
    #[token("ANY")]
    Any,

    #[token("SEARCH")]
    Search,

    #[token("NONE")]
    None,

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

    #[token("/", parse_regex)]
    Regex(Result<String>),

    #[token("{", parse_range)]
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
fn parse_range(lex: &mut Lexer<Token>) -> Result<Quantifier> {
    let src = lex.remainder(); // everything after the first '{'

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

    // parse minimum value --------------------------------------------------
    skip_ws(src, &mut pos);
    let start = pos;
    while let Some(ch) = src[pos..].chars().next() {
        if ch.is_ascii_digit() {
            pos += ch.len_utf8();
        } else {
            break;
        }
    }
    if start == pos {
        return Err(Error::InvalidRange(lex.span()));
    }
    let min: usize = src[start..pos]
        .parse()
        .map_err(|_| Error::InvalidRange(lex.span()))?;

    skip_ws(src, &mut pos);

    // parse optional comma and maximum value -------------------------------
    let max: Option<usize>;

    match src[pos..].chars().next() {
        Some(',') => {
            pos += 1;
            skip_ws(src, &mut pos);

            // If the next non-space char is '}', the range is open ended
            match src[pos..].chars().next() {
                Some('}') => {
                    pos += 1;
                    max = None;
                }
                Some(ch) if ch.is_ascii_digit() => {
                    let start = pos;
                    while let Some(ch) = src[pos..].chars().next() {
                        if ch.is_ascii_digit() {
                            pos += ch.len_utf8();
                        } else {
                            break;
                        }
                    }
                    if start == pos {
                        return Err(Error::InvalidRange(lex.span()));
                    }
                    let m: usize = src[start..pos]
                        .parse()
                        .map_err(|_| Error::InvalidRange(lex.span()))?;
                    skip_ws(src, &mut pos);
                    if !matches!(src[pos..].chars().next(), Some('}')) {
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

    // determine greediness -------------------------------------------------
    let mode = match src[pos..].chars().next() {
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

    // consume parsed characters (everything after '{')
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic_tokens() {
        // Test meta pattern operators
        assert_eq!(Token::lexer("&").next(), Some(Ok(Token::And)));
        assert_eq!(Token::lexer("|").next(), Some(Ok(Token::Or)));
        assert_eq!(Token::lexer("!").next(), Some(Ok(Token::Not)));
        assert_eq!(Token::lexer(">").next(), Some(Ok(Token::Sequence)));
        assert_eq!(Token::lexer("*").next(), Some(Ok(Token::RepeatZeroOrMore)));
        assert_eq!(Token::lexer("+").next(), Some(Ok(Token::RepeatOneOrMore)));
        assert_eq!(Token::lexer("?").next(), Some(Ok(Token::RepeatZeroOrOne)));

        // Test structure pattern keywords
        assert_eq!(Token::lexer("ARRAY").next(), Some(Ok(Token::Array)));
        assert_eq!(Token::lexer("MAP").next(), Some(Ok(Token::Map)));
        assert_eq!(Token::lexer("TAG").next(), Some(Ok(Token::Tagged)));

        // Test leaf pattern keywords
        assert_eq!(Token::lexer("BOOL").next(), Some(Ok(Token::Bool)));
        assert_eq!(Token::lexer("TEXT").next(), Some(Ok(Token::Text)));
        assert_eq!(Token::lexer("NUMBER").next(), Some(Ok(Token::Number)));

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
