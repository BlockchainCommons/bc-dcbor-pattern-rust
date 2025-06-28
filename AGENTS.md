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

## Planned Syntax Changes - Assessment and Development Plan

The following syntax changes have been requested:

1. Change the `>` sequence operator in `ArrayPattern` to a comma `,`
2. Change `ARRAY( x )` to `[ x ]` where `x` is anything that can appear in the parentheses
3. Change `ARRAY` by itself to `[ * ]`
4. Change `MAP( x )` to `{ x }` where `x` is anything that can appear in the parentheses
5. Change `MAP` by itself to `{ * }`

### Impact Assessment

**Complexity Level: HIGH**

This is a significant breaking change that will require modifications across multiple layers of the codebase, including lexer, parser, display formatting, and comprehensive test updates.

### Components Requiring Changes

#### 1. Lexical Analysis (`src/parse/token.rs`)
- **Current State**: Uses `ARRAY` and `MAP` keywords
- **Required Changes**:
  - Add new tokens: `BracketOpen` (`[`), `BracketClose` (`]`), `BraceOpen` (`{`), `BraceClose` (`}`), `Asterisk` (`*`) as structure wildcard
  - Modify existing `Range` token parser (currently handles `{n,m}` for quantifiers) to distinguish between range quantifiers and map syntax
  - Add token priorities to handle conflicts between map syntax `{}` and range quantifiers `{n,m}`
  - **Estimated Effort**: 2-3 days

#### 2. Parser Implementation (`src/parse/structure/`)
- **Current State**:
  - `array_parser.rs`: Handles `ARRAY` keyword-based parsing
  - `map_parser.rs`: Handles `MAP` keyword-based parsing
- **Required Changes**:
  - **Array Parser**: Complete rewrite to handle bracket syntax `[...]` instead of `ARRAY(...)`
  - **Map Parser**: Complete rewrite to handle brace syntax `{...}` instead of `MAP(...)`
  - **Sequence Parsing**: Modify array content parsing to use comma `,` instead of `>` for sequences
  - **Wildcard Handling**: Add parsing for `[*]` and `{*}` as wildcard patterns
  - **Primary Parser**: Update token routing in `primary_parser.rs` to handle new bracket/brace tokens
  - **Estimated Effort**: 5-7 days

#### 3. Pattern Display (`src/pattern/structure/`)
- **Current State**:
  - `ArrayPattern::Display` outputs `ARRAY`, `ARRAY({n})`, `ARRAY(pattern)` formats
  - `MapPattern::Display` outputs `MAP`, `MAP({n})`, `MAP(pattern)` formats
- **Required Changes**:
  - **Array Display**: Rewrite to output `[*]`, `[{n}]`, `[pattern]` formats
  - **Map Display**: Rewrite to output `{*}`, `{n}`, `{pattern}` formats
  - **Sequence Display**: Update sequence pattern display within arrays to use commas
  - **Estimated Effort**: 2-3 days

#### 4. Test Suite Updates
- **Current State**: Extensive test coverage using old syntax throughout 15+ test files
- **Required Changes**:
  - Update all pattern strings in tests to use new syntax
  - Update expected output strings to match new display format
  - Verify backward compatibility handling (if any)
  - **Files requiring updates**:
    - `tests/array_detailed_tests.rs`
    - `tests/array_capture_tests.rs`
    - `tests/map_pattern_integration_tests.rs`
    - `tests/map_capture_tests.rs`
    - `tests/test_complex_array_parsing.rs`
    - `tests/pattern_tests_structure.rs`
    - And ~10 others containing ARRAY/MAP patterns
  - **Estimated Effort**: 4-5 days

#### 5. Documentation Updates
- **Current State**: `docs/PatternSyntax.md` documents current syntax
- **Required Changes**:
  - Update all syntax examples
  - Update grammar specification
  - Add migration guide from old to new syntax
  - **Estimated Effort**: 1-2 days

### Technical Challenges

#### 1. Token Ambiguity Resolution
**Challenge**: The `{` token is currently used for range quantifiers `{n,m}` and will also be used for map syntax `{pattern}`.

**Solution Approach**:
- Implement lookahead parsing to distinguish contexts
- Use different parsing paths based on what follows the opening brace
- Ensure `{*}` (map wildcard) doesn't conflict with `{n,m}` (quantifier range)

#### 2. Sequence Operator Context Sensitivity
**Challenge**: The `>` operator is currently used for sequences everywhere, but should become `,` only within array patterns.

**Solution Approach**:
- Add parser context state to track when inside array vs. other pattern contexts
- Maintain `>` for general sequence patterns outside arrays
- Use `,` only within array bracket contexts `[...]`

#### 3. Backward Compatibility
**Challenge**: This is a complete syntax breaking change.

**Options**:
1. **Clean Break**: Remove old syntax entirely (recommended)
2. **Dual Support**: Support both syntaxes temporarily (adds significant complexity)
3. **Migration Period**: Deprecate old syntax with warnings

**Recommendation**: Clean break approach - simpler implementation and clearer semantics.

### Development Phases

#### Phase 1: Lexer Updates (Days 1-3)
1. Add new bracket/brace tokens to `Token` enum
2. Update token priorities and parsing callbacks
3. Add wildcard `*` token handling
4. Update error handling for new token conflicts

#### Phase 2: Parser Core (Days 4-10)
1. Rewrite `array_parser.rs` for bracket syntax
2. Rewrite `map_parser.rs` for brace syntax
3. Update `primary_parser.rs` token routing
4. Implement context-sensitive sequence parsing
5. Add comprehensive parser error handling

#### Phase 3: Display Implementation (Days 11-13)
1. Update `ArrayPattern::Display` implementation
2. Update `MapPattern::Display` implementation
3. Ensure round-trip consistency (parse → display → parse)

#### Phase 4: Test Suite Migration (Days 14-18)
1. Update all test pattern strings systematically
2. Update expected output assertions
3. Add tests for edge cases and error conditions
4. Verify comprehensive coverage

#### Phase 5: Documentation (Days 19-20)
1. Update `PatternSyntax.md` with new grammar
2. Update `AGENTS.md` with new syntax examples
3. Update README and other documentation
4. Create migration guide

### Validation Strategy

1. **Parser Round-trip Testing**: Ensure `parse(pattern.to_string()) == pattern` for all patterns
2. **Comprehensive Regression Testing**: All existing functionality must work with new syntax
3. **Edge Case Coverage**: Test malformed patterns, ambiguous syntax, complex nesting
4. **Performance Verification**: Ensure no performance regression in parsing/matching

### Estimated Total Effort

**20-25 development days** for a complete, production-ready implementation with full test coverage.

### Risk Factors

- **High**: Token ambiguity resolution complexity
- **Medium**: Maintaining sequence parsing correctness across contexts
- **Medium**: Comprehensive test migration without introducing regressions
- **Low**: Display formatting consistency

### Recommendation

This is a substantial but well-scoped refactoring project. The changes are clean conceptually but require careful implementation across multiple layers. The bracket/brace syntax will make the pattern language more intuitive and familiar to users coming from JSON or other structured data formats.

The development should proceed incrementally with extensive testing at each phase to ensure no regressions are introduced.

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
