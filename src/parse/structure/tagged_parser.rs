use dcbor::prelude::*;

use crate::{Error, Pattern, Result, TaggedPattern, parse::Token};

/// Parse a tagged pattern.
///
/// Supports the following syntax:
/// - `tagged` - matches any tagged value
/// - `tagged(value, pattern)` - matches tagged value with specific u64 tag and
///   content pattern
/// - `tagged(name, pattern)` - matches tagged value with named tag and content
///   pattern
/// - `tagged(/regex/, pattern)` - matches tagged value with tag name matching
///   regex and content pattern
pub(crate) fn parse_tagged(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            // Consume the '(' token
            lexer.next();

            // Track the current lexer position for error adjustment
            let remainder_start = lexer.span().end;
            let src = lexer.remainder();
            let (tag_pattern, content_pattern, consumed) =
                parse_tagged_inner(src, remainder_start)?;
            lexer.bump(consumed);

            // Expect closing parenthesis
            match lexer.next() {
                Some(Ok(Token::ParenClose)) => {
                    let pattern = match tag_pattern {
                        TagSelector::Any => TaggedPattern::Any,
                        TagSelector::Value(tag_val) => {
                            let tag = Tag::new(tag_val, "");
                            TaggedPattern::with_tag(tag, content_pattern)
                        }
                        TagSelector::Name(tag_name) => {
                            // For named tags, treat it as a named tag match
                            TaggedPattern::with_name(tag_name, content_pattern)
                        }
                        TagSelector::Regex(regex) => {
                            TaggedPattern::with_regex(regex, content_pattern)
                        }
                    };

                    Ok(Pattern::Structure(
                        crate::pattern::StructurePattern::Tagged(pattern),
                    ))
                }
                Some(Ok(token)) => {
                    Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
                }
                Some(Err(e)) => Err(e),
                None => Err(Error::ExpectedCloseParen(lexer.span())),
            }
        }
        _ => {
            // No parentheses, just "tagged" - matches any tagged value
            Ok(Pattern::Structure(
                crate::pattern::StructurePattern::Tagged(TaggedPattern::any()),
            ))
        }
    }
}

#[derive(Debug)]
enum TagSelector {
    Any,
    Value(u64),
    Name(String),
    Regex(regex::Regex),
}

fn parse_tagged_inner(
    src: &str,
    remainder_start: usize,
) -> Result<(TagSelector, Pattern, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);

    // Parse the tag selector (first parameter)
    let tag_selector = if src[pos..].starts_with('/') {
        // Regex pattern
        let (regex, used) = parse_text_regex(&src[pos..])?;
        pos += used;
        TagSelector::Regex(regex)
    } else {
        // Could be a number or a name
        let (word, used) = parse_bare_word(&src[pos..])?;
        pos += used;
        if let Ok(value) = word.parse::<u64>() {
            TagSelector::Value(value)
        } else {
            TagSelector::Name(word)
        }
    };

    // Expect comma
    skip_ws(src, &mut pos);
    if pos >= src.len() || src.as_bytes()[pos] != b',' {
        return Err(Error::UnexpectedEndOfInput);
    }
    pos += 1;
    skip_ws(src, &mut pos);

    // Parse the content pattern (second parameter)
    // For now, we'll parse the rest as a simple pattern string
    // This is a simplified approach - a more robust implementation would
    // need to handle nested parentheses and complex patterns
    let pattern_start = pos;
    let mut paren_depth = 0;
    while pos < src.len() {
        let ch = src.as_bytes()[pos];
        if ch == b'(' {
            paren_depth += 1;
        } else if ch == b')' {
            if paren_depth == 0 {
                break; // This is the closing paren for our tagged()
            }
            paren_depth -= 1;
        }
        pos += 1;
    }

    let pattern_src = &src[pattern_start..pos];
    let trimmed_pattern = pattern_src.trim();
    let trim_offset = pattern_src.len() - pattern_src.trim_start().len();

    let content_pattern = Pattern::parse(trimmed_pattern).map_err(|e| {
        // Adjust error spans to be relative to the original input
        match e {
            Error::UnrecognizedToken(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::UnrecognizedToken(adjusted_start..adjusted_end)
            }
            Error::ExtraData(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::ExtraData(adjusted_start..adjusted_end)
            }
            Error::UnexpectedToken(token, span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::UnexpectedToken(token, adjusted_start..adjusted_end)
            }
            Error::InvalidRegex(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::InvalidRegex(adjusted_start..adjusted_end)
            }
            Error::UnterminatedRegex(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::UnterminatedRegex(adjusted_start..adjusted_end)
            }
            Error::UnterminatedString(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::UnterminatedString(adjusted_start..adjusted_end)
            }
            Error::InvalidRange(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::InvalidRange(adjusted_start..adjusted_end)
            }
            Error::InvalidHexString(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::InvalidHexString(adjusted_start..adjusted_end)
            }
            Error::UnterminatedHexString(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::UnterminatedHexString(adjusted_start..adjusted_end)
            }
            Error::InvalidDateFormat(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::InvalidDateFormat(adjusted_start..adjusted_end)
            }
            Error::InvalidNumberFormat(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::InvalidNumberFormat(adjusted_start..adjusted_end)
            }
            Error::InvalidUr(msg, span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::InvalidUr(msg, adjusted_start..adjusted_end)
            }
            Error::ExpectedOpenParen(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::ExpectedOpenParen(adjusted_start..adjusted_end)
            }
            Error::ExpectedCloseParen(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::ExpectedCloseParen(adjusted_start..adjusted_end)
            }
            Error::ExpectedCloseBracket(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::ExpectedCloseBracket(adjusted_start..adjusted_end)
            }
            Error::ExpectedCloseBrace(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::ExpectedCloseBrace(adjusted_start..adjusted_end)
            }
            Error::ExpectedColon(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::ExpectedColon(adjusted_start..adjusted_end)
            }
            Error::ExpectedPattern(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::ExpectedPattern(adjusted_start..adjusted_end)
            }
            Error::UnmatchedParentheses(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::UnmatchedParentheses(adjusted_start..adjusted_end)
            }
            Error::UnmatchedBraces(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::UnmatchedBraces(adjusted_start..adjusted_end)
            }
            Error::InvalidCaptureGroupName(name, span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::InvalidCaptureGroupName(
                    name,
                    adjusted_start..adjusted_end,
                )
            }
            Error::InvalidDigestPattern(msg, span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::InvalidDigestPattern(msg, adjusted_start..adjusted_end)
            }
            Error::UnterminatedDigestQuoted(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::UnterminatedDigestQuoted(adjusted_start..adjusted_end)
            }
            Error::UnterminatedDateQuoted(span) => {
                let adjusted_start =
                    remainder_start + pattern_start + trim_offset + span.start;
                let adjusted_end =
                    remainder_start + pattern_start + trim_offset + span.end;
                Error::UnterminatedDateQuoted(adjusted_start..adjusted_end)
            }
            // For errors without spans, return them as-is
            _ => e,
        }
    })?;

    Ok((tag_selector, content_pattern, pos))
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

