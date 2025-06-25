use super::{ArrayPattern, MapPattern, TaggedPattern};
use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructurePattern {
    Array(ArrayPattern),
    Map(MapPattern),
    Tagged(TaggedPattern),
}

impl Matcher for StructurePattern {
    fn paths(&self, cbor: &dcbor::CBOR) -> Vec<Path> {
        match self {
            StructurePattern::Array(pattern) => pattern.paths(cbor),
            StructurePattern::Map(pattern) => pattern.paths(cbor),
            StructurePattern::Tagged(pattern) => pattern.paths(cbor),
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        captures: &mut Vec<String>,
    ) {
        match self {
            StructurePattern::Array(pattern) => {
                pattern.compile(code, literals, captures)
            }
            StructurePattern::Map(pattern) => {
                pattern.compile(code, literals, captures)
            }
            StructurePattern::Tagged(pattern) => {
                pattern.compile(code, literals, captures)
            }
        }
    }

    fn collect_capture_names(&self, names: &mut Vec<String>) {
        match self {
            StructurePattern::Array(pattern) => {
                pattern.collect_capture_names(names)
            }
            StructurePattern::Map(pattern) => {
                pattern.collect_capture_names(names)
            }
            StructurePattern::Tagged(pattern) => {
                pattern.collect_capture_names(names)
            }
        }
    }
}

impl std::fmt::Display for StructurePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructurePattern::Array(pattern) => write!(f, "{}", pattern),
            StructurePattern::Map(pattern) => write!(f, "{}", pattern),
            StructurePattern::Tagged(pattern) => write!(f, "{}", pattern),
        }
    }
}
