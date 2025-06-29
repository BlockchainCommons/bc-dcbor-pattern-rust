use logos::Span;
use thiserror::Error;

use crate::parse::Token;

/// Errors that can occur during parsing of dCBOR patterns.
#[derive(Debug, Clone, Error, PartialEq, Default)]
pub enum Error {
    #[error("Empty input")]
    EmptyInput,

    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("Extra data at end of input")]
    ExtraData(Span),

    #[error("Unexpected token {0:?}")]
    UnexpectedToken(Box<Token>, Span),

    #[error("Unrecognized token at position {0:?}")]
    UnrecognizedToken(Span),

    #[error("Invalid regex pattern at {0:?}")]
    InvalidRegex(Span),

    #[error("Unterminated regex pattern at {0:?}")]
    UnterminatedRegex(Span),

    #[error("Unterminated string literal at {0:?}")]
    UnterminatedString(Span),

    #[error("Invalid range at {0:?}")]
    InvalidRange(Span),

    #[error("Invalid hex string at {0:?}")]
    InvalidHexString(Span),

    #[error("Unterminated hex string at {0:?}")]
    UnterminatedHexString(Span),

    #[error("Invalid date format at {0:?}")]
    InvalidDateFormat(Span),

    #[error("Invalid number format at {0:?}")]
    InvalidNumberFormat(Span),

    #[error("Invalid UR: {0} at {1:?}")]
    InvalidUr(String, Span),

    #[error("Expected opening parenthesis")]
    ExpectedOpenParen(Span),

    #[error("Expected closing parenthesis")]
    ExpectedCloseParen(Span),

    #[error("Expected closing bracket")]
    ExpectedCloseBracket(Span),

    #[error("Expected closing brace")]
    ExpectedCloseBrace(Span),

    #[error("Expected colon")]
    ExpectedColon(Span),

    #[error("Expected pattern after operator")]
    ExpectedPattern(Span),

    #[error("Unmatched parentheses")]
    UnmatchedParentheses(Span),

    #[error("Unmatched braces")]
    UnmatchedBraces(Span),

    #[error("Invalid capture group name")]
    InvalidCaptureGroupName(String, Span),

    #[error("Invalid digest pattern: {0} at {1:?}")]
    InvalidDigestPattern(String, Span),

    #[error("Unterminated digest quoted pattern at {0:?}")]
    UnterminatedDigestQuoted(Span),

    #[error("Unterminated date quoted pattern at {0:?}")]
    UnterminatedDateQuoted(Span),

    #[error("Unknown error")]
    #[default]
    Unknown,
}

/// A Result type specialized for dCBOR pattern parsing.
pub type Result<T> = std::result::Result<T, Error>;
