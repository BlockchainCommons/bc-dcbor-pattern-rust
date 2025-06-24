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
            ValuePattern::ByteString(_pattern) => {
                // TODO: Implement when ByteStringPattern is ready
                unimplemented!("ByteStringPattern not yet implemented")
            }
            ValuePattern::Date(_pattern) => {
                // TODO: Implement when DatePattern is ready
                unimplemented!("DatePattern not yet implemented")
            }
            ValuePattern::Digest(_pattern) => {
                // TODO: Implement when DigestPattern is ready
                unimplemented!("DigestPattern not yet implemented")
            }
            ValuePattern::KnownValue(_pattern) => {
                // TODO: Implement when KnownValuePattern is ready
                unimplemented!("KnownValuePattern not yet implemented")
            }
            ValuePattern::Null(_pattern) => {
                // TODO: Implement when NullPattern is ready
                unimplemented!("NullPattern not yet implemented")
            }
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
            ValuePattern::ByteString(_pattern) => {
                unimplemented!("ByteStringPattern compile not yet implemented")
            }
            ValuePattern::Date(_pattern) => {
                unimplemented!("DatePattern compile not yet implemented")
            }
            ValuePattern::Digest(_pattern) => {
                unimplemented!("DigestPattern compile not yet implemented")
            }
            ValuePattern::KnownValue(_pattern) => {
                unimplemented!("KnownValuePattern compile not yet implemented")
            }
            ValuePattern::Null(_pattern) => {
                unimplemented!("NullPattern compile not yet implemented")
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
            ValuePattern::ByteString(pattern) => write!(f, "{:?}", pattern), /* Temporary */
            ValuePattern::Date(pattern) => write!(f, "{:?}", pattern), /* Temporary */
            ValuePattern::Digest(pattern) => write!(f, "{:?}", pattern), /* Temporary */
            ValuePattern::KnownValue(pattern) => write!(f, "{:?}", pattern), /* Temporary */
            ValuePattern::Null(pattern) => write!(f, "{:?}", pattern), /* Temporary */
            ValuePattern::Number(pattern) => write!(f, "{}", pattern),
            ValuePattern::Text(pattern) => write!(f, "{}", pattern),
        }
    }
}
