use crate::pattern::{
    meta::MetaPattern, structure::StructurePattern, value::ValuePattern,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Value(ValuePattern),
    Structure(StructurePattern),
    Meta(MetaPattern),
}
