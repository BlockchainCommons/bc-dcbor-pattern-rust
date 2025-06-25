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

**Current Status Differences:**
- ✅ Our VM is fully implemented with complete instruction set
- ✅ Our value patterns have working `compile()` methods
- ✅ Our structure patterns are fully implemented with working `compile()` methods
- ✅ Our meta patterns are fully implemented (7/8 - only search pattern incomplete)
- ✅ All value pattern parsers implemented (8/8 complete)
- ✅ All structure pattern parsers implemented (3/3 complete)
- ✅ All meta pattern parsers implemented (5/6 - only search parser missing)
- ✅ Main pattern parsing entry point fully supports complete syntax with operator precedence
- 🔨 Search pattern functionality partially implemented (structure exists, core methods unimplemented)

### Update Instructions for Contributors

**Critical**: This file reflects the current state as of December 2024. The crate is 98% complete.

**When completing the remaining search pattern work:**

1. **Update Implementation Status**: Move search pattern from 🔨 to ✅ when complete
2. **Update Test Coverage**: Add test counts for search pattern tests
3. **Update Current Status**: Change from "98% complete" to "100% complete"
4. **Mark Project Complete**: Update "Next Tasks" to indicate project completion

**Completion Indicators:**
- ✅ = Fully implemented and tested
- 🔨 = Partially implemented (only search pattern remains)
- ❌ = Not implemented (none remaining)

## Current Status

The `dcbor-pattern` crate is **ESSENTIALLY COMPLETE** with only minor remaining work!

**✅ FULLY IMPLEMENTED:**
- ✅ **Complete Pattern Infrastructure**: All pattern types with working `Matcher` trait implementations
- ✅ **Complete VM Implementation**: Full pattern matching virtual machine with all instruction types
- ✅ **Complete Parser Infrastructure**: Full text syntax parsing with proper operator precedence
- ✅ **All Value Patterns**: 8/8 value pattern types fully implemented with parsing
- ✅ **All Structure Patterns**: 3/3 structure pattern types fully implemented with parsing
- ✅ **All Meta Patterns**: 7/8 meta pattern types fully implemented with parsing
- ✅ **Main Pattern::parse**: Supports complete dCBOR pattern syntax including precedence
- ✅ **Comprehensive Test Suite**: 252 passing tests across all modules

**🔨 MINIMAL REMAINING WORK:**
- [ ] **Search Pattern**: Only remaining unimplemented pattern (paths() and compile() methods have unimplemented!())
- [ ] **Search Token**: Add SEARCH token to lexer for search pattern parsing
- [ ] **Search Parser**: Implement search_parser.rs (currently empty file)

**Note**: Search patterns are specialized for tree traversal and require additional design decisions about search semantics.

## Implementation Status

**Overall Progress: 98% Complete** - Only search pattern functionality remains unimplemented.

*Last Updated: December 2024*

### Pattern Module Implementation Status

#### ✅ Core Infrastructure - COMPLETE
- [x] `pattern_impl.rs` - Core Pattern enum and main Pattern::parse method (**FULLY IMPLEMENTED!**)
- [x] `matcher.rs` - Matcher trait definition (**COMPLETE WITH NOTE**: Contains fallback unimplemented!() for debugging only)
- [x] `vm.rs` - Pattern matching virtual machine (**FULLY IMPLEMENTED!**)

#### ✅ Value Patterns (pattern::value) - COMPLETE
**✅ All 8 value patterns fully implemented with Matcher trait and parsing:**
- [x] `bool_pattern.rs` - Boolean value patterns (**FULLY IMPLEMENTED!**)
- [x] `bytestring_pattern.rs` - Byte string patterns (**FULLY IMPLEMENTED!**)
- [x] `date_pattern.rs` - Date/time patterns (**FULLY IMPLEMENTED!**)
- [x] `digest_pattern.rs` - Cryptographic digest patterns (**FULLY IMPLEMENTED!**)
- [x] `known_value_pattern.rs` - Known value patterns (**FULLY IMPLEMENTED!**)
- [x] `null_pattern.rs` - Null value patterns (**FULLY IMPLEMENTED!**)
- [x] `number_pattern.rs` - Numeric patterns (int, float, ranges) (**FULLY IMPLEMENTED!**)
- [x] `text_pattern.rs` - Text string patterns (**FULLY IMPLEMENTED!**)
- [x] `value_pattern.rs` - Top-level value pattern enum (**FULLY IMPLEMENTED!**)

