# `dcbor-pattern` Crate Documentation

This file contains general information about the `dcbor-pattern` crate, which provides a pattern matcher and text syntax pattern parser for Deterministic CBOR (dCBOR) as implemented in the `dcbor` crate in this workspace. Further documentation including the pattern expression syntax can be found in the `docs/` directory. Make sure to read those before starting on any tasks.

## General Guidance

This crate is just a skeleton at the moment, based on the `bc-envelope-pattern` crate. You will be receiving tasks to implement the pattern matcher and text syntax parser for dCBOR. Always make sure that `cargo test` and `cargo clippy` pass before you're done with your changes.

## Crates in this Workspace

You will only be making changes to the `dcbor-pattern` crate, but it is important to understand the other crates in this workspace as they provide the context and dependencies for your work:

- `dcbor-pattern`: The crate you are currently working on, which provides the pattern matching and text syntax parsing functionality for dCBOR.
- `dcbor`: The core crate for deterministic CBOR, which provides the basic data structures and functionality for working with dCBOR values.
- `dcbor-parse`: A parser for dCBOR diagnostic notation, which is used to specify patterns in a human-readable format. You will use this crate to parse CBOR diagnostic notation into `CBOR` values.
- `bc-envelope`: The core crate for Gordian Envelope, which provides the basic data structures and functionality for working with Gordian Envelope.
- `bc-envelope-pattern`: A crate that provides pattern matching and text syntax parsing functionality for Gordian Envelope, which will eventually depend on `dcbor-pattern` for its LEAF pattern matching.

## Architectural Notes

### Important Differences between `dcbor-pattern` and `bc-envelope-pattern`

- This crate is focused on deterministic CBOR (dCBOR) patterns, while `bc-envelope-pattern` is focused on Gordian Envelope patterns.
- `bc-envelope-pattern` will eventually depend on `dcbor-pattern` for its LEAF pattern matching.
- This crate, `dcbor-pattern`, will not depend on `bc-envelope-pattern` as it is focused on the lower-level dCBOR patterns. It should never refer to Gordian Envelope concepts like subjects, assertions, or predicates.
- Some concepts mentioned in `bc-envelope-pattern` are properly concepts of dCBOR, such as dates, known values, and the like. These concepts will be implemented in this crate, `dcbor-pattern`.
- The concept of `Path` in this crate is analogous to the `Path` in `bc-envelope-pattern`, but each path element is a `CBOR` object, not an `Envelope`.
- `CBOR` objects, like `Envelope` objects, are trees. But the branching points of `CBOR` are its compound structures like arrays and maps, not assertions and wrapped envelopes.
- Both crates have analogous modules, such as `quantifier`.
- Both crates have analogous folder hierarchy, such as `pattern` and `parser`.
- A main difference is that `dcbor-pattern` refers to `value` patterns intead of `leaf` patterns. `value` patterns are atomic `CBOR` values, while `leaf` patterns in `bc-envelope-pattern` are *any* CBOR value, including compound structures like arrays and maps.

### Key Differences from bc-envelope-pattern

1. **Pattern Organization**:
   - `dcbor-pattern` separates atomic values (`pattern::value`) from compound structures (`pattern::structure`)
   - `bc-envelope-pattern` groups all CBOR values under `pattern::leaf` regardless of complexity

2. **VM Requirements**:
   - Our VM needs to handle dCBOR tree traversal (arrays, maps, tagged values)
   - `bc-envelope-pattern` VM handles Envelope tree traversal (subjects, assertions, predicates)

3. **Path Representation**:
   - Our `Path` uses `Vec<CBOR>` elements for dCBOR tree navigation
   - `bc-envelope-pattern` uses `Vec<Envelope>` for Envelope tree navigation

4. **Missing Core Infrastructure**:
   - No main pattern parsing entry point (`parse_pattern.rs`)
   - VM is completely empty (critical blocker)
   - Meta patterns are just stubs

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
- [ ] `digest_pattern.rs` - Cryptographic digest patterns
- [ ] `known_value_pattern.rs` - Known value patterns

**Missing value patterns (present in bc-envelope-pattern):**
- [ ] `cbor_pattern.rs` - Raw CBOR value patterns
- [ ] `array_pattern.rs` - CBOR array patterns (in structure module instead)
- [ ] `map_pattern.rs` - CBOR map patterns (in structure module instead)
- [ ] `tagged_pattern.rs` - CBOR tagged patterns (in structure module instead)

