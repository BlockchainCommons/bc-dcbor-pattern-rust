//! # Format Module for dcbor-pattern
//!
//! This module provides formatting utilities for displaying paths returned by
//! pattern matching. Unlike `bc-envelope-pattern` which supports digest URs and
//! envelope URs, this module focuses on CBOR diagnostic notation formatting.
//!
//! ## Features
//!
//! - **Diagnostic formatting**: Format CBOR elements using either standard or
//!   flat diagnostic notation
//! - **Path indentation**: Automatically indent nested path elements
//! - **Truncation support**: Optionally truncate long representations with
//!   ellipsis
//! - **Flexible options**: Choose whether to show all elements or just the
//!   final destination
//!
//! ## Usage
//!
//! ```rust
//! use dcbor::prelude::*;
//! use dcbor_pattern::{
//!     FormatPathsOpts, PathElementFormat, format_paths, format_paths_opt,
//! };
//!
//! // Create a path (normally this would come from pattern matching)
//! let path = vec![
//!     CBOR::from(42),
//!     CBOR::from("hello"),
//!     CBOR::from(vec![1, 2, 3]),
//! ];
//!
//! // Default formatting (indented, full diagnostic)
//! println!("{}", format_paths(&[path.clone()]));
//!
//! // Custom formatting options
//! let opts = FormatPathsOpts::new()
//!     .element_format(PathElementFormat::DiagnosticFlat(Some(20)))
//!     .last_element_only(true);
//! println!("{}", format_paths_opt(&[path], opts));
//! ```

#![allow(dead_code)]

use crate::Path;

/// A builder that provides formatting options for each path element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathElementFormat {
    /// Diagnostic format, with optional maximum length for truncation.
    Diagnostic(Option<usize>),
    /// Flat diagnostic format (single line), with optional maximum length for
    /// truncation.
    DiagnosticFlat(Option<usize>),
}

impl Default for PathElementFormat {
    fn default() -> Self { PathElementFormat::Diagnostic(None) }
}

/// Options for formatting paths.
#[derive(Debug, Clone)]
pub struct FormatPathsOpts {
    /// Whether to indent each path element.
    /// If true, each element will be indented by 4 spaces per level.
    indent: bool,

    /// Format for each path element.
    /// Default is `PathElementFormat::Diagnostic(None)`.
    element_format: PathElementFormat,

    /// If true, only the last element of each path will be formatted.
    /// This is useful for displaying only the final destination of a path.
    /// If false, all elements will be formatted.
    last_element_only: bool,
}

impl Default for FormatPathsOpts {
    /// Returns the default formatting options:
    /// - `indent`: true
    /// - `element_format`: PathElementFormat::Diagnostic(None)
    /// - `last_element_only`: false
    fn default() -> Self {
        Self {
            indent: true,
            element_format: PathElementFormat::default(),
            last_element_only: false,
        }
    }
}

impl FormatPathsOpts {
    /// Creates a new FormatPathsOpts with default values.
    pub fn new() -> Self { Self::default() }

    /// Sets whether to indent each path element.
    /// If true, each element will be indented by 4 spaces per level.
    pub fn indent(mut self, indent: bool) -> Self {
        self.indent = indent;
        self
    }

    /// Sets the format for each path element.
    /// Default is `PathElementFormat::Diagnostic(None)`.
    pub fn element_format(mut self, format: PathElementFormat) -> Self {
        self.element_format = format;
        self
    }

    /// Sets whether to format only the last element of each path.
    /// If true, only the last element will be formatted.
    /// If false, all elements will be formatted.
    pub fn last_element_only(mut self, last_element_only: bool) -> Self {
        self.last_element_only = last_element_only;
        self
    }
}

impl AsRef<FormatPathsOpts> for FormatPathsOpts {
    fn as_ref(&self) -> &FormatPathsOpts { self }
}

/// Format a single CBOR element according to the specified format.
fn format_cbor_element(
    cbor: &dcbor::CBOR,
    format: PathElementFormat,
) -> String {
    match format {
        PathElementFormat::Diagnostic(max_length) => {
            let diagnostic = cbor.diagnostic();
            truncate_with_ellipsis(&diagnostic, max_length)
        }
        PathElementFormat::DiagnosticFlat(max_length) => {
            let diagnostic = cbor.diagnostic_flat();
            truncate_with_ellipsis(&diagnostic, max_length)
        }
    }
}

