# `dcbor-pattern` Crate Documentation

This file contains general information about the `dcbor-pattern` crate, which provides a pattern matcher and text syntax pattern parser for Deterministic CBOR (dCBOR) as implemented in the `dcbor` crate in this workspace. Further documentation including the pattern expression syntax can be found in the `docs/` directory. Make sure to read those before starting on any tasks.

## General Guidance

You will be receiving tasks to implement the pattern matcher and text syntax parser for dCBOR.

- For pattern strings with embedded quotes or other special characters, use `r#""#` syntax to avoid awkward escaping issues.
- Always make sure that `cargo test` and `cargo clippy` pass before you're done with your changes.
- Avoid directly using `as_case` and `CBORCase` wherever possible. Make sure you understand the whole API in `dcbor` before you resort to using them.

## Crates in this Workspace

You will only be making changes to the `dcbor-pattern` crate, but it is important to understand the other crates in this workspace as they provide the context and dependencies for your work:

- `dcbor-pattern`: The crate you are currently working on, which provides the pattern matching and text syntax parsing functionality for dCBOR.
- `dcbor`: The core crate for deterministic CBOR, which provides the basic data structures and functionality for working with dCBOR values.
- `dcbor-parse`: A parser for dCBOR diagnostic notation, which is used to specify patterns in a human-readable format. You will use this crate to parse CBOR diagnostic notation into `CBOR` values.
- `bc-envelope`: The core crate for Gordian Envelope, which provides the basic data structures and functionality for working with Gordian Envelope.
- `bc-envelope-pattern`: A crate that provides pattern matching and text syntax parsing functionality for Gordian Envelope, which will eventually depend on `dcbor-pattern` for its LEAF pattern matching.

## Architectural Notes

### Key Differences from `bc-envelope-pattern`

This crate is focused on deterministic CBOR (dCBOR) patterns, while `bc-envelope-pattern` is focused on Gordian Envelope patterns. Understanding these differences is crucial for implementation:

**1. Dependency Relationship:**
- `bc-envelope-pattern` will eventually depend on `dcbor-pattern` for its LEAF pattern matching
- This crate will **never** depend on `bc-envelope-pattern` as it focuses on lower-level dCBOR patterns
- This crate should **never** refer to Gordian Envelope concepts (subjects, assertions, predicates)

**2. Pattern Organization:**
- `dcbor-pattern` separates atomic values (`pattern::value`) from compound structures (`pattern::structure`)
- `bc-envelope-pattern` groups all CBOR values under `pattern::leaf` regardless of complexity
- `value` patterns in this crate are atomic `CBOR` values only
- `leaf` patterns in `bc-envelope-pattern` include *any* CBOR value, including compound structures

**3. Tree Navigation:**
- Our `Path` uses `Vec<CBOR>` elements for dCBOR tree navigation
- `bc-envelope-pattern` uses `Vec<Envelope>` for Envelope tree navigation
- CBOR tree branching points: arrays, maps, tagged values
- Envelope tree branching points: assertions, wrapped envelopes

**4. VM Implementation:**
- Our VM handles dCBOR tree traversal (ArrayElement, MapKey, MapValue, TaggedContent)
- `bc-envelope-pattern` VM handles Envelope tree traversal (Subject, Assertion, Predicate, Object, Wrapped)

**5. Shared Concepts:**
- Some concepts in `bc-envelope-pattern` are properly dCBOR concepts (dates, known values, etc.)
- These will be implemented in this crate, not inherited from `bc-envelope-pattern`
- Both crates have analogous modules (`quantifier`) and folder hierarchy (`pattern`, `parse`)

**6. Current Status Differences:**
- ✅ Our VM is now fully implemented
- ✅ Our value patterns have working `compile()` methods
- 🔨 We still need main pattern parsing entry point (`parse_pattern.rs`)
- 🔨 Meta patterns need full implementation

### Update Instructions for Contributors

**When completing tasks, update the following sections in this file:**

1. **Implementation Status Lists**: Move items from `[ ]` to `[x]` as they're completed
2. **Priority Implementation Order**: Reorder based on what's been completed
3. **Test Coverage Status**: Update test counts and mark test suites as implemented
4. **Current Tasks**: Update the focus area as modules are completed

**Completion Indicators:**
- ✅ = Fully implemented and tested
- 🔨 = Partially implemented (needs completion)
- ❌ = Not implemented (stub only)

