use super::{
    BoolPattern, ByteStringPattern, DatePattern, DigestPattern,
    KnownValuePattern, NullPattern, NumberPattern, TextPattern,
};

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
