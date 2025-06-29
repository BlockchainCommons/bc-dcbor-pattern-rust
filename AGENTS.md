# dcbor-pattern Crate Documentation

## Overview

This crate provides pattern matching and text syntax parsing for Deterministic CBOR (dCBOR) as implemented in the `dcbor` crate. It supports complex pattern matching with named captures, search patterns, and nested structures.

The crate is ready for community review, with complete functionality and comprehensive test coverage.

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

- These are updates to `docs/PatternSyntax.md` to reflect the new simplified syntax.

- The new syntax generalizes the use of "prefixed single-quoted" patterns, e.g., `prefix'content'`. With no prefix, single-quoted patterns are understood as "known values" as defined below.

- Remember, all tests that expect multi-line output should use the rubric as shown in `tests/common/mod.rs` to ensure consistent formatting.

- As we complete each section, mark it as complete in this document, and update `docs/PatternSyntax.md` to reflect the new syntax.

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
- Number:
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
- Known Value
    - `known`
        - Matches any known value. (See the `known-values` crate for more information.)
    - `known'value'`
        - Matches the specified known value, which is a u64 value. dCBOR prints known values enclosed in single quotes, so we use that syntax here for familiarity.
    - `known'name'`
        - Matches the known value with the specified name. Again we use single quotes here for familiarity.
    - `known'/regex/'`
        - Matches a known value with a name that matches the specified regex. We do not use the single quotes here.


- Tagged
    - `tagged`
        - Matches any CBOR tagged value.
    - `tagged ( value, pattern )`
        - Matches the specified CBOR tagged value with content that matches the given pattern. The tag value is a u64 value, formatted as a bare integer with no delimiters apart from the enclosing parentheses.
    - `tagged ( name, pattern )`
        - Matches the CBOR tagged value with the specified name and content that matches the given pattern. The tag name is formatted as a bare alphanumeric string (including hyphens and underscores) with no delimiters apart from the enclosing parentheses.
    - `tagged ( /regex/, pattern )`
        - Matches a CBOR tagged value with a name that matches the specified regex and content that matches the given pattern.

#### Meta Patterns

- Any
    - `*`
        - A bare asterisk matches any value.
- None
    - Remove, as `!*` is a pattern that matches no values.
- Search
    - `search ( pattern )`
        - Visits every node in the CBOR tree, matching the specified pattern against each node.