#### 🔨 Structure Patterns (pattern::structure)
Comparison: `bc-envelope-pattern::pattern::structure` has 10 envelope-specific patterns vs our 4 dCBOR patterns

**Basic Matcher trait implementation:**
- [x] `structure_pattern.rs` - Top-level structure pattern enum (✅ **BASIC MATCHER IMPLEMENTED**)

**Stub implementations (need full implementation):**
- [ ] `array_pattern.rs` - CBOR array structure patterns
- [ ] `map_pattern.rs` - CBOR map structure patterns
- [ ] `tagged_pattern.rs` - CBOR tagged value patterns

#### 🔨 Meta Patterns (pattern::meta)
Comparison: `bc-envelope-pattern::pattern::meta` has 11 meta patterns vs our 8

**Stub implementations (need full implementation):**
- [ ] `and_pattern.rs` - Logical AND combinations
- [ ] `capture_pattern.rs` - Pattern capture groups
- [ ] `not_pattern.rs` - Logical NOT patterns
- [ ] `or_pattern.rs` - Logical OR combinations
- [ ] `repeat_pattern.rs` - Repetition patterns
- [ ] `search_pattern.rs` - Search patterns
- [ ] `meta_pattern.rs` - Top-level meta pattern enum

**Missing meta patterns (present in bc-envelope-pattern):**
- [ ] `any_pattern.rs` - Match any pattern (equivalent to `Any` variant)
- [ ] `none_pattern.rs` - Match no pattern (equivalent to `None` variant)
- [ ] `sequence_pattern.rs` - Sequential pattern matching

#### ✅ VM Implementation
**🎉 MAJOR BREAKTHROUGH: VM Implementation Complete**
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
- [x] `date_parser.rs` - Date/time parsing
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
- [ ] `repeat_parser.rs` - Repeat pattern parsing
- [ ] `search_parser.rs` - Search pattern parsing

**Missing critical parsers (present in bc-envelope-pattern):**
- [ ] `parse_pattern.rs` - Main pattern parsing entry point
- [ ] `utils.rs` - Parsing utility functions
- [ ] `group_parser.rs` - Group pattern parsing
- [ ] `primary_parser.rs` - Primary pattern parsing
- [ ] `sequence_parser.rs` - Sequence pattern parsing

### Test Coverage Status

#### ✅ Working Tests
- ✅ `parse_tests_value.rs` - 29 tests passing
- ✅ `pattern_tests_value.rs` - 42 tests passing
- ✅ `error_tests.rs` - 6 tests passing

#### ❌ Missing/Empty Test Suites
- [ ] `parse_tests_meta.rs` - 0 tests
- [ ] `parse_tests_structure.rs` - 0 tests
- [ ] `pattern_tests_meta.rs` - 0 tests
- [ ] `pattern_tests_structure.rs` - 0 tests

### Priority Implementation Order

🎉 **MAJOR PROGRESS: VM Implementation Complete!**

**Next Priority Tasks:**
1. **Meta Patterns** - Core logical operations (AND, OR, NOT, etc.) - now unblocked by VM
2. **Structure Patterns** - CBOR compound data structures (arrays, maps, tags)
3. **Missing Value Patterns** - Complete digest and known value patterns
4. **Main Parse Pattern** - Entry point for pattern parsing
5. **Complete Test Coverage** - Tests for all implemented features

**Completed Major Milestone:**
- ✅ **VM Implementation** - Fully functional pattern matching virtual machine

## Current Tasks

Every task for now will require you to compare the analogous implementation in `bc-envelope-pattern` and adapt it to the `dcbor-pattern` crate.

At the moment we are working on building out the `pattern::value` module, including the programmatic API for composing patterns and the VM for matching patterns against dCBOR values.

For a given `value` pattern, you will need to:

- Look at the `bc-envelope-pattern` crate's `pattern::leaf` module for inspiration.
- Implement the `pattern::value` module in `dcbor-pattern`, ensuring that it can handle the specific requirements of dCBOR patterns.
- Implement the `parser` module to parse text syntax patterns into `value` patterns.
- Implement unit tests for the `value` patterns, ensuring that they cover all edge cases and conform to the dCBOR specifications.
- Implement integration tests in the `tests` directory to ensure that the `value` patterns work correctly with the dCBOR values.
