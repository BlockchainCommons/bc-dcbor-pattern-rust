# dcbor-pattern Crate Documentation

## Overview

This crate provides pattern matching and text syntax parsing for Deterministic CBOR (dCBOR) as implemented in the `dcbor` crate. It supports complex pattern matching with named captures, search patterns, and nested structures.

The crate is ready for community review, with complete functionality and comprehensive test coverage.

## Development Plan: Pattern::parse_partial

### Feature Request: Partial Pattern Parsing

**Status**: ✅ COMPLETE - PHASE 1 IMPLEMENTED

**Motivation**:
Similar to how `dcbor-parse` provides `parse_dcbor_item_partial` which returns both the parsed CBOR item and the number of bytes consumed, we need analogous functionality for dcbor-pattern. This would enable parsing patterns from streams or strings where the pattern may be followed by other content.

**Current Status**:
✅ **IMPLEMENTED** - The `Pattern::parse_partial` method has been successfully implemented and tested. All functionality is working correctly with comprehensive test coverage.

**Current Limitation**:
The existing `Pattern::parse(input: &str)` method artificially restricts parsing by returning `Error::ExtraData` or `Error::UnrecognizedToken` when additional content follows a valid pattern. This is unnecessarily complex and limits use cases.

**Key Insight**:
After investigation, this is actually a **simplification** rather than a complex new feature. The lexer already knows when a pattern is complete - we just need to return the consumption information instead of treating "extra data" as an error.

### Implementation Plan

#### Phase 1: Core Implementation (Simple!)
1. **Add `parse_partial` method to Pattern**:
   ```rust
   impl Pattern {
       /// Parses a pattern from the beginning of a string and returns both
       /// the parsed Pattern and the number of bytes consumed.
       ///
       /// Unlike `parse()`, this function succeeds even if additional
       /// characters follow the first pattern.
       pub fn parse_partial(input: &str) -> Result<(Self, usize)> {
           use logos::Logos;
           use crate::parse::{Token, meta::parse_or};

           let mut lexer = Token::lexer(input);
           let pattern = parse_or(&mut lexer)?;

           // Calculate consumed bytes - much simpler than current approach!
           let consumed = match lexer.next() {
               Some(_) => lexer.span().start,
               None => input.len(),
           };

           Ok((pattern, consumed))
       }
   }
   ```

2. **Simplify existing `parse` method** (for backward compatibility):
   ```rust
   pub fn parse(input: &str) -> Result<Self> {
       let (pattern, consumed) = Self::parse_partial(input)?;
       if consumed < input.len() {
           // Find where we stopped to provide accurate error span
           let remaining_start = consumed;
           return Err(Error::ExtraData(remaining_start..input.len()));
       }
       Ok(pattern)
   }
   ```

#### Phase 2: Testing and Validation
1. **Unit tests**:
   ```rust
   #[test]
   fn test_parse_partial_basic() {
       let (pattern, consumed) = Pattern::parse_partial("true rest").unwrap();
       assert_eq!(pattern, Pattern::bool(true));
       assert_eq!(consumed, 4); // "true".len()
   }

   #[test]
   fn test_parse_full_compatibility() {
       // Existing behavior should still work
       assert!(Pattern::parse("true").is_ok());
       assert!(Pattern::parse("true extra").is_err()); // Still returns error for backward compatibility
   }
   ```

2. **Integration tests**:
   - Test various whitespace handling scenarios
   - Test complex nested patterns with trailing content
   - Verify error spans are accurate

3. **Documentation tests**:
   - Add comprehensive docstring examples
   - Ensure examples compile and run correctly

### Technical Notes

**Current Parsing Behavior Investigation**:
- `Pattern::parse("true extra")` → `Error::UnrecognizedToken` (because "extra" is not a valid token)
- `Pattern::parse("true false")` → `Error::ExtraData` (because "false" is a valid token after complete pattern)
- `Pattern::parse("42 |")` → `Error::UnexpectedEndOfInput` (because "|" expects another pattern)

**The Simplified Approach**:
Instead of the complex error checking in the current implementation:
```rust
match lexer.next() {
    None => Ok(pattern),
    Some(Ok(_)) => Err(Error::ExtraData(lexer.span())),
    Some(Err(e)) => { /* complex error handling */ }
}
```

We simply track consumption and let the caller decide what to do with remaining content:
```rust
let consumed = match lexer.next() {
    Some(_) => lexer.span().start,
    None => input.len(),
};
Ok((pattern, consumed))
```

