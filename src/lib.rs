//! # dCBOR Pattern Matching

mod error;
mod format;
mod interval;
mod parse;
mod pattern;
mod quantifier;
mod reluctance;

#[cfg(test)]
mod test_search_issue;

#[cfg(test)]
mod debug_complex_pattern;

#[cfg(test)]
mod search_captures_fix_test;

pub use error::*;
pub use format::*;
pub use interval::*;
pub use parse::*;
pub use pattern::*;
pub use quantifier::*;
pub use reluctance::*;
