use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching CBOR tagged value structures.
#[derive(Debug, Clone)]
pub enum TaggedPattern {
    /// Matches any tagged value.
    Any,
    /// Matches tagged values with the specific tag.
    WithTag(Tag),
    /// Matches tagged values with tags in the given set.
    WithTagSet(Vec<Tag>),
    /// Matches tagged values with content that matches the given pattern.
    WithContent(Box<Pattern>),
    /// Matches tagged values with specific tag AND content that matches the
    /// pattern.
    WithTagAndContent {
        tag: Tag,
        content_pattern: Box<Pattern>,
    },
    /// Matches tagged values with a tag having the given name.
    WithTagName(String),
    /// Matches tagged values with a tag name that matches the given regex AND
    /// content that matches the pattern.
    WithTagNameRegex(regex::Regex),
    /// Matches tagged values with a tag having the given name AND content that
    /// matches the pattern.
    WithTagNameAndContent {
        tag_name: String,
        content_pattern: Box<Pattern>,
    },
    /// Matches tagged values with a tag name that matches the given regex AND
    /// content that matches the pattern.
    WithTagNameRegexAndContent {
        tag_regex: regex::Regex,
        content_pattern: Box<Pattern>,
    },
}

impl TaggedPattern {
    /// Creates a new `TaggedPattern` that matches any tagged value.
    pub fn any() -> Self { TaggedPattern::Any }

    /// Creates a new `TaggedPattern` that matches tagged values with the
    /// specific tag.
    pub fn with_tag(tag: Tag) -> Self { TaggedPattern::WithTag(tag) }

    /// Creates a new `TaggedPattern` that matches tagged values with tags in
    /// the given set.
    pub fn with_tag_set(tags: Vec<Tag>) -> Self {
        TaggedPattern::WithTagSet(tags)
    }

    /// Creates a new `TaggedPattern` that matches tagged values with content
    /// that matches the given pattern.
    pub fn with_content(pattern: Pattern) -> Self {
        TaggedPattern::WithContent(Box::new(pattern))
    }

    /// Creates a new `TaggedPattern` that matches tagged values with specific
    /// tag AND content that matches the pattern.
    pub fn with_tag_and_content(tag: Tag, content_pattern: Pattern) -> Self {
        TaggedPattern::WithTagAndContent {
            tag,
            content_pattern: Box::new(content_pattern),
        }
    }

    /// Creates a new `TaggedPattern` that matches tagged values with a tag
    /// having the given name.
    pub fn with_tag_name(tag_name: String) -> Self {
        TaggedPattern::WithTagName(tag_name)
    }

    /// Creates a new `TaggedPattern` that matches tagged values with a tag name
    /// that matches the given regex.
    pub fn with_tag_name_regex(regex: regex::Regex) -> Self {
        TaggedPattern::WithTagNameRegex(regex)
    }

    /// Creates a new `TaggedPattern` that matches tagged values with a tag
    /// having the given name AND content that matches the pattern.
    pub fn with_tag_name_and_content(
        tag_name: String,
        content_pattern: Pattern,
    ) -> Self {
        TaggedPattern::WithTagNameAndContent {
            tag_name,
            content_pattern: Box::new(content_pattern),
        }
    }

