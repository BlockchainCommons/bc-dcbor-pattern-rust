use super::super::{
    Token,
    value::{
        parse_bool, parse_bool_false, parse_bool_true, parse_bytestring,
        parse_date, parse_digest, parse_known_value, parse_null, parse_number,
        parse_text,
    },
};
use crate::{
    Error, MapPattern, Pattern, Result,
    parse::structure::{parse_bracket_array, parse_bracket_map, parse_tagged},
    value::{parse_hex_regex_token, parse_hex_string_token},
};

/// Parse a primary pattern - the most basic unit of pattern matching.
///
/// This parser handles:
/// - * and search meta patterns
/// - Parenthesized group patterns
/// - Capture patterns (@name(...))
/// - All atomic value patterns (bool, text, number, etc.)
/// - All structure patterns (Array, Map, Tagged)
pub(crate) fn parse_primary(
    lexer: &mut logos::Lexer<Token>,
) -> Result<Pattern> {
    let token = match lexer.next() {
        Some(Ok(tok)) => tok,
        Some(Err(e)) => {
            // Convert Unknown errors to UnrecognizedToken with span information
            match e {
                Error::Unknown => {
                    return Err(Error::UnrecognizedToken(lexer.span()));
                }
                _ => return Err(e),
            }
        }
        None => return Err(Error::UnexpectedEndOfInput),
    };

    match token {
        // Meta patterns
        Token::RepeatZeroOrMore => Ok(Pattern::any()), /* '*' as standalone */
        // pattern means
        // "any"
        Token::Search => super::parse_search(lexer),

        // Parenthesized groups - parse the inner pattern and check for
        // quantifiers
        Token::ParenOpen => {
            let pattern = super::parse_or(lexer)?;
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => {
                    // After closing parenthesis, check for quantifiers
                    // Always force RepeatPattern creation for parentheses
                    super::parse_quantifier(pattern, lexer, true)
                }
                Some(Ok(token)) => {
                    Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::UnexpectedEndOfInput),
            }
        }

        // Capture patterns (@name(...))
        Token::GroupName(name) => super::parse_capture(lexer, name),

        // Value patterns
        Token::Bool => parse_bool(lexer),
        Token::BoolTrue => parse_bool_true(lexer),
        Token::BoolFalse => parse_bool_false(lexer),
        Token::ByteString => parse_bytestring(lexer),
        Token::Date => parse_date(lexer),
        Token::Digest => parse_digest(lexer),
        Token::DigestQuoted(res) => {
            let digest_pattern = res?;
            Ok(Pattern::Value(crate::pattern::ValuePattern::Digest(
                digest_pattern,
            )))
        }
        Token::DateQuoted(res) => {
            let date_pattern = res?;
            Ok(Pattern::Value(crate::pattern::ValuePattern::Date(
                date_pattern,
            )))
        }
        Token::Known => parse_known_value(lexer),
        Token::Null => parse_null(lexer),
        Token::Number => parse_number(lexer),
        Token::Text => parse_text(lexer),

        // Direct string literal
        Token::StringLiteral(res) => {
            let value = res?;
            Ok(Pattern::text(value))
        }

        // Single-quoted pattern (non-prefixed known value)
        Token::SingleQuoted(res) => {
            let value = res?;
            parse_single_quoted_as_known_value(value)
        }

        // Direct regex literal
        Token::Regex(res) => {
            let regex_str = res?;
            let regex = regex::Regex::new(&regex_str)
                .map_err(|_| Error::InvalidRegex(lexer.span()))?;
            Ok(Pattern::text_regex(regex))
        }

        // Direct hex string literal
        Token::HexString(res) => parse_hex_string_token(res),

        // Direct hex regex literal
        Token::HexRegex(res) => parse_hex_regex_token(res),

        // Structure patterns
        Token::Tagged => parse_tagged(lexer),

        Token::Array => Ok(Pattern::Structure(
            crate::pattern::StructurePattern::Array(crate::ArrayPattern::any()),
        )),

        Token::Map => Ok(Pattern::Structure(
            crate::pattern::StructurePattern::Map(crate::MapPattern::any()),
        )),

        // Bracket syntax for arrays
        Token::BracketOpen => parse_bracket_array(lexer),

        // Brace syntax for maps
        Token::BraceOpen => parse_bracket_map(lexer),

        // Range tokens that represent map length constraints (e.g., {3}, {2,5})
        Token::Range(res) => {
            // Range tokens at the top level represent map length constraints
            let quantifier = res?;

            // Convert quantifier to appropriate MapPattern
            let pattern = MapPattern::with_length_interval(quantifier.into());

            Ok(Pattern::Structure(crate::pattern::StructurePattern::Map(
                pattern,
            )))
        }

        // New simplified number syntax
        Token::NumberLiteral(res) => {
            let value = res?;

            // Look ahead for range operator
            match lexer.clone().next() {
                Some(Ok(Token::Ellipsis)) => {
                    lexer.next(); // consume the ellipsis
                    match lexer.next() {
                        Some(Ok(Token::NumberLiteral(Ok(end_value)))) => {
                            Ok(Pattern::number_range(value..=end_value))
                        }
                        Some(Ok(Token::NumberLiteral(Err(e)))) => Err(e),
                        Some(Ok(token)) => Err(Error::UnexpectedToken(
                            Box::new(token),
                            lexer.span(),
                        )),
                        Some(Err(e)) => Err(e),
                        None => Err(Error::UnexpectedEndOfInput),
                    }
                }
                _ => Ok(Pattern::number(value)),
            }
        }

        Token::NaN => Ok(Pattern::number_nan()),
        Token::Infinity => Ok(Pattern::number_infinity()),
        Token::NegInfinity => Ok(Pattern::number_neg_infinity()),

        Token::GreaterThanOrEqual => match lexer.next() {
            Some(Ok(Token::NumberLiteral(Ok(value)))) => {
                Ok(Pattern::number_greater_than_or_equal(value))
            }
            Some(Ok(Token::NumberLiteral(Err(e)))) => Err(e),
            Some(Ok(token)) => {
                Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
            }
            Some(Err(e)) => Err(e),
            None => Err(Error::UnexpectedEndOfInput),
        },

        Token::LessThanOrEqual => match lexer.next() {
            Some(Ok(Token::NumberLiteral(Ok(value)))) => {
                Ok(Pattern::number_less_than_or_equal(value))
            }
            Some(Ok(Token::NumberLiteral(Err(e)))) => Err(e),
            Some(Ok(token)) => {
                Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
            }
            Some(Err(e)) => Err(e),
            None => Err(Error::UnexpectedEndOfInput),
        },

        Token::GreaterThan => match lexer.next() {
            Some(Ok(Token::NumberLiteral(Ok(value)))) => {
                Ok(Pattern::number_greater_than(value))
            }
            Some(Ok(Token::NumberLiteral(Err(e)))) => Err(e),
            Some(Ok(token)) => {
                Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
            }
            Some(Err(e)) => Err(e),
            None => Err(Error::UnexpectedEndOfInput),
        },

        Token::LessThan => match lexer.next() {
            Some(Ok(Token::NumberLiteral(Ok(value)))) => {
                Ok(Pattern::number_less_than(value))
            }
            Some(Ok(Token::NumberLiteral(Err(e)))) => Err(e),
            Some(Ok(token)) => {
                Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
            }
            Some(Err(e)) => Err(e),
            None => Err(Error::UnexpectedEndOfInput),
        },

        // Unexpected tokens
        _ => Err(Error::UnexpectedToken(Box::new(token), lexer.span())),
    }
}

/// Parse a single-quoted pattern as a known value.
/// This handles the non-prefixed single-quoted syntax:
/// - 'value' -> known value by numeric ID
/// - 'name' -> known value by name
/// - '/regex/' -> known value by regex
fn parse_single_quoted_as_known_value(value: String) -> Result<Pattern> {
    // Check if it's a regex pattern (starts and ends with /)
    if value.starts_with('/') && value.ends_with('/') && value.len() > 2 {
        let regex_str = &value[1..value.len() - 1];
        let regex = regex::Regex::new(regex_str)
            .map_err(|_| Error::InvalidRegex(0..value.len()))?;
        return Ok(Pattern::known_value_regex(regex));
    }

    // Try to parse as numeric ID
    if let Ok(numeric_value) = value.parse::<u64>() {
        return Ok(Pattern::known_value(known_values::KnownValue::new(
            numeric_value,
        )));
    }

    // Otherwise treat as name
    Ok(Pattern::known_value_named(value))
}
