# dcbor-pattern Crate Documentation

## Overview

This crate provides pattern matching and text processing for Deterministic CBOR (dCBOR) as implemented in the `dcbor` crate. It supports complex pattern matching with named captures, search patterns, and nested structures.

The crate is ready for community review, with complete functionality and comprehensive test coverage.

## Development Plan

`array_pattern.rs` has become quite the behemoth.

Starting linecount: 1115
Current linecount: 1114

### Refactoring Analysis for array_pattern.rs

The following redundancies and refactoring opportunities have been identified:

#### 1. Duplicate Repeat Pattern Handling Logic
**Lines affected:** ~150-200, ~180-230, ~500-550, ~600-650
**Issue:** The logic for handling repeat patterns with quantifiers (min/max count calculation, greedy backtracking) is duplicated across multiple methods:
- `backtrack_sequence_match()`
- `backtrack_sequence_assignments()`
- Similar logic in both capture and non-capture branches

**Proposed refactor:** Extract into helper functions:
```rust
fn calculate_repeat_bounds(&self, quantifier: &Quantifier, remaining_elements: usize) -> (usize, usize)
fn try_repeat_match(&self, repeat_pattern: &RepeatPattern, arr: &[CBOR], start_idx: usize, count: usize) -> bool
```

#### 2. Pattern Type Detection Logic
**Lines affected:** ~400-450, ~470-520, ~800-850
**Issue:** Pattern type checking (is_repeat, is_capture_with_repeat, etc.) is scattered and repeated throughout the code.

**Proposed refactor:** Extract into helper functions:
```rust
fn is_repeat_pattern(pattern: &Pattern) -> bool
fn is_capture_with_repeat(pattern: &Pattern) -> Option<&RepeatPattern>
fn extract_repeat_pattern(pattern: &Pattern) -> Option<&RepeatPattern>
```

#### 3. Backtracking State Management
**Lines affected:** ~500-700 (assignment tracking), ~150-300 (match tracking)
**Issue:** Similar backtracking logic for both boolean matching and assignment tracking, with nearly identical state management.

**Proposed refactor:** Generic backtracking framework:
```rust
trait BacktrackState {
    fn save_state(&self) -> Self;
    fn restore_state(&mut self, saved: Self);
}

fn backtrack_with_state<S: BacktrackState>(&self, patterns: &[Pattern], arr: &[CBOR], state: &mut S) -> bool
```

#### 4. Element-to-Pattern Assignment Logic
**Lines affected:** ~320-380, ~600-750
**Issue:** Complex logic for mapping array elements to sequence patterns is duplicated between matching and capture collection.

**Proposed refactor:** Extract into dedicated struct:
```rust
struct SequenceAssigner<'a> {
    patterns: &'a [Pattern],
    arr: &'a [CBOR],
}

impl SequenceAssigner {
    fn find_assignments(&self) -> Option<Vec<(usize, usize)>>
    fn can_match(&self) -> bool
}
```

#### 5. Capture Context Path Building
**Lines affected:** ~280-320, ~350-400, ~750-800
**Issue:** Building array context paths for captures is repeated in multiple places with slight variations.

**Proposed refactor:** Extract helper functions:
```rust
fn build_array_context_path(array_cbor: &CBOR, element: &CBOR, inner_path: &[CBOR]) -> Vec<CBOR>
fn build_sub_array_context_path(array_cbor: &CBOR, elements: &[CBOR]) -> Vec<CBOR>
```

#### 6. VM Compilation Branches
**Lines affected:** ~900-1000
**Issue:** Similar logic for checking captures and choosing between simple/complex compilation paths.

**Proposed refactor:** Extract into helper:
```rust
fn has_captures(&self) -> bool
fn compile_with_captures(&self, code: &mut Vec<Instr>, literals: &mut Vec<Pattern>, captures: &mut Vec<String>)
fn compile_without_captures(&self, code: &mut Vec<Instr>, literals: &mut Vec<Pattern>)
```

#### 7. Pattern Display Formatting
**Lines affected:** ~1050-1115
**Issue:** The array element pattern formatting could be part of a more general pattern formatting utility.

**Proposed refactor:** Move to a shared formatting module or trait.

#### Impact Assessment:
- **Estimated line reduction:** 300-400 lines (25-35% reduction)
- **Complexity reduction:** High - eliminates major code duplication
- **Maintainability improvement:** High - centralized logic for common operations
- **Risk level:** Medium - requires careful testing of backtracking logic

#### Recommended Refactoring Order:
1. Start with helper functions (#1, #2, #5) - lowest risk
2. Extract assignment logic (#4) - medium risk but high impact
3. Implement generic backtracking (#3) - highest risk but biggest payoff
4. Clean up compilation and formatting (#6, #7) - low risk cleanup

#### Refactoring Results - Pattern Type Detection Logic (#2)

**COMPLETED** - ✅

**Target:** Extract pattern type detection helper functions to eliminate scattered pattern matching logic.

**Changes made:**
- Added 4 new helper functions:
  - `is_repeat_pattern(pattern: &Pattern) -> bool`
  - `extract_capture_with_repeat(pattern: &Pattern) -> Option<&RepeatPattern>`
  - `extract_repeat_pattern(pattern: &Pattern) -> Option<&RepeatPattern>`
  - `has_repeat_patterns(patterns: &[Pattern]) -> bool`
- Replaced 6 instances of duplicated pattern type checking throughout the file
- Consolidated pattern detection logic into reusable helper functions

**Line count reduction:** 1115 → 1119 lines (net +4 lines due to helper functions, but eliminated ~20 lines of duplicate logic)

**Benefits:**
- ✅ Eliminated code duplication in pattern type detection
- ✅ Improved readability and maintainability
- ✅ Centralized pattern matching logic for easier future changes
- ✅ All tests pass - no functional changes

**Next recommended target:** Duplicate Repeat Pattern Handling Logic (#1)
- This target offers the biggest potential line reduction (estimated 50-70 lines)
- Focuses on the quantifier calculation and greedy backtracking logic that appears in multiple methods
- Medium complexity but significant impact on code maintainability

#### Refactoring Results - Duplicate Repeat Pattern Handling Logic (#1)

**COMPLETED** - ✅

**Target:** Extract duplicate quantifier calculation and greedy backtracking logic.

**Changes made:**
- Added 4 new helper functions:
  - `calculate_repeat_bounds(quantifier, element_idx, arr_len) -> (usize, usize)`
  - `can_repeat_match(repeat_pattern, arr, element_idx, rep_count) -> bool`
  - `try_repeat_backtrack_match()` - For boolean sequence matching
  - `try_repeat_backtrack_assignments()` - For assignment tracking
- Replaced 4 instances of duplicated repeat pattern logic in:
  - `backtrack_sequence_match()` - direct repeat patterns
  - `backtrack_sequence_match()` - capture+repeat patterns
  - `backtrack_sequence_assignments()` - direct repeat patterns
  - `backtrack_sequence_assignments()` - capture+repeat patterns

**Line count reduction:** 1135 → 1095 lines (**-40 lines**, 3.5% reduction)

**Benefits:**
- ✅ Eliminated ~80 lines of duplicate quantifier/backtracking logic
- ✅ Centralized complex repeat pattern matching into reusable functions
- ✅ Improved maintainability - changes to repeat logic only need to be made in one place
- ✅ All tests pass - no functional changes
- ✅ Significantly reduced code complexity in core matching methods

**Next recommended target:** Capture Context Path Building (#5)
- Lower complexity than backtracking framework (#3) or assignment logic (#4)
- Clear duplication in path building for captures
- Estimated 15-25 line reduction with high maintainability benefit
