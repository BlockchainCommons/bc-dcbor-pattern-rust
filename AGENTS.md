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
- ✅ Our structure patterns are fully implemented with working `compile()` methods
- ✅ Our meta patterns are fully implemented (except search pattern stub)
- 🔨 Value pattern parsers partially implemented (5/8 done: bool, date, null, number, text)
- 🔨 Meta pattern parsers minimally implemented (1/6 done: repeat)
- ❌ Structure pattern parsers not implemented (0/3 done)
- 🔨 Main pattern parsing entry point partially implemented (supports 5 basic types)

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

The parsing implementation is now nearly complete! The pattern matching infrastructure (VM and patterns) is fully implemented, and the text syntax parsing is almost complete.

All major meta pattern parsers are now implemented:
- ✅ **OR Parser**: Top-level parser with left associativity
- ✅ **AND Parser**: Mid-level parser with proper precedence
- ✅ **NOT Parser**: Right-associative NOT operator
- ✅ **Primary Parser**: Atomic patterns, parentheses, captures
- ✅ **Capture Parser**: Named capture groups (@name(pattern))
- ✅ **Repeat Parser**: Quantifiers (*, +, ?, {n,m})

The main Pattern::parse method now supports the full dCBOR pattern syntax with proper operator precedence and all atomic patterns.

**Remaining Work:**
Only the search pattern functionality remains to be implemented. This is a specialized pattern for tree traversal that requires additional infrastructure beyond basic pattern matching.

## Implementation Status

*Note: Update this section when completing tasks to track progress.*

### Pattern Module Implementation Status

#### ✅ Core Infrastructure
- [x] `pattern_impl.rs` - Core Pattern enum and basic structure
- [x] `matcher.rs` - Matcher trait definition (basic structure)
- [x] `vm.rs` - VM skeleton (needs full implementation)

#### ✅ Value Patterns (pattern::value)
Comparison: `bc-envelope-pattern::pattern::leaf` has 13 pattern types vs our 8

**✅ Fully implemented with Matcher trait:**
- [x] `bool_pattern.rs` - Boolean value patterns (**FULLY IMPLEMENTED!**)
- [x] `bytestring_pattern.rs` - Byte string patterns (**FULLY IMPLEMENTED!**)
- [x] `date_pattern.rs` - Date/time patterns (**FULLY IMPLEMENTED!**)
- [x] `digest_pattern.rs` - Cryptographic digest patterns (**FULLY IMPLEMENTED!**)
- [x] `known_value_pattern.rs` - Known value patterns (**FULLY IMPLEMENTED!**)
- [x] `null_pattern.rs` - Null value patterns (**FULLY IMPLEMENTED!**)
- [x] `number_pattern.rs` - Numeric patterns (int, float, ranges) (**FULLY IMPLEMENTED!**)
- [x] `text_pattern.rs` - Text string patterns (**FULLY IMPLEMENTED!**)
- [x] `value_pattern.rs` - Top-level value pattern enum (**FULLY IMPLEMENTED!**)

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
- [ ] `search_pattern.rs` - Search patterns (**STUB: Has structure but paths() and compile() are unimplemented!()**)

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
  - ✅ All structure pattern `compile()` methods now working

### Parse Module Implementation Status

#### ✅ Core Infrastructure
- [x] `token.rs` - Lexer tokens for pattern parsing
- [x] `parse/mod.rs` - Module organization

#### 🔨 Value Parsers (parse::value)
**✅ Fully implemented parsers:**
- [x] `bool_parser.rs` - Boolean value parsing (**FULLY IMPLEMENTED**)
- [x] `bytestring_parser.rs` - Byte string parsing (**FULLY IMPLEMENTED with hex and binary regex support!**)
- [x] `date_parser.rs` - Date/time parsing (**FULLY IMPLEMENTED with dcbor-parse integration**)
- [x] `digest_parser.rs` - Digest value parsing (**FULLY IMPLEMENTED with hex prefix and UR string support!**)
- [x] `known_value_parser.rs` - Known value parsing (**FULLY IMPLEMENTED with numeric, named, and regex support!**)
- [x] `null_parser.rs` - Null value parsing (**FULLY IMPLEMENTED**)
- [x] `number_parser.rs` - Numeric value parsing (**FULLY IMPLEMENTED**)
- [x] `text_parser.rs` - Text string parsing (**FULLY IMPLEMENTED with regex and string literal support!**)

**❌ Empty files (need implementation):**

#### ✅ Structure Parsers (parse::structure)
**✅ Fully implemented:**
- [x] `array_parser.rs` - CBOR array parsing (**FULLY IMPLEMENTED with length range support!**)
- [x] `map_parser.rs` - CBOR map parsing (**FULLY IMPLEMENTED with length range support!**)
- [x] `tagged_parser.rs` - CBOR tagged value parsing (**FULLY IMPLEMENTED with tag value, name, and regex support!**)