#### ✅ Structure Patterns (pattern::structure) - COMPLETE
**✅ All 3 structure patterns fully implemented with Matcher trait and parsing:**
- [x] `structure_pattern.rs` - Top-level structure pattern enum (**FULLY IMPLEMENTED!**)
- [x] `array_pattern.rs` - CBOR array structure patterns (**FULLY IMPLEMENTED!**)
- [x] `map_pattern.rs` - CBOR map structure patterns (**FULLY IMPLEMENTED!**)
- [x] `tagged_pattern.rs` - CBOR tagged value patterns (**FULLY IMPLEMENTED!**)

#### 🔨 Meta Patterns (pattern::meta) - ALMOST COMPLETE
**✅ Fully implemented with Matcher trait (7/8 patterns):**
- [x] `any_pattern.rs` - Match any CBOR value patterns (**FULLY IMPLEMENTED!**)
- [x] `none_pattern.rs` - Match no CBOR value patterns (**FULLY IMPLEMENTED!**)
- [x] `and_pattern.rs` - Logical AND combinations (**FULLY IMPLEMENTED!**)
- [x] `or_pattern.rs` - Logical OR combinations (**FULLY IMPLEMENTED!**)
- [x] `not_pattern.rs` - Logical NOT patterns (**FULLY IMPLEMENTED!**)
- [x] `capture_pattern.rs` - Pattern capture groups (**FULLY IMPLEMENTED!**)
- [x] `meta_pattern.rs` - Top-level meta pattern enum (**FULLY IMPLEMENTED!**)
- [x] `repeat_pattern.rs` - Repetition patterns (**FULLY IMPLEMENTED!**)

**🔨 Stub implementations (1/8 patterns need completion):**
- [ ] `search_pattern.rs` - Search patterns (**STUB: Has structure but paths() and compile() methods have unimplemented!()**)

#### ✅ VM Implementation - COMPLETE
- [x] `vm.rs` - Pattern matching virtual machine (**FULLY IMPLEMENTED!**)
  - ✅ Complete instruction set (15 instruction types)
  - ✅ dCBOR tree navigation with Axis system
  - ✅ Thread-based execution model with backtracking
  - ✅ Pattern compilation support for all implemented patterns
  - ✅ Repeat pattern support with quantifiers
  - ✅ Capture group infrastructure

### Parse Module Implementation Status

#### ✅ Core Infrastructure - COMPLETE
- [x] `token.rs` - Lexer tokens for pattern parsing (**COMPLETE**: 40+ token types with proper lexing)
- [x] `parse/mod.rs` - Module organization (**COMPLETE**)

#### ✅ Value Parsers (parse::value) - COMPLETE
**✅ All 8 value parsers fully implemented:**
- [x] `bool_parser.rs` - Boolean value parsing (**FULLY IMPLEMENTED**)
- [x] `bytestring_parser.rs` - Byte string parsing (**FULLY IMPLEMENTED**)
- [x] `date_parser.rs` - Date/time parsing (**FULLY IMPLEMENTED**)
- [x] `digest_parser.rs` - Digest value parsing (**FULLY IMPLEMENTED**)
- [x] `known_value_parser.rs` - Known value parsing (**FULLY IMPLEMENTED**)
- [x] `null_parser.rs` - Null value parsing (**FULLY IMPLEMENTED**)
- [x] `number_parser.rs` - Numeric value parsing (**FULLY IMPLEMENTED**)
- [x] `text_parser.rs` - Text string parsing (**FULLY IMPLEMENTED**)