**Critical Path Dependencies:**
1. VM implementation blocks all `compile()` methods
2. Main pattern parser blocks high-level pattern usage
3. Meta patterns are needed for complex pattern composition
4. Structure patterns are needed for compound dCBOR data

## Current Tasks

Every task for now will require you to compare the analogous implementation in `bc-envelope-pattern` and adapt it to the `dcbor-pattern` crate.

For a given task pattern, you will need to:

- Look at the `bc-envelope-pattern` crate's analogous module for inspiration.
- Implement the `pattern` module in `dcbor-pattern`, ensuring that it can handle the specific requirements of dCBOR patterns.
- Implement the `parser` module in `dcbor-pattern` to parse text syntax patterns into patterns.
- Implement unit tests for the patterns, ensuring that they cover all edge cases and conform to the dCBOR and pattern expression syntax specifications.
- Implement integration tests in the `tests` directory to ensure that the patterns work correctly with the dCBOR values.

## Implementation Status

*Note: Update this section when completing tasks to track progress.*

### Pattern Module Implementation Status

#### ✅ Core Infrastructure
- [x] `pattern_impl.rs` - Core Pattern enum and basic structure
- [x] `matcher.rs` - Matcher trait definition (basic structure)
- [x] `vm.rs` - VM skeleton (needs full implementation)

#### ✅ Value Patterns (pattern::value)
Comparison: `bc-envelope-pattern::pattern::leaf` has 13 pattern types vs our 8

**Implemented with `paths()` method:**
- [x] `bool_pattern.rs` - Boolean value patterns
- [x] `bytestring_pattern.rs` - Byte string patterns
- [x] `date_pattern.rs` - Date/time patterns
- [x] `null_pattern.rs` - Null value patterns
- [x] `number_pattern.rs` - Numeric patterns (int, float, ranges)
- [x] `text_pattern.rs` - Text string patterns
- [x] `value_pattern.rs` - Top-level value pattern enum

**Stub implementations (need full implementation):**
- [ ] `digest_pattern.rs` - Cryptographic digest patterns (`Digest` is implemented in `bc-components`)
- [ ] `known_value_pattern.rs` - Known value patterns

#### ✅ Structure Patterns (pattern::structure)
Comparison: `bc-envelope-pattern::pattern::structure` has 10 envelope-specific patterns vs our 4 dCBOR patterns

**✅ Fully implemented with Matcher trait:**
- [x] `structure_pattern.rs` - Top-level structure pattern enum (**FULLY IMPLEMENTED!**)
- [x] `array_pattern.rs` - CBOR array structure patterns (**FULLY IMPLEMENTED!**)
- [x] `map_pattern.rs` - CBOR map structure patterns (**FULLY IMPLEMENTED!**)
- [x] `tagged_pattern.rs` - CBOR tagged value patterns (**FULLY IMPLEMENTED!**)

#### ✅ Meta Patterns (pattern::meta)
Comparison: `bc-envelope-pattern::pattern::meta` has 11 meta patterns vs our 8

**✅ Fully implemented with Matcher trait:**
- [x] `any_pattern.rs` - Match any CBOR value patterns (**FULLY IMPLEMENTED!**)
- [x] `none_pattern.rs` - Match no CBOR value patterns (**FULLY IMPLEMENTED!**)
- [x] `and_pattern.rs` - Logical AND combinations (**FULLY IMPLEMENTED!**)
- [x] `or_pattern.rs` - Logical OR combinations (**FULLY IMPLEMENTED!**)
- [x] `not_pattern.rs` - Logical NOT patterns (**FULLY IMPLEMENTED!**)
- [x] `capture_pattern.rs` - Pattern capture groups (**FULLY IMPLEMENTED!**)
- [x] `meta_pattern.rs` - Top-level meta pattern enum (**FULLY IMPLEMENTED!**)
- [x] `repeat_pattern.rs` - Repetition patterns (**FULLY IMPLEMENTED!**)

