use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching CBOR tagged value structures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaggedPattern {
    /// Matches any tagged value.
    Any,
    /// Matches tagged values with the specific tag.
    WithTag(Tag),
    /// Matches tagged values with tags in the given set.
    WithTagSet(Vec<Tag>),
    /// Matches tagged values with content that matches the given pattern.
    WithContent(Box<Pattern>),
    /// Matches tagged values with specific tag AND content that matches the pattern.
    WithTagAndContent {
        tag: Tag,
        content_pattern: Box<Pattern>,
    },
}

impl TaggedPattern {
    /// Creates a new `TaggedPattern` that matches any tagged value.
    pub fn any() -> Self {
        TaggedPattern::Any
    }

    /// Creates a new `TaggedPattern` that matches tagged values with the specific tag.
    pub fn with_tag(tag: Tag) -> Self {
        TaggedPattern::WithTag(tag)
    }

    /// Creates a new `TaggedPattern` that matches tagged values with tags in the given set.
    pub fn with_tag_set(tags: Vec<Tag>) -> Self {
        TaggedPattern::WithTagSet(tags)
    }

    /// Creates a new `TaggedPattern` that matches tagged values with content
    /// that matches the given pattern.
    pub fn with_content(pattern: Pattern) -> Self {
        TaggedPattern::WithContent(Box::new(pattern))
    }

    /// Creates a new `TaggedPattern` that matches tagged values with specific tag
    /// AND content that matches the pattern.
    pub fn with_tag_and_content(tag: Tag, content_pattern: Pattern) -> Self {
        TaggedPattern::WithTagAndContent {
            tag,
            content_pattern: Box::new(content_pattern),
        }
    }
}

impl Matcher for TaggedPattern {
    fn paths(&self, cbor: &CBOR) -> Vec<Path> {
        // First check if this is a tagged value
        match cbor.as_case() {
            CBORCase::Tagged(tag, content) => {
                match self {
                    TaggedPattern::Any => {
                        // Match any tagged value - return the tagged value itself
                        vec![vec![cbor.clone()]]
                    }
                    TaggedPattern::WithTag(target_tag) => {
                        if tag == target_tag {
                            vec![vec![cbor.clone()]]
                        } else {
                            vec![]
                        }
                    }
                    TaggedPattern::WithTagSet(tags) => {
                        if tags.contains(tag) {
                            vec![vec![cbor.clone()]]
                        } else {
                            vec![]
                        }
                    }
                    TaggedPattern::WithContent(pattern) => {
                        if pattern.matches(content) {
                            vec![vec![cbor.clone()]]
                        } else {
                            vec![]
                        }
                    }
                    TaggedPattern::WithTagAndContent { tag: target_tag, content_pattern } => {
                        if tag == target_tag && content_pattern.matches(content) {
                            vec![vec![cbor.clone()]]
                        } else {
                            vec![]
                        }
                    }
                }
            }
            _ => {
                // Not a tagged value, no match
                vec![]
            }
        }
    }

    fn compile(
        &self,
        code: &mut Vec<Instr>,
        literals: &mut Vec<Pattern>,
        _captures: &mut Vec<String>,
    ) {
        let idx = literals.len();
        literals.push(Pattern::Structure(
            crate::pattern::StructurePattern::Tagged(self.clone())
        ));
        code.push(Instr::MatchStructure(idx));
    }
}

impl std::fmt::Display for TaggedPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaggedPattern::Any => write!(f, "TAGGED"),
            TaggedPattern::WithTag(tag) => {
                write!(f, "TAGGED_TAG({})", tag.value())
            }
            TaggedPattern::WithTagSet(tags) => {
                let tag_values: Vec<String> = tags.iter().map(|t| t.value().to_string()).collect();
                write!(f, "TAGGED_TAGS([{}])", tag_values.join(", "))
            }
            TaggedPattern::WithContent(pattern) => {
                write!(f, "TAGGED_CONTENT({})", pattern)
            }
            TaggedPattern::WithTagAndContent { tag, content_pattern } => {
                write!(f, "TAGGED_TC({}, {})", tag.value(), content_pattern)
            }
        }
    }
}