#### ✅ Structure Parsers (parse::structure) - COMPLETE
**✅ All 3 structure parsers fully implemented:**
- [x] `array_parser.rs` - CBOR array parsing (**FULLY IMPLEMENTED**)
- [x] `map_parser.rs` - CBOR map parsing (**FULLY IMPLEMENTED**)
- [x] `tagged_parser.rs` - CBOR tagged value parsing (**FULLY IMPLEMENTED**)

#### 🔨 Meta Parsers (parse::meta) - ALMOST COMPLETE
**✅ Fully implemented (5/6 parsers):**
- [x] `repeat_parser.rs` - Repeat pattern parsing (**FULLY IMPLEMENTED**)
- [x] `and_parser.rs` - AND pattern parsing (**FULLY IMPLEMENTED**)
- [x] `or_parser.rs` - OR pattern parsing (**FULLY IMPLEMENTED**)
- [x] `not_parser.rs` - NOT pattern parsing (**FULLY IMPLEMENTED**)
- [x] `capture_parser.rs` - Capture pattern parsing (**FULLY IMPLEMENTED**)
- [x] `primary_parser.rs` - Primary pattern parsing (**FULLY IMPLEMENTED**)

**❌ Missing parsers (1/6 need implementation):**
- [ ] `search_parser.rs` - Search pattern parsing (**EMPTY FILE**: Depends on search pattern infrastructure)

### Test Coverage Status

**✅ COMPREHENSIVE TEST SUITE: 252 TOTAL PASSING TESTS**

#### ✅ All Test Suites Implemented and Passing
- ✅ **parse_tests_value.rs** - **27 tests** (value pattern parsing)
- ✅ **pattern_tests_value.rs** - **34 tests** (value pattern functionality)
- ✅ **pattern_tests_meta.rs** - **23 tests** (meta pattern functionality)
- ✅ **pattern_tests_structure.rs** - **10 tests** (structure pattern functionality)
- ✅ **parse_tests_meta.rs** - **26 tests** (meta pattern parsing)
- ✅ **map_pattern_integration_tests.rs** - **4 tests** (map pattern integration)
- ✅ **Plus 128 internal module tests** - Unit tests within individual pattern and parser modules

#### ❌ Empty Test Files (No Tests Needed)
- **error_tests.rs** - 0 tests (empty file - error testing done within modules)
- **parse_tests_structure.rs** - 0 tests (empty file - structure parsing tested within modules)

**No missing test coverage** - All implemented functionality has comprehensive test coverage.

## Next Tasks

The `dcbor-pattern` crate is **98% COMPLETE**! Only search pattern functionality remains.

### Immediate Priority Tasks

**1. Complete Search Pattern Implementation**
   - Implement `search_pattern.rs` methods:
     - `paths()` - Define search traversal semantics
     - `compile()` - Generate VM instructions for search
   - Design decisions needed:
     - Search scope (entire tree vs subtree)
     - Search order (depth-first, breadth-first)
     - Match termination (first match vs all matches)

**2. Add Search Pattern Parsing Support**
   - Add `SEARCH` token to `token.rs` lexer
   - Implement `search_parser.rs` with appropriate syntax
   - Integrate search parsing into `primary_parser.rs`

**3. Search Pattern Testing**
   - Add search pattern tests to test suites
   - Verify search functionality with complex CBOR structures

### Technical Notes for Search Implementation

- Search patterns traverse the entire dCBOR tree looking for matches
- Unlike other patterns that match at current position, search patterns explore all paths
- May need additional VM instructions for tree traversal state management
- Consider performance implications of exhaustive tree search

### Project Completion

Once search patterns are implemented, the `dcbor-pattern` crate will be **FEATURE COMPLETE** with:
- ✅ All 8 value pattern types
- ✅ All 3 structure pattern types
- ✅ All 8 meta pattern types (including search)
- ✅ Complete text syntax parsing with operator precedence
- ✅ Full VM-based pattern matching engine
- ✅ Comprehensive test coverage (250+ tests)

### Optional Future Enhancements

- **Performance optimizations** for complex pattern matching
- **Additional pattern types** if new use cases emerge
- **Enhanced error reporting** with better diagnostic messages