#### ✅ Meta Parsers (parse::meta)
**✅ Fully implemented:**
- [x] `repeat_parser.rs` - Repeat pattern parsing (**FULLY IMPLEMENTED with quantifier support!**)
- [x] `and_parser.rs` - AND pattern parsing (**FULLY IMPLEMENTED with precedence support!**)
- [x] `or_parser.rs` - OR pattern parsing (**FULLY IMPLEMENTED with precedence support!**)
- [x] `not_parser.rs` - NOT pattern parsing (**FULLY IMPLEMENTED with right associativity!**)
- [x] `capture_parser.rs` - Capture pattern parsing (**FULLY IMPLEMENTED with proper error handling!**)
- [x] `primary_parser.rs` - Primary pattern parsing (**FULLY IMPLEMENTED as foundation parser!**)

**🔨 Stub implementations (need full implementation):**
- [ ] `search_parser.rs` - Search pattern parsing (**STUB: Depends on search pattern infrastructure**)

**❌ Missing critical parsers (present in bc-envelope-pattern):**
- [x] `parse_pattern.rs` - Main pattern parsing entry point (**FULLY IMPLEMENTED: Pattern::parse now supports full hierarchy OR->AND->NOT->PRIMARY!**)
- [ ] `utils.rs` - Parsing utility functions
- [ ] `group_parser.rs` - Group pattern parsing (**IMPLEMENTED: Integrated into primary_parser.rs**)
- [x] `primary_parser.rs` - Primary pattern parsing (**FULLY IMPLEMENTED as foundation parser!**)
- [ ] `sequence_parser.rs` - Sequence pattern parsing (**NOT NEEDED: dcbor-pattern doesn't have sequence patterns**)

### Test Coverage Status

#### ✅ Working Tests
- ✅ `parse_tests_value.rs` - **35 tests passing** (includes comprehensive date, text, bytestring, and digest pattern parsing tests)
- ✅ `pattern_tests_value.rs` - **40 tests passing** (includes 5 comprehensive known value pattern tests + 6 new digest pattern tests)
- ✅ `pattern_tests_meta.rs` - **23 tests passing** (**6 NEW repeat pattern tests + 7 capture pattern tests!**)
- ✅ `pattern_tests_structure.rs` - **10 tests passing**
- ✅ `error_tests.rs` - **67 tests passing** (actual count much higher than previously documented)
- ✅ `parse_tests_meta.rs` - **26 tests passing** (**NEW: Comprehensive meta pattern parser tests with precedence verification!**)
- **Total**: **209 tests passing** (**26 new meta pattern parser tests added!**)

#### ❌ Missing/Empty Test Suites
- [ ] `parse_tests_meta.rs` - 0 tests
- [ ] `parse_tests_structure.rs` - 0 tests

### Priority Implementation Order

**Next Priority Tasks:**
1. **Search Pattern Infrastructure** - Implement search_pattern.rs fully (paths() and compile() methods) and add SEARCH token support
2. **Complete Stub Patterns** - Once search pattern is fully implemented, add search_parser.rs
3. **Parsing Utility Functions** - Add utils.rs for common parsing operations if needed
4. **Complete Test Coverage** - Tests for search patterns when implemented

**Completed Major Milestones:**
- ✅ **Complete Meta Pattern Parser Infrastructure** - Full OR->AND->NOT->PRIMARY parser hierarchy with precedence support!
- ✅ **All Meta Pattern Parsers Complete** - AND, OR, NOT, capture, and primary parsers fully implemented (5/6 implemented)!
- ✅ **Parser Integration Complete** - Pattern::parse now supports full dCBOR pattern syntax including meta patterns!
- ✅ **Comprehensive Test Suite** - 26 new meta pattern parser tests covering simple, complex, and edge cases!
- ✅ **Map Pattern Parsing** - Full map pattern parsing with length range support and Pattern::parse integration!
- ✅ **Array Pattern Parsing** - Full array pattern parsing with length range support and Pattern::parse integration!
- ✅ **All Value Pattern Parsers Complete** - All 8 value pattern parsers fully implemented (bool, bytestring, date, digest, known value, null, number, text)!
- ✅ **Digest Pattern Parsing** - Full digest pattern parsing with hex prefix and UR string support, including Pattern::parse integration!
- ✅ **Known Value Pattern Parsing** - Full known value pattern parsing with numeric, named, and regex support!
- ✅ **Pattern Compilation Issues Fixed** - Removed all unimplemented!() calls from pattern_impl.rs for StructurePattern!
- ✅ **VM Implementation** - Fully functional pattern matching virtual machine
- ✅ **Core Meta Patterns** - AND, OR, NOT, ANY, NONE patterns fully implemented with tests!
- ✅ **Capture Patterns** - Full capture group implementation with comprehensive tests!
- ✅ **Repeat Patterns** - Full repetition quantifier implementation with parsing support!
- ✅ **Structure Patterns** - CBOR array, map, and tagged value patterns fully implemented with tests!
- ✅ **Date Pattern Parsing** - Full ISO-8601 date pattern parsing with dcbor-parse integration supporting all forms (single, range, regex)!
- ✅ **Known Value Patterns** - Complete implementation with proper CBOR tag 40000 handling, supporting Any, Value, Named, and Regex variants with comprehensive tests!
- ✅ **Text Pattern Parsing** - Full text pattern parsing with string literal and regex support, including proper escape sequence handling and round-trip parsing/display!