/// Parse a bare word (alphanumeric with hyphens and underscores)
fn parse_bare_word(src: &str) -> Result<(String, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);
    let start = pos;
    while pos < src.len() {
        let ch = src[pos..].chars().next().unwrap();
        if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}' | ',' | ')') {
            break;
        }
        pos += ch.len_utf8();
    }
    if start == pos {
        return Err(Error::UnexpectedEndOfInput);
    }
    let word = src[start..pos].to_string();
    skip_ws(src, &mut pos);
    Ok((word, pos))
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
    fn test_parse_tagged_any() {
        let pattern = Pattern::parse("tagged").unwrap();
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Tagged(
                TaggedPattern::any()
            ))
        );
        assert_eq!(pattern.to_string(), "tagged");
    }

    #[test]
    fn test_parse_tagged_with_value() {
        let pattern = Pattern::parse("tagged(1234, text)").unwrap();
        let tag = Tag::new(1234, "");
        assert_eq!(
            pattern,
            Pattern::Structure(crate::pattern::StructurePattern::Tagged(
                TaggedPattern::with_tag(tag, Pattern::any_text())
            ))
        );
    }

    #[test]
    fn test_parse_tagged_with_regex() {
        let pattern = Pattern::parse("tagged(/test.*/, text)").unwrap();
        match pattern {
            Pattern::Structure(crate::pattern::StructurePattern::Tagged(
                TaggedPattern::Regex { .. },
            )) => {} // This is expected
            _ => panic!("Expected TaggedPattern with regex"),
        }
    }

    #[test]
    fn test_parse_tagged_with_name() {
        let pattern = Pattern::parse("tagged(myTag, number)").unwrap();
        match pattern {
            Pattern::Structure(crate::pattern::StructurePattern::Tagged(
                TaggedPattern::Name { name: tag_name, .. },
            )) => {
                assert_eq!(tag_name, "myTag");
            }
            _ => panic!("Expected TaggedPattern with name and content"),
        }
    }

    #[test]
    fn test_parse_complex_regex() {
        let pattern = Pattern::parse("tagged(/^test[0-9]+$/, text)").unwrap();
        match pattern {
            Pattern::Structure(crate::pattern::StructurePattern::Tagged(
                TaggedPattern::Regex { regex: tag_regex, .. },
            )) => {
                assert_eq!(tag_regex.as_str(), "^test[0-9]+$");
            }
            _ => panic!("Expected TaggedPattern with regex and content"),
        }
    }

    #[test]
    fn test_parse_tagged_value_zero() {
        let pattern = Pattern::parse("tagged(0, null)").unwrap();
        match pattern {
            Pattern::Structure(crate::pattern::StructurePattern::Tagged(
                TaggedPattern::Tag { tag, .. },
            )) => {
                assert_eq!(tag.value(), 0);
            }
            _ => panic!("Expected TaggedPattern with tag value 0"),
        }
    }
}
