use super::{ArrayPattern, MapPattern, TaggedPattern};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructurePattern {
    Array(ArrayPattern),
    Map(MapPattern),
    Tagged(TaggedPattern),
}