    /// Creates a new `TaggedPattern` that matches tagged values with a tag name
    /// that matches the given regex AND content that matches the pattern.
    pub fn with_tag_name_regex_and_content(
        tag_regex: regex::Regex,
        content_pattern: Pattern,
    ) -> Self {
        TaggedPattern::WithTagNameRegexAndContent {
            tag_regex,
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
                        // Match any tagged value - return the tagged value
                        // itself
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
                    TaggedPattern::WithTagAndContent {
                        tag: target_tag,
                        content_pattern,
                    } => {
                        if tag == target_tag && content_pattern.matches(content)
                        {
                            vec![vec![cbor.clone()]]
                        } else {
                            vec![]
                        }
                    }
                    TaggedPattern::WithTagName(target_name) => {
                        // Look up the tag name and compare
                        if let Some(name) = tag.name() {
                            if name.as_str() == target_name {
                                vec![vec![cbor.clone()]]
                            } else {
                                vec![]
                            }
                        } else {
                            vec![]
                        }
                    }
                    TaggedPattern::WithTagNameRegex(regex) => {
                        // Check if tag name matches the regex
                        if let Some(name) = tag.name() {
                            if regex.is_match(name.as_str()) {
                                vec![vec![cbor.clone()]]
                            } else {
                                vec![]
                            }
                        } else {
                            vec![]
                        }
                    }
                    TaggedPattern::WithTagNameAndContent {
                        tag_name,
                        content_pattern,
                    } => {
                        if let Some(name) = tag.name() {
                            if name.as_str() == tag_name
                                && content_pattern.matches(content)
                            {
                                vec![vec![cbor.clone()]]
                            } else {
                                vec![]
                            }
                        } else {
                            vec![]
                        }
                    }
                    TaggedPattern::WithTagNameRegexAndContent {
                        tag_regex,
                        content_pattern,
                    } => {
                        if let Some(name) = tag.name() {
                            if tag_regex.is_match(name.as_str())
                                && content_pattern.matches(content)
                            {
                                vec![vec![cbor.clone()]]
                            } else {
                                vec![]
                            }
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
            crate::pattern::StructurePattern::Tagged(self.clone()),
        ));
        code.push(Instr::MatchStructure(idx));
    }

    fn collect_capture_names(&self, names: &mut Vec<String>) {
        match self {
            TaggedPattern::Any => {
                // No captures in a simple any pattern
            }
            TaggedPattern::WithTag(_) => {
                // No captures in tag-only patterns
            }
            TaggedPattern::WithTagSet(_) => {
                // No captures in tag set patterns
            }
            TaggedPattern::WithContent(content_pattern) => {
                // Collect captures from the content pattern
                content_pattern.collect_capture_names(names);
            }
            TaggedPattern::WithTagAndContent { content_pattern, .. } => {
                // Collect captures from the content pattern
                content_pattern.collect_capture_names(names);
            }
            TaggedPattern::WithTagName(_) => {
                // No captures in tag name patterns
            }
            TaggedPattern::WithTagNameRegex(_) => {
                // No captures in tag name regex patterns
            }
            TaggedPattern::WithTagNameAndContent { content_pattern, .. } => {
                // Collect captures from the content pattern
                content_pattern.collect_capture_names(names);
            }
            TaggedPattern::WithTagNameRegexAndContent { content_pattern, .. } => {
                // Collect captures from the content pattern
                content_pattern.collect_capture_names(names);
            }
        }
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
                let tag_values: Vec<String> =
                    tags.iter().map(|t| t.value().to_string()).collect();
                write!(f, "TAGGED_TAGS([{}])", tag_values.join(", "))
            }
            TaggedPattern::WithContent(pattern) => {
                write!(f, "TAGGED_CONTENT({})", pattern)
            }
            TaggedPattern::WithTagAndContent { tag, content_pattern } => {
                write!(f, "TAGGED_TC({}, {})", tag.value(), content_pattern)
            }
            TaggedPattern::WithTagName(name) => {
                write!(f, "TAGGED_NAME({})", name)
            }
            TaggedPattern::WithTagNameRegex(regex) => {
                write!(f, "TAGGED_REGEX({})", regex.as_str())
            }
            TaggedPattern::WithTagNameAndContent {
                tag_name,
                content_pattern,
            } => {
                write!(f, "TAGGED_NC({}, {})", tag_name, content_pattern)
            }
            TaggedPattern::WithTagNameRegexAndContent {
                tag_regex,
                content_pattern,
            } => {
                write!(
                    f,
                    "TAGGED_RC({}, {})",
                    tag_regex.as_str(),
                    content_pattern
                )
            }
        }
    }
}

impl PartialEq for TaggedPattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TaggedPattern::Any, TaggedPattern::Any) => true,
            (TaggedPattern::WithTag(a), TaggedPattern::WithTag(b)) => a == b,
            (TaggedPattern::WithTagSet(a), TaggedPattern::WithTagSet(b)) => {
                a == b
            }
            (TaggedPattern::WithContent(a), TaggedPattern::WithContent(b)) => {
                a == b
            }
            (
                TaggedPattern::WithTagAndContent {
                    tag: tag_a,
                    content_pattern: content_a,
                },
                TaggedPattern::WithTagAndContent {
                    tag: tag_b,
                    content_pattern: content_b,
                },
            ) => tag_a == tag_b && content_a == content_b,
            (TaggedPattern::WithTagName(a), TaggedPattern::WithTagName(b)) => {
                a == b
            }
            (
                TaggedPattern::WithTagNameRegex(a),
                TaggedPattern::WithTagNameRegex(b),
            ) => a.as_str() == b.as_str(),
            (
                TaggedPattern::WithTagNameAndContent {
                    tag_name: name_a,
                    content_pattern: content_a,
                },
                TaggedPattern::WithTagNameAndContent {
                    tag_name: name_b,
                    content_pattern: content_b,
                },
            ) => name_a == name_b && content_a == content_b,
            (
                TaggedPattern::WithTagNameRegexAndContent {
                    tag_regex: regex_a,
                    content_pattern: content_a,
                },
                TaggedPattern::WithTagNameRegexAndContent {
                    tag_regex: regex_b,
                    content_pattern: content_b,
                },
            ) => regex_a.as_str() == regex_b.as_str() && content_a == content_b,
            _ => false,
        }
    }
}

impl Eq for TaggedPattern {}
