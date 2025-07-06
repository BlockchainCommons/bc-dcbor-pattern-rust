pub mod meta;
mod structure;
mod token;
pub mod value;

// pub use meta::*;
// pub use structure::*;
pub use token::*;

// pub use value::*;
use crate::{Error, Pattern, Result};

impl Pattern {
    /// Parse a pattern expression from a string.
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
