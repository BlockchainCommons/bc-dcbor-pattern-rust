use dcbor::prelude::*;

use crate::pattern::{Matcher, Path, Pattern, vm::Instr};

/// Pattern for matching CBOR tagged value structures.
#[derive(Debug, Clone)]
pub enum TaggedPattern {
    /// Matches any tagged value.
    Any,
    /// Matches tagged values with specific tag AND content that matches the
    /// pattern.
    Tag {
        tag: Tag,
        pattern: Box<Pattern>,
    },
    Name {
        name: String,
        pattern: Box<Pattern>,
    },
    /// Matches tagged values with a tag name that matches the given regex AND
    /// content that matches the pattern.
    Regex {
        regex: regex::Regex,
        pattern: Box<Pattern>,
    },
}

impl PartialEq for TaggedPattern {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TaggedPattern::Any, TaggedPattern::Any) => true,
            (
                TaggedPattern::Tag { tag: tag_a, pattern: content_a },
                TaggedPattern::Tag { tag: tag_b, pattern: content_b },
            ) => tag_a == tag_b && content_a == content_b,
            (
                TaggedPattern::Name { name: name_a, pattern: content_a },
                TaggedPattern::Name { name: name_b, pattern: content_b },
            ) => name_a == name_b && content_a == content_b,
            (
                TaggedPattern::Regex { regex: regex_a, pattern: content_a },
                TaggedPattern::Regex { regex: regex_b, pattern: content_b },
            ) => regex_a.as_str() == regex_b.as_str() && content_a == content_b,
            _ => false,
        }
    }
}

impl Eq for TaggedPattern {}

impl TaggedPattern {
    /// Creates a new `TaggedPattern` that matches any tagged value.
    pub fn any() -> Self {
        TaggedPattern::Any
    }

    /// Creates a new `TaggedPattern` that matches tagged values with specific
    /// tag AND content that matches the pattern.
    pub fn with_tag(tag: impl Into<Tag>, content_pattern: Pattern) -> Self {
        TaggedPattern::Tag {
            tag: tag.into(),
            pattern: Box::new(content_pattern),
        }
    }

    /// Creates a new `TaggedPattern` that matches tagged values with a tag
    /// having the given name AND content that matches the pattern.
    pub fn with_tag_name(tag_name: String, content_pattern: Pattern) -> Self {
        TaggedPattern::Name {
            name: tag_name,
            pattern: Box::new(content_pattern),
        }
    }

    /// Creates a new `TaggedPattern` that matches tagged values with a tag name
    /// that matches the given regex AND content that matches the pattern.
    pub fn with_tag_name_regex(
        tag_regex: regex::Regex,
        content_pattern: Pattern,
    ) -> Self {
        TaggedPattern::Regex {
            regex: tag_regex,
            pattern: Box::new(content_pattern),
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
                    TaggedPattern::Tag {
                        tag: target_tag,
                        pattern: content_pattern,
                    } => {
                        if tag == target_tag && content_pattern.matches(content)
                        {
                            vec![vec![cbor.clone()]]
                        } else {
                            vec![]
                        }
                    }
                    TaggedPattern::Name {
                        name: tag_name,
                        pattern: content_pattern,
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
                    TaggedPattern::Regex {
                        regex: tag_regex,
                        pattern: content_pattern,
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
        captures: &mut Vec<String>,
    ) {
        // Collect capture names from inner patterns
        self.collect_capture_names(captures);

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
            TaggedPattern::Tag { pattern: content_pattern, .. } => {
                // Collect captures from the content pattern
                content_pattern.collect_capture_names(names);
            }
            TaggedPattern::Name { pattern: content_pattern, .. } => {
                // Collect captures from the content pattern
                content_pattern.collect_capture_names(names);
            }
            TaggedPattern::Regex { pattern: content_pattern, .. } => {
                // Collect captures from the content pattern
                content_pattern.collect_capture_names(names);
            }
        }
    }

    fn paths_with_captures(
        &self,
        cbor: &dcbor::CBOR,
    ) -> (Vec<Path>, std::collections::HashMap<String, Vec<Path>>) {
        // Check if this CBOR value is a tagged value
        let dcbor::CBORCase::Tagged(tag_value, content) = cbor.as_case() else {
            return (vec![], std::collections::HashMap::new());
        };

        match self {
            TaggedPattern::Any => {
                // Matches any tagged value, no captures
                (vec![vec![cbor.clone()]], std::collections::HashMap::new())
            }
            TaggedPattern::Tag {
                tag: expected_tag,
                pattern: content_pattern,
            } => {
                if *tag_value == *expected_tag {
                    // Match specific tag and check content with potential
                    // captures
                    let (content_paths, captures) =
                        content_pattern.paths_with_captures(content);
                    if !content_paths.is_empty() {
                        // Build paths that include the tagged value as root
                        let tagged_paths: Vec<Path> = content_paths
                            .iter()
                            .map(|content_path| {
                                let mut path = vec![cbor.clone()];
                                path.extend_from_slice(&content_path[1..]); // Skip the content's root
                                path
                            })
                            .collect();

                        // Update captures to include tagged value as root
                        let mut updated_captures =
                            std::collections::HashMap::new();
                        for (name, capture_paths) in captures {
                            let updated_paths: Vec<Path> = capture_paths
                                .iter()
                                .map(|_capture_path| {
                                    // For tagged patterns, the capture path
                                    // should be [tagged_value, content]
                                    vec![cbor.clone(), content.clone()]
                                })
                                .collect();
                            updated_captures.insert(name, updated_paths);
                        }

                        (tagged_paths, updated_captures)
                    } else {
                        (vec![], captures)
                    }
                } else {
                    (vec![], std::collections::HashMap::new())
                }
            }
            _ => {
                // For other variants, fall back to basic paths without captures
                (self.paths(cbor), std::collections::HashMap::new())
            }
        }
    }
}

impl std::fmt::Display for TaggedPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaggedPattern::Any => write!(f, "tagged"),
            TaggedPattern::Tag { tag, pattern: content_pattern } => {
                write!(f, "tagged({}, {})", tag.value(), content_pattern)
            }
            TaggedPattern::Name {
                name: tag_name,
                pattern: content_pattern,
            } => {
                write!(f, "tagged({}, {})", tag_name, content_pattern)
            }
            TaggedPattern::Regex {
                regex: tag_regex,
                pattern: content_pattern,
            } => {
                write!(
                    f,
                    "tagged(/{}/,  {})",
                    tag_regex.as_str(),
                    content_pattern
                )
            }
        }
    }
}
