# dcbor-pattern Crate Documentation

## Overview

This crate provides pattern matching and text syntax parsing for Deterministic CBOR (dCBOR) as implemented in the `dcbor` crate. It supports complex pattern matching with named captures, search patterns, and nested structures.

The crate is ready for community review, with complete functionality and comprehensive test coverage.

### Usage Example
```rust
use dcbor_pattern::{Matcher, Pattern, format_paths_with_captures};

// Parse a pattern with named captures
let pattern = Pattern::parse("@name(SEQUENCE(NUMBER, TEXT))")?;

// Match against CBOR data and collect captures
let (paths, captures) = pattern.paths_with_captures(&cbor_data);

// Format results with structured capture display
let formatted = format_paths_with_captures(&paths, &captures, Default::default());
```

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

## Current Implementation Status

### ✅ COMPLETED: Map Bracket Syntax Change

**Change**: Replace `MAP(x)` syntax with `{ x }` where `x` is anything that can appear in the parentheses

**Status**: COMPLETED - All tests passing

**Implementation Details**:
- Added `BraceOpen` and `BraceClose` tokens to lexer for `{` and `}` syntax
- Created new map-specific bracket parser (`bracket_map_parser.rs`) that handles `{ }` syntax
- Modified lexer to handle token priority ambiguity between range quantifiers and map patterns
- Updated `MapPattern::Display` to format using bracket syntax `{ }`
- Updated all affected tests to use new bracket syntax
- Updated `PatternSyntax.md` documentation with new syntax examples

**Files Modified**:
- `src/parse/token.rs` (added BraceOpen/BraceClose tokens, fixed Range token regex)
- `src/parse/structure/bracket_map_parser.rs` (new file)
- `src/parse/structure/mod.rs`
- `src/parse/meta/primary_parser.rs`
- `src/pattern/structure/map_pattern.rs`
- `src/error.rs`
- `docs/PatternSyntax.md`
- Multiple test files

**Key Technical Points**:
- Bracket map parser handles `{*}`, `{n}`, `{n,m}`, `{n,}`, and `{pattern:pattern, ...}` syntax
- Lexer disambiguates between quantifier ranges `{n,m}` and map patterns by context
- Range token regex made more selective to avoid conflicts
- Primary parser handles Range tokens at top level, mapping them to map patterns
- Context-aware display formatting in `MapPattern::Display`

**New Map Syntax Examples**:
- Old: `MAP` → New: `{*}` (matches any map)
- Old: `MAP({3})` → New: `{{3}}` (exactly 3 entries)
- Old: `MAP({3,5})` → New: `{{3,5}}` (between 3 and 5 entries)
- Old: `MAP({3,})` → New: `{{3,}}` (at least 3 entries)
- Old: `MAP("key": "value")` → New: `{"key": "value"}` (specific key-value pairs)

### ✅ COMPLETED: Array Sequence Operator Change

**Change**: Replace `>` sequence operator with `,` in ArrayPattern

**Status**: COMPLETED - All tests passing

**Implementation Details**:
- Created new array-specific sequence parser (`array_sequence_parser.rs`) that uses comma `,` instead of `>` for sequences within arrays
- Modified `array_parser.rs` to use the new array-specific parsing hierarchy
- Updated `ArrayPattern::Display` to format sequences with commas in array context
- Updated all affected tests (11+ test files) to use new comma syntax
- Updated `PatternSyntax.md` documentation with new syntax examples

**Files Modified**:
- `src/parse/structure/array_sequence_parser.rs` (new file)
- `src/parse/structure/mod.rs`
- `src/parse/structure/array_parser.rs`
- `src/pattern/structure/array_pattern.rs`
- `docs/PatternSyntax.md`
- Multiple test files

**Key Technical Points**:
- Array-specific parser hierarchy: `parse_array_or` → `parse_array_and` → `parse_array_not` → `parse_array_sequence`
- Context-aware display formatting in `ArrayPattern::format_array_element_pattern()`
- Sequence patterns outside arrays still use `>` (maintained backward compatibility for non-array contexts)
- New bracket array parser handles `[pattern]` syntax with comma-separated sequences

### ✅ COMPLETED SYNTAX CHANGES

**All syntax changes have been completed**:

1. ✅ **COMPLETED**: Change `ARRAY( x )` to `[ x ]` where `x` is anything that can appear in the parentheses
2. ✅ **COMPLETED**: Change `ARRAY` by itself to `[*]`
3. ✅ **COMPLETED**: Change `MAP(x)` to `{ x }` where `x` is anything that can appear in the parentheses
4. ✅ **COMPLETED**: Change `MAP` by itself to `{*}`

**Array Bracket Syntax (COMPLETED)**:
- ✅ Implemented bracket tokens `[` and `]` in lexer
- ✅ Created bracket array parser (`bracket_array_parser.rs`)
- ✅ Integrated with main parser in `primary_parser.rs`
- ✅ Updated `ArrayPattern::Display` to use bracket syntax
- ✅ Added support for comma-separated sequences in arrays
- ✅ Updated all tests to use bracket syntax
- ✅ Updated documentation (`docs/PatternSyntax.md`)

**New Array Syntax Examples**:
- Old: `ARRAY` → New: `[*]` (matches any array)
- Old: `ARRAY({3})` → New: `[{3}]` (exactly 3 elements)
- Old: `ARRAY(NUMBER(42))` → New: `[NUMBER(42)]` (single element)
- Old: `ARRAY(TEXT("a"), TEXT("b"))` → New: `[TEXT("a"), TEXT("b")]` (sequence)
- Old: `ARRAY((ANY)*, NUMBER(42))` → New: `[(ANY)*, NUMBER(42)]` (with repeats)

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