### Technical Considerations

**Lexer State Management**:
- The `logos::Lexer` already provides accurate span information
- No complex state management needed - lexer handles token boundaries naturally
- Existing error reporting infrastructure works perfectly for partial parsing

**Error Reporting**:
- Error spans remain accurate since they're based on lexer position
- Backward compatibility maintained through the existing `parse()` method
- New `parse_partial()` method eliminates artificial restrictions

**Memory Efficiency**:
- No additional memory overhead - uses existing lexer infrastructure
- Zero-copy parsing where possible (already implemented)
- Simpler code path should improve performance

### Dependencies and Prerequisites

- No new external dependencies required
- Leverages existing `logos` lexer infrastructure
- Compatible with current `dcbor` and `dcbor-parse` versions
- Maintains API stability for existing `Pattern::parse()` method

### Success Criteria

- ✅ `Pattern::parse_partial` successfully parses patterns from partial strings
- ✅ Returns accurate byte consumption counts
- ✅ Maintains 100% backward compatibility with existing `parse()` method
- ✅ Comprehensive test coverage for new functionality
- ✅ Simple, maintainable implementation
- ✅ Performance improvement over current complex error checking

### Future Enhancements

- Consider adding convenience methods for common streaming scenarios
- Add examples showing integration with parser combinators
- Document best practices for partial parsing workflows

## Development Guidelines

- Use `r#""#` syntax for pattern strings with embedded quotes
- Ensure `cargo test` and `cargo clippy` pass before committing
- Avoid `as_case` and `CBORCase` where possible - use the full `dcbor` API
- Use 4 spaces for indentation in formatted output (consistent with dCBOR diagnostic notation)

## Architecture

### Pattern Types
- **Value Patterns**: Atomic CBOR values (bool, number, text, etc.)
- **Structure Patterns**: Compound structures (arrays, maps, tagged values)
- **Meta Patterns**: Logical combinations (and, or, not, captures, search, etc.)

### Key Components
- `Pattern`: Main enum with parsing and matching functionality
- `Matcher` trait: Core matching interface with `paths_with_captures()`
- VM: Pattern matching virtual machine for complex patterns
- Parser: Text syntax parser supporting full dCBOR pattern language

### Dependencies
- `dcbor`: Core deterministic CBOR implementation
- `dcbor-parse`: Diagnostic notation parser for test data
- `known-values`: Registry of well-known CBOR values
- `bc-components`: Blockchain Commons components (for digest patterns)

## Format Functions

### `format_paths_with_captures()`

The primary formatting function that outputs captures in a structured way:

```rust
use dcbor_pattern::{format_paths_with_captures, FormatPathsOpts};

let formatted = format_paths_with_captures(&paths, &captures, FormatPathsOpts::default());
```

**Output format example:**
```
@capture1
    path1 element1
        path1 element2
            path1 element3
    path2 element1
        path2 element2
            path2 element3
@capture2
    path1 element1
        path1 element2
            path1 element3
path1 element1
    path1 element2
        path1 element3
path2 element1
    path2 element2
        path2 element3
```

**Features:**
- Captures are displayed first, sorted lexicographically by name
- Capture names are prefixed with `@`
- Capture content is indented by 4 spaces
- Regular paths follow after all captures
- Consistent 4-space indentation throughout

### `format_paths()`

Convenience function that calls `format_paths_with_captures()` with empty captures:

```rust
use dcbor_pattern::format_paths;

let formatted = format_paths(&paths);
```

### Testing Integration

Tests now use `assert_actual_expected!` with `format_paths_with_captures()` for comprehensive validation of both paths and captures in a single assertion, making tests more concise and maintainable.

- To run all tests in a module: `cargo test --test <module>`
- To run an individual test: `cargo test --test <module> <test_name>`
- To run all tests: `cargo test`

### Current Task: Syntax Simplification

- These are updates to [docs/DCBORPatternSyntax.md](docs/dcbor_patex.md) to reflect the new simplified syntax.

- The new syntax generalizes the use of "prefixed single-quoted" patterns, e.g., `prefix'content'`. With no prefix, single-quoted patterns are understood as "known values" as defined below.

- Remember, all tests that expect multi-line output should use the rubric as shown in `tests/common/mod.rs` to ensure consistent formatting.

- As we complete each section, mark it as complete in this document, and update [docs/DCBORPatternSyntax.md](docs/dcbor_patex.md) to reflect the new syntax.

#### Value Patterns

