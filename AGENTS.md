# dcbor-pattern Crate Documentation

## Overview

This crate provides pattern matching and text syntax parsing for Deterministic CBOR (dCBOR) as implemented in the `dcbor` crate. It supports complex pattern matching with named captures, search patterns, and nested structures.

The crate is believed to be ready for community review, with complete functionality and comprehensive test coverage.

### Usage Example
```rust
use dcbor_pattern::{Matcher, Pattern};

// Parse a pattern with named captures
let pattern = Pattern::parse("@name(SEQUENCE(NUMBER, TEXT))")?;

// Match against CBOR data and collect captures
let (paths, captures) = pattern.paths_with_captures(&cbor_data);
```

## Development Guidelines

- Use `r#""#` syntax for pattern strings with embedded quotes
- Ensure `cargo test` and `cargo clippy` pass before committing
- Avoid `as_case` and `CBORCase` where possible - use the full `dcbor` API

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
