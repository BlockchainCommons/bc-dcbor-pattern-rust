use crate::{
    Error, Result,
    pattern::{
        Matcher, Path, meta::MetaPattern, structure::StructurePattern,
        value::ValuePattern, vm::Instr,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Value(ValuePattern),
    Structure(StructurePattern),
    Meta(MetaPattern),
}

impl Pattern {
    /// Creates a pattern that matches any boolean value.
    pub fn any_bool() -> Self {
        Pattern::Value(ValuePattern::Bool(
            crate::pattern::value::BoolPattern::any(),
        ))
    }

    /// Creates a pattern that matches a specific boolean value.
    pub fn bool(value: bool) -> Self {
        Pattern::Value(ValuePattern::Bool(
            crate::pattern::value::BoolPattern::value(value),
        ))
    }

    /// Creates a pattern that matches any number value.
    pub fn any_number() -> Self {
        Pattern::Value(ValuePattern::Number(
            crate::pattern::value::NumberPattern::any(),
        ))
    }

    /// Creates a pattern that matches a specific number value.
    pub fn number<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Pattern::Value(ValuePattern::Number(
            crate::pattern::value::NumberPattern::exact(value),
        ))
    }

    /// Creates a pattern that matches numbers within a range.
    pub fn number_range<A>(range: std::ops::RangeInclusive<A>) -> Self
    where
        A: Into<f64> + Copy,
    {
        Pattern::Value(ValuePattern::Number(
            crate::pattern::value::NumberPattern::range(range),
        ))
    }

    /// Creates a pattern that matches numbers greater than the specified value.
    pub fn number_greater_than<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Pattern::Value(ValuePattern::Number(
            crate::pattern::value::NumberPattern::greater_than(value),
        ))
    }

    /// Creates a pattern that matches numbers greater than or equal to the
    /// specified value.
    pub fn number_greater_than_or_equal<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Pattern::Value(ValuePattern::Number(
            crate::pattern::value::NumberPattern::greater_than_or_equal(value),
        ))
    }

    /// Creates a pattern that matches numbers less than the specified value.
    pub fn number_less_than<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Pattern::Value(ValuePattern::Number(
            crate::pattern::value::NumberPattern::less_than(value),
        ))
    }

    /// Creates a pattern that matches numbers less than or equal to the
    /// specified value.
    pub fn number_less_than_or_equal<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        Pattern::Value(ValuePattern::Number(
            crate::pattern::value::NumberPattern::less_than_or_equal(value),
        ))
    }

    /// Creates a pattern that matches NaN values.
    pub fn number_nan() -> Self {
        Pattern::Value(ValuePattern::Number(
            crate::pattern::value::NumberPattern::nan(),
        ))
    }

    /// Creates a pattern that matches any text value.
    pub fn any_text() -> Self {
        Pattern::Value(ValuePattern::Text(
            crate::pattern::value::TextPattern::any(),
        ))
    }

    /// Creates a pattern that matches a specific text value.
    pub fn text<T: Into<String>>(value: T) -> Self {
        Pattern::Value(ValuePattern::Text(
            crate::pattern::value::TextPattern::value(value),
        ))
    }

    /// Creates a pattern that matches text using a regex.
    pub fn text_regex(regex: regex::Regex) -> Self {
        Pattern::Value(ValuePattern::Text(
            crate::pattern::value::TextPattern::regex(regex),
        ))
    }

    /// Creates a pattern that matches any byte string value.
    pub fn any_byte_string() -> Self {
        Pattern::Value(ValuePattern::ByteString(
            crate::pattern::value::ByteStringPattern::any(),
        ))
    }

    /// Creates a pattern that matches a specific byte string value.
    pub fn byte_string(value: impl AsRef<[u8]>) -> Self {
        Pattern::Value(ValuePattern::ByteString(
            crate::pattern::value::ByteStringPattern::value(value),
        ))
    }

    /// Creates a pattern that matches byte strings using a binary regex.
    pub fn byte_string_regex(regex: regex::bytes::Regex) -> Self {
        Pattern::Value(ValuePattern::ByteString(
            crate::pattern::value::ByteStringPattern::regex(regex),
        ))
    }

    /// Creates a pattern that matches any date value.
    pub fn any_date() -> Self {
        Pattern::Value(ValuePattern::Date(
            crate::pattern::value::DatePattern::any(),
        ))
    }

    /// Creates a pattern that matches a specific date value.
    pub fn date(date: dcbor::Date) -> Self {
        Pattern::Value(ValuePattern::Date(
            crate::pattern::value::DatePattern::value(date),
        ))
    }

    /// Creates a pattern that matches dates within a range (inclusive).
    pub fn date_range(range: std::ops::RangeInclusive<dcbor::Date>) -> Self {
        Pattern::Value(ValuePattern::Date(
            crate::pattern::value::DatePattern::range(range),
        ))
    }

    /// Creates a pattern that matches dates that are on or after the specified
    /// date.
    pub fn date_earliest(date: dcbor::Date) -> Self {
        Pattern::Value(ValuePattern::Date(
            crate::pattern::value::DatePattern::earliest(date),
        ))
    }

    /// Creates a pattern that matches dates that are on or before the specified
    /// date.
    pub fn date_latest(date: dcbor::Date) -> Self {
        Pattern::Value(ValuePattern::Date(
            crate::pattern::value::DatePattern::latest(date),
        ))
    }

    /// Creates a pattern that matches a date by its ISO-8601 string
    /// representation.
    pub fn date_iso8601(iso_string: impl Into<String>) -> Self {
        Pattern::Value(ValuePattern::Date(
            crate::pattern::value::DatePattern::iso8601(iso_string),
        ))
    }

    /// Creates a pattern that matches dates whose ISO-8601 string
    /// representation matches the given regex pattern.
    pub fn date_regex(regex: regex::Regex) -> Self {
        Pattern::Value(ValuePattern::Date(
            crate::pattern::value::DatePattern::regex(regex),
        ))
    }

    /// Creates a pattern that matches null values.
    pub fn null() -> Self {
        Pattern::Value(ValuePattern::Null(
            crate::pattern::value::NullPattern::new(),
        ))
    }

    /// Creates a pattern that matches any known value.
    pub fn any_known_value() -> Self {
        Pattern::Value(ValuePattern::KnownValue(
            crate::pattern::value::KnownValuePattern::any(),
        ))
    }

    /// Creates a pattern that matches a specific known value.
    pub fn known_value(value: known_values::KnownValue) -> Self {
        Pattern::Value(ValuePattern::KnownValue(
            crate::pattern::value::KnownValuePattern::value(value),
        ))
    }

    /// Creates a pattern that matches a known value by name.
    pub fn known_value_named(name: impl Into<String>) -> Self {
        Pattern::Value(ValuePattern::KnownValue(
            crate::pattern::value::KnownValuePattern::named(name),
        ))
    }

    /// Creates a pattern that matches known values using a regex on their
    /// names.
    pub fn known_value_regex(regex: regex::Regex) -> Self {
        Pattern::Value(ValuePattern::KnownValue(
            crate::pattern::value::KnownValuePattern::regex(regex),
        ))
    }

    // Digest pattern convenience methods

    /// Creates a pattern that matches a specific digest.
    pub fn digest(digest: bc_components::Digest) -> Self {
        Pattern::Value(ValuePattern::Digest(
            crate::pattern::value::DigestPattern::digest(digest),
        ))
    }

    /// Creates a pattern that matches digests with the specified prefix.
    pub fn digest_prefix(prefix: impl AsRef<[u8]>) -> Self {
        Pattern::Value(ValuePattern::Digest(
            crate::pattern::value::DigestPattern::prefix(prefix),
        ))
    }

    /// Creates a pattern that matches digests using a binary regex.
    pub fn digest_binary_regex(regex: regex::bytes::Regex) -> Self {
        Pattern::Value(ValuePattern::Digest(
            crate::pattern::value::DigestPattern::binary_regex(regex),
        ))
    }

    /// Creates a pattern that always matches any CBOR value.
    pub fn any() -> Self {
        Pattern::Meta(MetaPattern::Any(crate::pattern::meta::AnyPattern::new()))
    }

    /// Creates a pattern that never matches any CBOR value.
    pub fn none() -> Self {
        Pattern::Meta(MetaPattern::None(
            crate::pattern::meta::NonePattern::new(),
        ))
    }

    /// Creates a pattern that matches if all contained patterns match.
    pub fn and(patterns: Vec<Pattern>) -> Self {
        Pattern::Meta(MetaPattern::And(crate::pattern::meta::AndPattern::new(
            patterns,
        )))
    }

    /// Creates a pattern that matches if any contained pattern matches.
    pub fn or(patterns: Vec<Pattern>) -> Self {
        Pattern::Meta(MetaPattern::Or(crate::pattern::meta::OrPattern::new(
            patterns,
        )))
    }

    /// Creates a pattern that matches if the inner pattern does not match.
    pub fn not_matching(pattern: Pattern) -> Self {
        Pattern::Meta(MetaPattern::Not(crate::pattern::meta::NotPattern::new(
            pattern,
        )))
    }

    /// Creates a pattern that captures matches with the given name.
    pub fn capture(name: impl AsRef<str>, pattern: Pattern) -> Self {
        Pattern::Meta(MetaPattern::Capture(
            crate::pattern::meta::CapturePattern::new(name, pattern),
        ))
    }

    /// Creates a pattern that matches with repetition using a quantifier.
    pub fn repeat(pattern: Pattern, quantifier: crate::Quantifier) -> Self {
        Pattern::Meta(MetaPattern::Repeat(
            crate::pattern::meta::RepeatPattern::repeat(pattern, quantifier),
        ))
    }

    /// Creates a pattern that wraps another pattern (matches exactly once).
    pub fn group(pattern: Pattern) -> Self {
        Pattern::Meta(MetaPattern::Repeat(
            crate::pattern::meta::RepeatPattern::new(pattern),
        ))
    }

    /// Parses a pattern from a string.
    ///
    /// This implementation currently supports boolean, date, number, null, and
    /// text patterns. More patterns will be added as they are implemented.
    pub fn parse(input: &str) -> Result<Self> {
        use logos::Logos;

        use crate::parse::{
            Token,
            value::{
                parse_bool, parse_bytestring, parse_date, parse_null, parse_number, parse_text,
            },
        };

        let mut lexer = Token::lexer(input);

        match lexer.next() {
            Some(Ok(Token::Bool)) => parse_bool(&mut lexer),
            Some(Ok(Token::ByteString)) => parse_bytestring(&mut lexer),
            Some(Ok(Token::Date)) => parse_date(&mut lexer),
            Some(Ok(Token::Number)) => parse_number(&mut lexer),
            Some(Ok(Token::Null)) => parse_null(&mut lexer),
            Some(Ok(Token::Text)) => parse_text(&mut lexer),
            Some(Ok(token)) => {
                Err(Error::UnexpectedToken(Box::new(token), lexer.span()))
            }
            Some(Err(e)) => Err(e),
            None => Err(Error::EmptyInput),
        }
    }
}

impl Matcher for Pattern {
    fn paths(&self, cbor: &dcbor::CBOR) -> Vec<Path> {
        match self {
            Pattern::Value(pattern) => pattern.paths(cbor),
            Pattern::Structure(pattern) => pattern.paths(cbor),
            Pattern::Meta(pattern) => pattern.paths(cbor),
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        match self {
            Pattern::Value(pattern) => {
                pattern.compile(code, literals, captures);
            }
            Pattern::Structure(pattern) => {
                pattern.compile(code, literals, captures);
            }
            Pattern::Meta(pattern) => {
                pattern.compile(code, literals, captures);
            }
        }
    }

    fn is_complex(&self) -> bool {
        match self {
            Pattern::Value(pattern) => pattern.is_complex(),
            Pattern::Structure(_pattern) => false, /* TODO: implement when */
            // ready
            Pattern::Meta(pattern) => pattern.is_complex(),
        }
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Value(pattern) => write!(f, "{}", pattern),
            Pattern::Structure(pattern) => write!(f, "{:?}", pattern), /* Temporary */
            Pattern::Meta(pattern) => write!(f, "{}", pattern),
        }
    }
}