/// Truncates a string to the specified maximum length, appending an ellipsis if
/// truncated. If `max_length` is None, returns the original string.
fn truncate_with_ellipsis(s: &str, max_length: Option<usize>) -> String {
    match max_length {
        Some(max_len) if s.len() > max_len => {
            if max_len > 1 {
                format!("{}…", &s[0..(max_len - 1)])
            } else {
                "…".to_string()
            }
        }
        _ => s.to_string(),
    }
}

/// Format each path element on its own line, each line successively indented by
/// 4 spaces. Options can be provided to customize the formatting.
pub fn format_path_opt(
    path: &Path,
    opts: impl AsRef<FormatPathsOpts>,
) -> String {
    let opts = opts.as_ref();

    if opts.last_element_only {
        // Only format the last element, no indentation.
        if let Some(element) = path.iter().last() {
            format_cbor_element(element, opts.element_format)
        } else {
            String::new()
        }
    } else {
        match opts.element_format {
            PathElementFormat::Diagnostic(_)
            | PathElementFormat::DiagnosticFlat(_) => {
                // Multi-line output with indentation for diagnostic formats.
                let mut lines = Vec::new();
                for (index, element) in path.iter().enumerate() {
                    let indent = if opts.indent {
                        " ".repeat(index * 4)
                    } else {
                        String::new()
                    };

                    let content =
                        format_cbor_element(element, opts.element_format);
                    lines.push(format!("{}{}", indent, content));
                }
                lines.join("\n")
            }
        }
    }
}

/// Format each path element on its own line, each line successively indented by
/// 4 spaces.
pub fn format_path(path: &Path) -> String {
    format_path_opt(path, FormatPathsOpts::default())
}

/// Format multiple paths with custom formatting options.
pub fn format_paths_opt(
    paths: &[Path],
    opts: impl AsRef<FormatPathsOpts>,
) -> String {
    let opts = opts.as_ref();

    // Join all formatted paths with a newline for all formats.
    paths
        .iter()
        .map(|path| format_path_opt(path, opts))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Format multiple paths with default options.
pub fn format_paths(paths: &[Path]) -> String {
    format_paths_opt(paths, FormatPathsOpts::default())
}

#[cfg(test)]
mod tests {
    use dcbor::prelude::*;

    use super::*;

    fn create_test_path() -> Path {
        vec![
            CBOR::from(42),
            CBOR::from("test"),
            CBOR::from(vec![1, 2, 3]),
        ]
    }

    #[test]
    fn test_format_path_default() {
        let path = create_test_path();
        let formatted = format_path(&path);

        // Should have indentation and default diagnostic format
        assert!(formatted.contains("42"));
        assert!(formatted.contains("\"test\""));
        assert!(formatted.contains("[1, 2, 3]"));
    }

    #[test]
    fn test_format_path_flat() {
        let path = create_test_path();
        let opts = FormatPathsOpts::new()
            .element_format(PathElementFormat::DiagnosticFlat(None));
        let formatted = format_path_opt(&path, opts);

        // Should format with flat diagnostic
        assert!(formatted.contains("42"));
        assert!(formatted.contains("\"test\""));
        assert!(formatted.contains("[1, 2, 3]"));
    }

    #[test]
    fn test_format_path_last_element_only() {
        let path = create_test_path();
        let opts = FormatPathsOpts::new().last_element_only(true);
        let formatted = format_path_opt(&path, opts);

        // Should only contain the last element
        assert!(!formatted.contains("42"));
        assert!(!formatted.contains("\"test\""));
        assert!(formatted.contains("[1, 2, 3]"));
    }

    #[test]
    fn test_truncate_with_ellipsis() {
        assert_eq!(truncate_with_ellipsis("hello", None), "hello");
        assert_eq!(truncate_with_ellipsis("hello", Some(10)), "hello");
        assert_eq!(truncate_with_ellipsis("hello world", Some(5)), "hell…");
        assert_eq!(truncate_with_ellipsis("hello", Some(1)), "…");
    }

    #[test]
    fn test_format_paths_multiple() {
        let path1 = vec![CBOR::from(1)];
        let path2 = vec![CBOR::from(2)];
        let paths = vec![path1, path2];

        let formatted = format_paths(&paths);
        let lines: Vec<&str> = formatted.split('\n').collect();

        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("1"));
        assert!(lines[1].contains("2"));
    }
}
