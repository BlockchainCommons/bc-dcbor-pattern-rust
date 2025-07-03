use super::{
    BoolPattern, ByteStringPattern, DatePattern, DigestPattern,
    KnownValuePattern, NullPattern, NumberPattern, TextPattern,
};
use crate::pattern::{Matcher, Path, Pattern, vm::Instr};
use dcbor::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValuePattern {
    Bool(BoolPattern),
    ByteString(ByteStringPattern),
    Date(DatePattern),
    Digest(DigestPattern),
    KnownValue(KnownValuePattern),
    Null(NullPattern),
    Number(NumberPattern),
    Text(TextPattern),
}

impl Matcher for ValuePattern {
    fn paths(&self, haystack: &CBOR) -> Vec<Path> {
        match self {
            ValuePattern::Bool(pattern) => pattern.paths(haystack),
            ValuePattern::ByteString(pattern) => pattern.paths(haystack),
            ValuePattern::Date(pattern) => pattern.paths(haystack),
            ValuePattern::Digest(pattern) => pattern.paths(haystack),
            ValuePattern::KnownValue(pattern) => pattern.paths(haystack),
            ValuePattern::Null(pattern) => pattern.paths(haystack),
            ValuePattern::Number(pattern) => pattern.paths(haystack),
            ValuePattern::Text(pattern) => pattern.paths(haystack),
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        match self {
            ValuePattern::Bool(pattern) => {
                pattern.compile(code, literals, captures)
            }
            ValuePattern::ByteString(pattern) => {
                pattern.compile(code, literals, captures)
            }
            ValuePattern::Date(pattern) => {
                pattern.compile(code, literals, captures)
            }
            ValuePattern::Digest(pattern) => {
                pattern.compile(code, literals, captures)
            }
            ValuePattern::KnownValue(pattern) => {
                pattern.compile(code, literals, captures)
            }
            ValuePattern::Null(pattern) => {
                pattern.compile(code, literals, captures)
            }
            ValuePattern::Number(pattern) => {
                pattern.compile(code, literals, captures)
            }
            ValuePattern::Text(pattern) => {
                pattern.compile(code, literals, captures)
            }
        }
    }
}

impl std::fmt::Display for ValuePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValuePattern::Bool(pattern) => write!(f, "{}", pattern),
            ValuePattern::ByteString(pattern) => write!(f, "{}", pattern),
            ValuePattern::Date(pattern) => write!(f, "{}", pattern),
            ValuePattern::Digest(pattern) => write!(f, "{}", pattern),
            ValuePattern::KnownValue(pattern) => write!(f, "{}", pattern),
            ValuePattern::Null(pattern) => write!(f, "{}", pattern),
            ValuePattern::Number(pattern) => write!(f, "{}", pattern),
            ValuePattern::Text(pattern) => write!(f, "{}", pattern),
        }
    }
}
