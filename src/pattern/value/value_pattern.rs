use super::{
    BoolPattern, ByteStringPattern, DatePattern, DigestPattern,
    KnownValuePattern, NullPattern, NumberPattern, TextPattern,
};
use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

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
    fn paths(&self, cbor: &dcbor::CBOR) -> Vec<Path> {
        match self {
            ValuePattern::Bool(pattern) => pattern.paths(cbor),
            ValuePattern::ByteString(pattern) => pattern.paths(cbor),
            ValuePattern::Date(pattern) => pattern.paths(cbor),
            ValuePattern::Digest(_pattern) => {
                // TODO: Implement when DigestPattern is ready
                unimplemented!("DigestPattern not yet implemented")
            }
            ValuePattern::KnownValue(_pattern) => {
                // TODO: Implement when KnownValuePattern is ready
                unimplemented!("KnownValuePattern not yet implemented")
            }
            ValuePattern::Null(pattern) => pattern.paths(cbor),
            ValuePattern::Number(pattern) => pattern.paths(cbor),
            ValuePattern::Text(pattern) => pattern.paths(cbor),
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
            ValuePattern::Digest(_pattern) => {
                unimplemented!("DigestPattern compile not yet implemented")
            }
            ValuePattern::KnownValue(_pattern) => {
                unimplemented!("KnownValuePattern compile not yet implemented")
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
            ValuePattern::Digest(pattern) => write!(f, "{:?}", pattern), /* Temporary */
            ValuePattern::KnownValue(pattern) => write!(f, "{:?}", pattern), /* Temporary */
            ValuePattern::Null(pattern) => write!(f, "{}", pattern),
            ValuePattern::Number(pattern) => write!(f, "{}", pattern),
            ValuePattern::Text(pattern) => write!(f, "{}", pattern),
        }
    }
}
