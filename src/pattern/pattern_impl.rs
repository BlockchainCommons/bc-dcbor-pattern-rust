use crate::{
    Error, Result,
    pattern::{
        Matcher, Path, meta::MetaPattern, structure::StructurePattern,
        value::ValuePattern, vm::Instr,
    },
};

use dcbor::prelude::*;

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
            crate::pattern::value::NumberPattern::value(value),
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

    /// Creates a pattern that matches positive infinity values.
    pub fn number_infinity() -> Self {
        Pattern::Value(ValuePattern::Number(
            crate::pattern::value::NumberPattern::infinity(),
        ))
    }

    /// Creates a pattern that matches negative infinity values.
    pub fn number_neg_infinity() -> Self {
        Pattern::Value(ValuePattern::Number(
            crate::pattern::value::NumberPattern::neg_infinity(),
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
    pub fn date(date: Date) -> Self {
        Pattern::Value(ValuePattern::Date(
            crate::pattern::value::DatePattern::value(date),
        ))
    }

    /// Creates a pattern that matches dates within a range (inclusive).
    pub fn date_range(range: std::ops::RangeInclusive<Date>) -> Self {
        Pattern::Value(ValuePattern::Date(
            crate::pattern::value::DatePattern::range(range),
        ))
    }

    /// Creates a pattern that matches dates that are on or after the specified
    /// date.
    pub fn date_earliest(date: Date) -> Self {
        Pattern::Value(ValuePattern::Date(
            crate::pattern::value::DatePattern::earliest(date),
        ))
    }

    /// Creates a pattern that matches dates that are on or before the specified
    /// date.
    pub fn date_latest(date: Date) -> Self {
        Pattern::Value(ValuePattern::Date(
            crate::pattern::value::DatePattern::latest(date),
        ))
    }

    /// Creates a pattern that matches a date by its ISO-8601 string
    /// representation.
    pub fn date_iso8601(iso_string: impl Into<String>) -> Self {
        Pattern::Value(ValuePattern::Date(
            crate::pattern::value::DatePattern::string(iso_string),
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
        Pattern::Value(ValuePattern::Null(crate::pattern::value::NullPattern))
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

    /// Creates a pattern that matches any digest value.
    pub fn any_digest() -> Self {
        Pattern::Value(ValuePattern::Digest(
            crate::pattern::value::DigestPattern::any(),
        ))
    }

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

    /// Creates a search pattern that recursively searches the entire dCBOR
    /// tree.
    pub fn search(pattern: Pattern) -> Self {
        Pattern::Meta(MetaPattern::Search(
            crate::pattern::meta::SearchPattern::new(pattern),
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

    /// Creates a sequence pattern that matches patterns in order.
    pub fn sequence(patterns: Vec<Pattern>) -> Self {
        Pattern::Meta(MetaPattern::Sequence(
            crate::pattern::meta::SequencePattern::new(patterns),
        ))
    }

    /// Creates a pattern that matches any array.
    pub fn any_array() -> Self {
        Pattern::Structure(crate::pattern::structure::StructurePattern::Array(
            crate::pattern::structure::ArrayPattern::any(),
        ))
    }

    /// Creates a pattern that matches any map.
    pub fn any_map() -> Self {
        Pattern::Structure(crate::pattern::structure::StructurePattern::Map(
            crate::pattern::structure::MapPattern::any(),
        ))
    }
}


impl Pattern {
    /// Creates a pattern that matches any tagged value.
    pub fn any_tagged() -> Self {
        Pattern::Structure(crate::pattern::structure::StructurePattern::Tagged(
            crate::pattern::structure::TaggedPattern::any(),
        ))
    }

    /// Creates a pattern that matches a tagged item with content pattern.
    pub fn tagged(tag: impl Into<Tag>, pattern: Pattern) -> Self {
        Pattern::Structure(crate::pattern::structure::StructurePattern::Tagged(
            crate::pattern::structure::TaggedPattern::with_tag(tag, pattern),
        ))
    }

    /// Creates a pattern that matches a tagged item with content pattern and
    /// a specific tag name.
    pub fn tagged_name(name: impl Into<String>, pattern: Pattern) -> Self {
        Pattern::Structure(crate::pattern::structure::StructurePattern::Tagged(
            crate::pattern::structure::TaggedPattern::with_name(name, pattern),
        ))
    }

    /// Creates a pattern that matches a tagged item with content pattern and
    /// a regex for the tag name.
    pub fn tagged_regex(regex: regex::Regex, pattern: Pattern) -> Self {
        Pattern::Structure(crate::pattern::structure::StructurePattern::Tagged(
            crate::pattern::structure::TaggedPattern::with_regex(regex, pattern),
        ))
    }
}

impl Pattern {
    /// Parse a pattern expression from a string.
    ///
    /// This method supports the full dCBOR pattern syntax including:
    /// - Value patterns: bool, text, number, null, bstr, date, digest, known
    /// - Structure patterns: Array, Map, Tagged
    /// - Meta patterns: *, NONE, AND (&), OR (|), NOT (!)
    /// - Capture patterns: @name(pattern)
    /// - Grouping with parentheses
    /// - Quantifiers: *, +, ?, {n,m}
    ///
    /// Examples:
    /// - `bool` - matches any boolean value
    /// - `true` - matches the boolean value true
    /// - `false` - matches the boolean value false
    /// - `"hello"` - matches the text "hello"
    /// - `1..10` - matches numbers from 1 to 10
    /// - `bool | text` - matches boolean or text values
    /// - `@name(text)` - captures text with name "name"
    pub fn parse(input: &str) -> Result<Self> {
        let (pattern, consumed) = Self::parse_partial(input)?;
        if consumed < input.len() {
            // Find where we stopped to provide accurate error span
            return Err(Error::ExtraData(consumed..input.len()));
        }
        Ok(pattern)
    }

    /// Parses a pattern from the beginning of a string and returns both
    /// the parsed Pattern and the number of bytes consumed.
    ///
    /// Unlike `parse()`, this function succeeds even if additional
    /// characters follow the first pattern. The returned index points to the
    /// first unparsed character after the pattern.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use dcbor_pattern::Pattern;
    /// let (pattern, consumed) = Pattern::parse_partial("true rest").unwrap();
    /// assert_eq!(pattern, Pattern::bool(true));
    /// assert_eq!(consumed, 5); // "true ".len() - includes whitespace
    /// ```
    pub fn parse_partial(input: &str) -> Result<(Self, usize)> {
        use logos::Logos;

        use crate::parse::{Token, meta::parse_or};

        let mut lexer = Token::lexer(input);
        let pattern = parse_or(&mut lexer)?;

        // Calculate consumed bytes - much simpler than current approach!
        let consumed = match lexer.next() {
            Some(_) => lexer.span().start,
            None => input.len(),
        };

        Ok((pattern, consumed))
    }
}

impl TryFrom<&str> for Pattern {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::parse(value)
    }
}

impl Matcher for Pattern {
    fn paths_with_captures(
        &self,
        haystack: &CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        // Collect all capture names from this pattern
        let mut capture_names = Vec::new();
        self.collect_capture_names(&mut capture_names);

        // If no captures, use the faster direct path matching
        if capture_names.is_empty() {
            return (self.paths(haystack), std::collections::HashMap::new());
        }

        // For certain pattern types, delegate directly to their
        // paths_with_captures
        match self {
            Pattern::Meta(pattern) => {
                // Meta patterns like SearchPattern handle their own capture
                // logic
                return pattern.paths_with_captures(haystack);
            }
            Pattern::Structure(pattern) => {
                // Structure patterns like ArrayPattern handle their own capture
                // logic, including special handling for SequencePattern
                return pattern.paths_with_captures(haystack);
            }
            _ => {
                // Use VM for other pattern types that need it
            }
        }

        // Compile pattern to VM program for capture-aware matching
        let mut code = Vec::new();
        let mut literals = Vec::new();
        let mut captures = Vec::new();

        self.compile(&mut code, &mut literals, &mut captures);
        code.push(crate::pattern::vm::Instr::Accept);

        let program = crate::pattern::vm::Program {
            code,
            literals,
            capture_names: captures,
        };

        // Run VM to get paths and captures
        crate::pattern::vm::run(&program, haystack)
    }

    fn paths(&self, haystack: &CBOR) -> Vec<Path> {
        match self {
            Pattern::Value(pattern) => pattern.paths(haystack),
            Pattern::Structure(pattern) => pattern.paths(haystack),
            Pattern::Meta(pattern) => pattern.paths(haystack),
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

    /// Recursively collect all capture names from this pattern.
    fn collect_capture_names(&self, names: &mut Vec<String>) {
        match self {
            Pattern::Value(_) => {
                // Value patterns don't contain captures
            }
            Pattern::Structure(pattern) => {
                pattern.collect_capture_names(names);
            }
            Pattern::Meta(pattern) => {
                pattern.collect_capture_names(names);
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
            Pattern::Structure(pattern) => write!(f, "{}", pattern),
            Pattern::Meta(pattern) => write!(f, "{}", pattern),
        }
    }
}
