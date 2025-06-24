use super::{ArrayPattern, MapPattern, TaggedPattern};
use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructurePattern {
    Array(ArrayPattern),
    Map(MapPattern),
    Tagged(TaggedPattern),
}

impl Matcher for StructurePattern {
    fn paths(&self, _cbor: &dcbor::CBOR) -> Vec<Path> {
        // TODO: Implement structure pattern matching
        vec![]
    }

    fn compile(
        &self,
        _code: &mut Vec<Instr>,
        _literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        // TODO: Implement structure pattern compilation
        unimplemented!("StructurePattern::compile not yet implemented")
    }
}

impl std::fmt::Display for StructurePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructurePattern::Array(_) => write!(f, "array"),
            StructurePattern::Map(_) => write!(f, "map"),
            StructurePattern::Tagged(_) => write!(f, "tagged"),
        }
    }
}