**🔨 Stub implementations (need full implementation):**
- [ ] `search_pattern.rs` - Search patterns (will be implemented using `dcbor` crate's `walk` module)

**Missing meta patterns (present in bc-envelope-pattern):**
- [ ] `sequence_pattern.rs` - Sequential pattern matching

#### ✅ VM Implementation
- [x] `vm.rs` - Pattern matching virtual machine (**FULLY IMPLEMENTED!**)
  - ✅ Complete instruction set (15 instruction types)
  - ✅ dCBOR tree navigation with Axis system (ArrayElement, MapKey, MapValue, TaggedContent)
  - ✅ Thread-based execution model with backtracking
  - ✅ Pattern compilation support for atomic patterns
  - ✅ Repeat pattern support with quantifiers
  - ✅ Capture group infrastructure
  - ✅ All value pattern `compile()` methods now working
  - **Unblocks**: All pattern compilation that was previously `unimplemented!()`

### Parse Module Implementation Status

#### ✅ Core Infrastructure
- [x] `token.rs` - Lexer tokens for pattern parsing
- [x] `parse/mod.rs` - Module organization

#### ✅ Value Parsers (parse::value)
**Implemented parsers:**
- [x] `bool_parser.rs` - Boolean value parsing
- [x] `bytestring_parser.rs` - Byte string parsing
- [x] `date_parser.rs` - Date/time parsing (**FULLY IMPLEMENTED with dcbor-parse integration**)
- [x] `null_parser.rs` - Null value parsing
- [x] `number_parser.rs` - Numeric value parsing
- [x] `text_parser.rs` - Text string parsing

**Stub implementations:**
- [ ] `digest_parser.rs` - Digest value parsing
- [ ] `known_value_parser.rs` - Known value parsing

#### 🔨 Structure Parsers (parse::structure)
**Stub implementations:**
- [ ] `array_parser.rs` - CBOR array parsing
- [ ] `map_parser.rs` - CBOR map parsing
- [ ] `tagged_parser.rs` - CBOR tagged value parsing

#### 🔨 Meta Parsers (parse::meta)
**Mostly stub implementations:**
- [ ] `and_parser.rs` - AND pattern parsing (has `todo!()`)
- [ ] `capture_parser.rs` - Capture pattern parsing
- [ ] `not_parser.rs` - NOT pattern parsing
- [ ] `or_parser.rs` - OR pattern parsing
- [ ] `search_parser.rs` - Search pattern parsing

**✅ Recently completed:**
- [x] `repeat_parser.rs` - Repeat pattern parsing (**FULLY IMPLEMENTED with quantifier support!**)

**Missing critical parsers (present in bc-envelope-pattern):**
- [ ] `parse_pattern.rs` - Main pattern parsing entry point (**PARTIAL: Pattern::parse supports BOOL, DATE, NUMBER, NULL**)
- [ ] `utils.rs` - Parsing utility functions
- [ ] `group_parser.rs` - Group pattern parsing
- [ ] `primary_parser.rs` - Primary pattern parsing
- [ ] `sequence_parser.rs` - Sequence pattern parsing

### Test Coverage Status

#### ✅ Working Tests
- ✅ `parse_tests_value.rs` - **15 tests passing** (includes comprehensive date pattern parsing tests)
- ✅ `pattern_tests_value.rs` - 42 tests passing
- ✅ `pattern_tests_meta.rs` - **23 tests passing** (**6 NEW repeat pattern tests + 7 capture pattern tests!**)
- ✅ `pattern_tests_structure.rs` - **10 tests passing**
- ✅ `error_tests.rs` - 6 tests passing

#### ❌ Missing/Empty Test Suites
- [ ] `parse_tests_meta.rs` - 0 tests
- [ ] `parse_tests_structure.rs` - 0 tests

### Priority Implementation Order

**Next Priority Tasks:**
1. **Complete Meta Patterns** - Search patterns (final meta pattern)
2. **Missing Value Patterns** - Complete digest and known value patterns
3. **Structure Pattern Parsers** - Text syntax parsing for arrays, maps, and tagged values
4. **Main Parse Pattern** - Entry point for pattern parsing
5. **Complete Test Coverage** - Tests for all implemented features

**Completed Major Milestones:**
- ✅ **VM Implementation** - Fully functional pattern matching virtual machine
- ✅ **Core Meta Patterns** - AND, OR, NOT, ANY, NONE patterns fully implemented with tests!
- ✅ **Capture Patterns** - Full capture group implementation with comprehensive tests!
- ✅ **Repeat Patterns** - Full repetition quantifier implementation with parsing support!
- ✅ **Structure Patterns** - CBOR array, map, and tagged value patterns fully implemented with tests!
- ✅ **Date Pattern Parsing** - Full ISO-8601 date pattern parsing with dcbor-parse integration supporting all forms (single, range, regex)!
