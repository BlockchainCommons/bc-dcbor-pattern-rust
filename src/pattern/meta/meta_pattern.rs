use super::{AndPattern, CapturePattern, NotPattern, OrPattern, RepeatPattern, SearchPattern};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaPattern {
    Any,
    None,
    And(AndPattern),
    Or(OrPattern),
    Not(NotPattern),
    Repeat(RepeatPattern),
    Capture(CapturePattern),
    Search(SearchPattern),
}
