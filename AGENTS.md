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

## Test Quality Assessment

### Current Test State
After surveying the test codebase, several quality issues have been identified:

#### 1. **Debug Tests with No Assertions**
Several test files contain "debug" tests that only print output without any assertions:
- `debug_capture.rs`: 3 tests with only `println!` statements
- `debug_vm_navigation.rs`: 1 test with only debug output
- `debug_array_capture.rs`: 2 tests with only debug output
- `debug_map_capture.rs`: 1 test with only debug output
- `debug_array_detailed.rs`: Debug tests with no validation
- `debug_search_capture.rs`: Debug tests with no validation

#### 2. **Performance Tests with Limited Assertions**
`test_performance.rs` contains 6 performance tests that primarily use `println!` for timing information but lack comprehensive validation of the actual pattern matching results.

#### 3. **Missing `format_paths` Validation**
Many tests use `pattern.paths()` but don't validate the formatted output using `assert_actual_expected!`:
- `test_advanced_nested_patterns.rs`: 25+ tests using only `assert!(pattern.matches())`
- `test_new_sequence_apis.rs`: Tests check basic functionality but miss path format validation
- `capture_integration_tests.rs`: Comprehensive capture tests but limited path format checking
- Several parsing test files use basic assertions instead of path format validation

#### 4. **Well-Implemented Test Examples**
Good patterns found in:
- `pattern_tests_value.rs`: Consistently uses `assert_actual_expected!(format_paths(&paths), expected)`
- `map_pattern_integration_tests.rs`: Proper use of `assert_actual_expected!` for path validation

### Test Improvement Plan

#### Phase 1: Convert Debug Tests to Proper Tests
1. **Immediate Actions**:
   - Convert all `debug_*.rs` files to use `assert_actual_expected!` with expected outputs
   - Remove or convert standalone `println!` statements to proper assertions
   - Add `format_paths` validation to capture meaningful test output

2. **Specific Files to Update**:
   ```
   tests/debug_capture.rs -> tests/capture_detailed_tests.rs
   tests/debug_vm_navigation.rs -> tests/vm_navigation_tests.rs
   tests/debug_array_capture.rs -> tests/array_capture_tests.rs
   tests/debug_map_capture.rs -> tests/map_capture_tests.rs
   tests/debug_array_detailed.rs -> tests/array_detailed_tests.rs
   tests/debug_search_capture.rs -> tests/search_capture_tests.rs
   ```

#### Phase 2: Enhance Existing Tests
1. **Add `format_paths` validation** to tests currently using only `assert!(pattern.matches())`:
   - `test_advanced_nested_patterns.rs`: Add path format checking to all 25+ test cases
   - `test_new_sequence_apis.rs`: Add path validation for sequence pattern tests
   - `capture_integration_tests.rs`: Enhance with formatted path output validation

2. **Performance Test Enhancement**:
   - Keep timing assertions but add result validation using `assert_actual_expected!`
   - Verify both performance characteristics AND correctness of pattern matching

#### Phase 3: Test Quality Standards
1. **Mandatory Requirements**:
   - Every test that calls `pattern.paths()` should validate `format_paths()` output
   - Use `assert_actual_expected!` for any complex output validation
   - No tests should rely solely on `println!` statements
   - Performance tests must validate correctness, not just timing

2. **Best Practices**:
   - Follow the pattern in `pattern_tests_value.rs` and `map_pattern_integration_tests.rs`
   - Use `indoc!` macro for multi-line expected outputs
   - Group related test assertions logically
   - Provide meaningful test failure messages

#### Implementation Priority
1. **High Priority**: Convert debug tests to proper assertions (Phase 1)
2. **Medium Priority**: Add `format_paths` validation to existing tests (Phase 2)
3. **Low Priority**: Standardize test patterns across all files (Phase 3)

This plan will significantly improve test coverage quality and make the test suite more reliable for detecting regressions in pattern matching behavior.
