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

impl Error {
    /// Adjusts the span of an error by adding the given offset to both start and end positions.
    /// Returns a new error with adjusted span, or the original error if it has no span.
    pub fn adjust_span(self, offset: usize) -> Self {
        match self {
            Error::UnrecognizedToken(span) => {
                Error::UnrecognizedToken(offset + span.start..offset + span.end)
            }
            Error::ExtraData(span) => {
                Error::ExtraData(offset + span.start..offset + span.end)
            }
            Error::UnexpectedToken(token, span) => Error::UnexpectedToken(
                token,
                offset + span.start..offset + span.end,
            ),
            Error::InvalidRegex(span) => {
                Error::InvalidRegex(offset + span.start..offset + span.end)
            }
            Error::UnterminatedRegex(span) => {
                Error::UnterminatedRegex(offset + span.start..offset + span.end)
            }
            Error::UnterminatedString(span) => Error::UnterminatedString(
                offset + span.start..offset + span.end,
            ),
            Error::InvalidRange(span) => {
                Error::InvalidRange(offset + span.start..offset + span.end)
            }
            Error::InvalidHexString(span) => {
                Error::InvalidHexString(offset + span.start..offset + span.end)
            }
            Error::UnterminatedHexString(span) => Error::UnterminatedHexString(
                offset + span.start..offset + span.end,
            ),
            Error::InvalidDateFormat(span) => {
                Error::InvalidDateFormat(offset + span.start..offset + span.end)
            }
            Error::InvalidNumberFormat(span) => Error::InvalidNumberFormat(
                offset + span.start..offset + span.end,
            ),
            Error::InvalidUr(msg, span) => {
                Error::InvalidUr(msg, offset + span.start..offset + span.end)
            }
            Error::ExpectedOpenParen(span) => {
                Error::ExpectedOpenParen(offset + span.start..offset + span.end)
            }
            Error::ExpectedCloseParen(span) => Error::ExpectedCloseParen(
                offset + span.start..offset + span.end,
            ),
            Error::ExpectedCloseBracket(span) => Error::ExpectedCloseBracket(
                offset + span.start..offset + span.end,
            ),
            Error::ExpectedCloseBrace(span) => Error::ExpectedCloseBrace(
                offset + span.start..offset + span.end,
            ),
            Error::ExpectedColon(span) => {
                Error::ExpectedColon(offset + span.start..offset + span.end)
            }
            Error::ExpectedPattern(span) => {
                Error::ExpectedPattern(offset + span.start..offset + span.end)
            }
            Error::UnmatchedParentheses(span) => Error::UnmatchedParentheses(
                offset + span.start..offset + span.end,
            ),
            Error::UnmatchedBraces(span) => {
                Error::UnmatchedBraces(offset + span.start..offset + span.end)
            }
            Error::InvalidCaptureGroupName(name, span) => {
                Error::InvalidCaptureGroupName(
                    name,
                    offset + span.start..offset + span.end,
                )
            }
            Error::InvalidDigestPattern(msg, span) => {
                Error::InvalidDigestPattern(
                    msg,
                    offset + span.start..offset + span.end,
                )
            }
            Error::UnterminatedDigestQuoted(span) => {
                Error::UnterminatedDigestQuoted(
                    offset + span.start..offset + span.end,
                )
            }
            Error::UnterminatedDateQuoted(span) => {
                Error::UnterminatedDateQuoted(
                    offset + span.start..offset + span.end,
                )
            }
            // For errors without spans, return them as-is
            _ => self,
        }
    }
}