- ✅ Boolean **COMPLETE**:
    - `bool`
        - Matches any boolean value.
    - `true`
        - Matches the boolean value `true`.
    - `false`
        - Matches the boolean value `false`.
- ✅ Text **COMPLETE**:
    - `text`
        - Matches any text value.
    - `"string"`
        - Matches a text value with the specified string. dCBOR diagnostic notation uses double quotes for text strings, so we use that syntax here for familiarity.
    - `/text-regex/`
        - Matches a text value that matches the specified regex. No double quotes are used here, as the regex is not a string but a pattern to match against the text value.
- ✅ Null **COMPLETE**:
    - `null`
        - Matches the null value.
- ✅ ByteString **COMPLETE**:
    - `bstr`
        - Matches any byte string.
    - `h'hex'`
        - Matches a byte string with the specified hex value. Note that the `h'...'` syntax is used to denote hex strings in CBOR diagnostic notation, so we use it here for familiarity.
    - `h'/regex/'`
        - Matches a byte string that matches the specified binary regex.
- ✅ Digest **COMPLETE**:
    - `digest`
        - Matches any digest value.
    - `digest'hex'`
        - Matches a digest whose value starts with the specified hex prefix. Up to 32 bytes can be specified, which is the length of the full SHA-256 digest.
    - `digest'ur:digest/value'`
        - Matches the specified `ur:digest` value, parsed using `Digest::from_ur_string()`.
    - `digest'/regex/'`
        - Matches a digest value that matches the specified binary regex.
- ✅ Date **COMPLETE**:
    - `date`
        - Matches any date value.
    - `date'iso-8601'`
        - Matches a date value with the specified ISO 8601 format. This is a bare string with no delimiters apart from the enclosing parentheses.
    - `date'iso-8601...iso-8601'`
        - Matches a date value within the specified range.
    - `date'iso-8601...'`
        - Matches a date value greater than or equal to the specified ISO 8601 date.
    - `date'...iso-8601'`
        - Matches a date value less than or equal to the specified ISO 8601 date.
    - `date'/regex/'`
        - Matches a date value that matches the specified regex.
- ✅ Number **COMPLETE**:
    - `number`
        - keyword `number` matches any number.
    - `value`
        - Bare numeric value matches the specified number.
    - `value...value`
        - Matches a number within the specified range.
    - `>=value`
        - Matches a number greater than or equal to the specified value.
    - `<=value`
        - Matches a number less than or equal to the specified value.
    - `>value`
        - Matches a number greater than the specified value.
    - `<value`
        - Matches a number less than the specified value.
    - `NaN`
        - Matches the NaN (Not a Number) value.
    - `Infinity`
        - Matches the Infinity value.
    - `-Infinity`
        - Matches the negative Infinity value.
- ✅ Known Value **COMPLETE**:
    - `known`
        - Matches any known value. (See the `known-values` crate for more information.)
    - `'value'`
        - Matches the specified known value, which is a u64 value. dCBOR prints known values enclosed in single quotes, so we use that syntax here for familiarity. Note: This is a non-prefixed single-quoted pattern.
    - `'name'`
        - Matches the known value with the specified name. Again we use single quotes here for familiarity. Note: This is a non-prefixed single-quoted pattern.
    - `'/regex/'`
        - Matches a known value with a name that matches the specified regex. We do not use the single quotes here. Note: This is a non-prefixed single-quoted pattern.


- ✅ Tagged **COMPLETE**:
    - `tagged`
        - Matches any CBOR tagged value.
    - `tagged ( value, pattern )`
        - Matches the specified CBOR tagged value with content that matches the given pattern. The tag value is a u64 value, formatted as a bare integer with no delimiters apart from the enclosing parentheses.
    - `tagged ( name, pattern )`
        - Matches the CBOR tagged value with the specified name and content that matches the given pattern. The tag name is formatted as a bare alphanumeric string (including hyphens and underscores) with no delimiters apart from the enclosing parentheses.
    - `tagged ( /regex/, pattern )`
        - Matches a CBOR tagged value with a name that matches the specified regex and content that matches the given pattern.

#### Meta Patterns

- ✅ Any **COMPLETE**:
    - `*`
        - A bare asterisk matches any value.
- ✅ **REMOVED**: None
    - The `NONE` syntax has been completely removed. Use `!*` instead, which provides identical behavior (matches no values).
- ✅ Search **COMPLETE**:
    - `search ( pattern )`
        - Visits every node in the CBOR tree, matching the specified pattern against each node.
