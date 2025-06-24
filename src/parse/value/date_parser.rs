use dcbor::Date;
use dcbor_parse::parse_dcbor_item;

use crate::{Error, Pattern, Result, parse::Token};

pub(crate) fn parse_date(lexer: &mut logos::Lexer<Token>) -> Result<Pattern> {
    let mut lookahead = lexer.clone();
    match lookahead.next() {
        Some(Ok(Token::ParenOpen)) => {
            lexer.next();
            let src = lexer.remainder();
            let (pattern, consumed) = parse_date_inner(src)?;
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
        _ => Ok(Pattern::any_date()),
    }
}

fn parse_date_inner(src: &str) -> Result<(Pattern, usize)> {
    let mut pos = 0;
    skip_ws(src, &mut pos);

    // Check for regex pattern
    if src[pos..].starts_with('/') {
        let (regex, used) = parse_text_regex(&src[pos..])?;
        pos += used;
        return Ok((Pattern::date_regex(regex), pos));
    }

    // Check for open-ended range starting with ellipsis
    if src[pos..].starts_with("...") {
        pos += 3;
        let date = parse_iso8601(src, &mut pos)?;
        return Ok((Pattern::date_latest(date), pos));
    }

    // Parse first date
    let first = parse_iso8601(src, &mut pos)?;

    // Check for range operator
    if src[pos..].starts_with("...") {
        pos += 3;
        // Check if this is an open-ended range (no second date)
        skip_ws(src, &mut pos);
        if pos >= src.len() || src.as_bytes()[pos] == b')' {
            return Ok((Pattern::date_earliest(first), pos));
        }
        // Otherwise, parse the second date
        let second = parse_iso8601(src, &mut pos)?;
        return Ok((Pattern::date_range(first..=second), pos));
    }

    Ok((Pattern::date(first), pos))
}

fn skip_ws(src: &str, pos: &mut usize) {
    while let Some(ch) = src[*pos..].chars().next() {
        if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
            *pos += ch.len_utf8();
        } else {
            break;
        }
    }
}

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

fn parse_iso8601(src: &str, pos: &mut usize) -> Result<Date> {
    skip_ws(src, pos);
    let start = *pos;
    while *pos < src.len() {
        if src[*pos..].starts_with("...") || src.as_bytes()[*pos] == b')' {
            break;
        }
        let ch = src[*pos..].chars().next().unwrap();
        if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0c}') {
            break;
        }
        *pos += ch.len_utf8();
    }
    if start == *pos {
        return Err(Error::InvalidDateFormat(0..0));
    }
    let iso = &src[start..*pos];

    // Try to parse as ISO-8601 using dcbor-parse
    match parse_dcbor_item(iso) {
        Ok(cbor) => match Date::try_from(cbor) {
            Ok(date) => {
                skip_ws(src, pos);
                Ok(date)
            }
            Err(_) => Err(Error::InvalidDateFormat(0..0)),
        },
        Err(_) => Err(Error::InvalidDateFormat(0..0)),
    }
}
