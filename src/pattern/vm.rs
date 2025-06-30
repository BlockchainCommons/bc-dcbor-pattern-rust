//! Tiny Thompson-style VM for walking dCBOR trees.
//!
//! The VM runs byte-code produced by `Pattern::compile` methods.

use std::collections::{HashMap, HashSet};

use dcbor::prelude::*;

use super::{Matcher, Path, Pattern};
use crate::{Quantifier, Reluctance};

/// Navigation axis for traversing dCBOR tree structures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// Navigate to array elements
    ArrayElement,
    /// Navigate to map keys
    MapKey,
    /// Navigate to map values
    MapValue,
    /// Navigate to tagged value content
    TaggedContent,
}

impl Axis {
    /// Return child CBOR values reachable from `cbor` via this axis.
    pub fn children(&self, cbor: &CBOR) -> Vec<CBOR> {
        match (self, cbor.as_case()) {
            (Axis::ArrayElement, CBORCase::Array(arr)) => arr.clone(),
            (Axis::MapKey, CBORCase::Map(map)) => {
                map.iter().map(|(k, _v)| k.clone()).collect()
            }
            (Axis::MapValue, CBORCase::Map(map)) => {
                map.iter().map(|(_k, v)| v.clone()).collect()
            }
            (Axis::TaggedContent, CBORCase::Tagged(_, content)) => {
                vec![content.clone()]
            }
            _ => Vec::new(),
        }
    }
}

/// Bytecode instructions for the pattern VM.
#[derive(Debug, Clone)]
pub enum Instr {
    /// Match predicate: `literals[idx].matches(cbor)`
    MatchPredicate(usize),
    /// Match structure: use `literals[idx].paths(cbor)` for structure patterns
    MatchStructure(usize),
    /// ε-split: fork execution to `a` and `b`
    Split { a: usize, b: usize },
    /// Unconditional jump to instruction at index
    Jump(usize),
    /// Descend to children via axis, one thread per child
    PushAxis(Axis),
    /// Pop one CBOR value from the path
    Pop,
    /// Emit current path
    Save,
    /// Final accept, emit current path and halt thread
    Accept,
    /// Recursively search for pattern at `pat_idx` and propagate captures
    Search {
        pat_idx: usize,
        capture_map: Vec<(String, usize)>,
    },
    /// Save current path and start new sequence from last CBOR value
    ExtendSequence,
    /// Combine saved path with current path for final result
    CombineSequence,
    /// Match only if pattern at `pat_idx` does not match
    NotMatch { pat_idx: usize },
    /// Repeat a sub pattern according to range and greediness
    Repeat {
        pat_idx: usize,
        quantifier: Quantifier,
    },
    /// Mark the start of a capture group
    CaptureStart(usize),
    /// Mark the end of a capture group
    CaptureEnd(usize),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub code: Vec<Instr>,
    pub literals: Vec<Pattern>,
    pub capture_names: Vec<String>,
}

/// Internal back-tracking state.
#[derive(Clone)]
struct Thread {
    pc: usize,
    cbor: CBOR,
    path: Path,
    /// Stack of saved paths for nested sequence patterns
    saved_paths: Vec<Path>,
    captures: Vec<Vec<Path>>,
    capture_stack: Vec<Vec<usize>>,
}

/// Match atomic patterns without recursion into the VM.
///
/// This function handles only the patterns that are safe to use in
/// MatchPredicate instructions - Value, Structure, Any, and None patterns. Meta
/// patterns should never be passed to this function as they need to be compiled
/// to bytecode.
#[allow(clippy::panic)]
pub(crate) fn atomic_paths(
    p: &crate::pattern::Pattern,
    cbor: &CBOR,
) -> Vec<Path> {
    use crate::pattern::Pattern::*;
    match p {
        Value(v) => v.paths(cbor),
        Structure(s) => s.paths(cbor),
        Meta(meta) => match meta {
            crate::pattern::meta::MetaPattern::Any(_) => {
                vec![vec![cbor.clone()]]
            }
            crate::pattern::meta::MetaPattern::Search(_) => {
                panic!(
                    "SearchPattern should be compiled to Search instruction, not MatchPredicate"
                );
            }
            _ => panic!(
                "non-atomic meta pattern used in MatchPredicate: {:?}",
                meta
            ),
        },
    }
}

fn repeat_paths(
    pat: &Pattern,
    cbor: &CBOR,
    path: &Path,
    quantifier: Quantifier,
) -> Vec<(CBOR, Path)> {
    // Build states for all possible repetition counts
    let mut states: Vec<Vec<(CBOR, Path)>> =
        vec![vec![(cbor.clone(), path.clone())]];
    let bound = quantifier.max().unwrap_or(usize::MAX);

    // Try matching the pattern repeatedly
    for _ in 0..bound {
        let mut next = Vec::new();
        for (c, pth) in states.last().unwrap().iter() {
            for sub_path in pat.paths(c) {
                if let Some(last) = sub_path.last() {
                    if last.to_cbor_data() == c.to_cbor_data() {
                        continue; // Avoid infinite loops
                    }
                    let mut combined = pth.clone();
                    if sub_path.first() == Some(c) {
                        combined.extend(sub_path.iter().skip(1).cloned());
                    } else {
                        combined.extend(sub_path.iter().cloned());
                    }
                    next.push((last.clone(), combined));
                }
            }
        }
        if next.is_empty() {
            break; // No more matches possible
        }
        states.push(next);
    }

    // Zero repetition case
    let has_zero_rep = quantifier.min() == 0;
    let zero_rep_result = if has_zero_rep {
        vec![(cbor.clone(), path.clone())]
    } else {
        vec![]
    };

    // Calculate maximum allowed repetitions
    let max_possible = states.len() - 1;
    let max_allowed = bound.min(max_possible);

    // Check if we can satisfy the minimum repetition requirement
    if max_allowed < quantifier.min() && quantifier.min() > 0 {
        return Vec::new();
    }

    // Calculate the range of repetition counts based on min and max
    // Ensure we don't include zero here - it's handled separately
    let min_count = if quantifier.min() == 0 {
        1
    } else {
        quantifier.min()
    };
    let max_count = if max_allowed < min_count {
        return zero_rep_result;
    } else {
        max_allowed
    };

    let count_range = min_count..=max_count;

    // Generate list of counts to try based on reluctance
    let counts: Vec<usize> = match quantifier.reluctance() {
        Reluctance::Greedy => count_range.rev().collect(),
        Reluctance::Lazy => count_range.collect(),
        Reluctance::Possessive => {
            if max_count >= min_count {
                vec![max_count]
            } else {
                vec![]
            }
        }
    };

    // Collect results based on the counts determined above
    let mut out = Vec::new();

    // For greedy repetition, try higher counts first
    if matches!(quantifier.reluctance(), Reluctance::Greedy) {
        // Include results from counts determined by reluctance
        for c in counts {
            if let Some(list) = states.get(c) {
                out.extend(list.clone());
            }
        }

        // For greedy matching, add zero repetition case at the end if
        // applicable
        if has_zero_rep && out.is_empty() {
            out.push((cbor.clone(), path.clone()));
        }
    } else {
        // For lazy/possessive, include zero repetition first if applicable
        if has_zero_rep {
            out.push((cbor.clone(), path.clone()));
        }

        // Then include results from counts determined by reluctance
        for c in counts {
            if let Some(list) = states.get(c) {
                out.extend(list.clone());
            }
        }
    }

    out
}

/// Execute `prog` starting at `root`. Every time `SAVE` or `ACCEPT` executes,
/// current `path` is pushed into result.
/// Execute a single thread until it halts. Returns true if any paths were
/// produced.
fn run_thread(
    prog: &Program,
    start: Thread,
    out: &mut Vec<(Path, Vec<Vec<Path>>)>,
) -> bool {
    use Instr::*;
    let mut produced = false;
    let mut stack = vec![start];

    while let Some(mut th) = stack.pop() {
        loop {
            match prog.code[th.pc] {
                MatchPredicate(idx) => {
                    if atomic_paths(&prog.literals[idx], &th.cbor).is_empty() {
                        break;
                    }
                    th.pc += 1;
                }
                MatchStructure(idx) => {
                    // Use the structure pattern's matcher, with captures if
                    // present
                    if let crate::pattern::Pattern::Structure(sp) =
                        &prog.literals[idx]
                    {
                        let (structure_paths, structure_captures) =
                            sp.paths_with_captures(&th.cbor);

                        if structure_paths.is_empty() {
                            break;
                        }

                        // Merge structure captures into thread captures
                        for (i, name) in prog.capture_names.iter().enumerate() {
                            if let Some(captured_paths) =
                                structure_captures.get(name)
                            {
                                // Ensure capture storage is initialized
                                while th.captures.len() <= i {
                                    th.captures.push(Vec::new());
                                }
                                th.captures[i].extend(captured_paths.clone());
                            }
                        }

                        // Handle structure paths
                        if structure_paths.len() == 1
                            && structure_paths[0].len() == 1
                        {
                            // Simple case: single path with single element
                            th.pc += 1;
                        } else {
                            // Complex case: multiple paths or multi-element
                            // paths
                            for structure_path in structure_paths {
                                if let Some(target) = structure_path.last() {
                                    let mut new_thread = th.clone();
                                    new_thread.cbor = target.clone();
                                    new_thread.path.extend(
                                        structure_path.iter().skip(1).cloned(),
                                    );
                                    new_thread.pc += 1;
                                    stack.push(new_thread);
                                }
                            }
                            break;
                        }
                    } else {
                        panic!(
                            "MatchStructure used with non-structure pattern"
                        );
                    }
                }
                Split { a, b } => {
                    let mut th2 = th.clone();
                    th2.pc = b;
                    stack.push(th2);
                    th.pc = a;
                }
                Jump(addr) => {
                    th.pc = addr;
                }
                PushAxis(axis) => {
                    let children = axis.children(&th.cbor);
                    for child in children {
                        let mut new_thread = th.clone();
                        new_thread.cbor = child.clone();
                        new_thread.path.push(child);
                        new_thread.pc += 1;
                        stack.push(new_thread);
                    }
                    break;
                }
                Pop => {
                    if th.path.is_empty() {
                        break;
                    }
                    th.path.pop();
                    if let Some(parent) = th.path.last() {
                        th.cbor = parent.clone();
                    }
                    th.pc += 1;
                }
                Save => {
                    out.push((th.path.clone(), th.captures.clone()));
                    produced = true;
                    th.pc += 1;
                }
                Accept => {
                    out.push((th.path.clone(), th.captures.clone()));
                    produced = true;
                    break;
                }
                Search { pat_idx, ref capture_map } => {
                    // Implement recursive search pattern with capture support
                    let (search_results, captures) =
                        prog.literals[pat_idx].paths_with_captures(&th.cbor);

                    for search_path in search_results {
                        let mut new_thread = th.clone();
                        new_thread.path = search_path.clone();

                        // Apply capture mappings - map captured paths to thread
                        // state
                        for (name, capture_idx) in capture_map {
                            if *capture_idx < new_thread.captures.len() {
                                if let Some(capture_paths) = captures.get(name)
                                {
                                    for capture_path in capture_paths {
                                        new_thread.captures[*capture_idx]
                                            .push(capture_path.clone());
                                    }
                                }
                            }
                        }

                        new_thread.pc += 1;
                        stack.push(new_thread);
                    }
                    break;
                }
                ExtendSequence => {
                    th.saved_paths.push(th.path.clone());
                    if let Some(last) = th.path.last().cloned() {
                        th.path = vec![last.clone()];
                        th.cbor = last;
                    }
                    th.pc += 1;
                }
                CombineSequence => {
                    if let Some(saved) = th.saved_paths.pop() {
                        let mut combined = saved;
                        if th.path.len() > 1 {
                            combined.extend(th.path.iter().skip(1).cloned());
                        }
                        th.path = combined;
                    }
                    th.pc += 1;
                }
                NotMatch { pat_idx } => {
                    if !prog.literals[pat_idx].paths(&th.cbor).is_empty() {
                        break; // Pattern matched, so NOT pattern fails
                    }
                    th.pc += 1;
                }
                Repeat { pat_idx, quantifier } => {
                    let repeat_results = repeat_paths(
                        &prog.literals[pat_idx],
                        &th.cbor,
                        &th.path,
                        quantifier,
                    );
                    for (result_cbor, result_path) in repeat_results {
                        let mut new_thread = th.clone();
                        new_thread.cbor = result_cbor;
                        new_thread.path = result_path;
                        new_thread.pc += 1;
                        stack.push(new_thread);
                    }
                    break;
                }
                CaptureStart(idx) => {
                    // Initialize capture group
                    while th.captures.len() <= idx {
                        th.captures.push(Vec::new());
                    }
                    while th.capture_stack.len() <= idx {
                        th.capture_stack.push(Vec::new());
                    }
                    // Store the current path to capture it at CaptureEnd
                    th.capture_stack[idx].push(th.path.len());
                    th.pc += 1;
                }
                CaptureEnd(idx) => {
                    // Finalize capture group
                    if let Some(_start_len) = th
                        .capture_stack
                        .get_mut(idx)
                        .and_then(|stack| stack.pop())
                    {
                        // For captures, we want to capture the full path to the
                        // current CBOR value
                        // not just the delta since CaptureStart
                        let captured_path = th.path.clone();
                        if let Some(captures) = th.captures.get_mut(idx) {
                            captures.push(captured_path);
                        }
                    }
                    th.pc += 1;
                }
            }
        }
    }

    produced
}

/// Execute a program against a dCBOR value, returning all matching paths and
/// captures.
pub fn run(
    prog: &Program,
    root: &CBOR,
) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
    let start = Thread {
        pc: 0,
        cbor: root.clone(),
        path: vec![root.clone()],
        saved_paths: Vec::new(),
        captures: Vec::new(),
        capture_stack: Vec::new(),
    };

    let mut results = Vec::new();
    run_thread(prog, start, &mut results);

    // Deduplicate paths while preserving original order
    let mut seen_paths = HashSet::new();
    let paths: Vec<Path> = results
        .iter()
        .filter_map(|(path, _)| {
            if seen_paths.contains(path) {
                None // Already seen, skip
            } else {
                seen_paths.insert(path.clone());
                Some(path.clone()) // First occurrence, keep
            }
        })
        .collect();

    // Build capture map from capture names and results
    // Collect all captured paths from all threads, then deduplicate per capture
    // while preserving order
    let mut captures = HashMap::new();
    for (i, name) in prog.capture_names.iter().enumerate() {
        let mut captured_paths = Vec::new();
        for (_, thread_captures) in &results {
            if let Some(capture_group) = thread_captures.get(i) {
                captured_paths.extend(capture_group.clone());
            }
        }

        // Deduplicate captured paths for this capture name while preserving
        // order
        if !captured_paths.is_empty() {
            let mut seen_capture_paths = HashSet::new();
            let deduplicated_captured_paths: Vec<Path> = captured_paths
                .into_iter()
                .filter(|path| {
                    if seen_capture_paths.contains(path) {
                        false // Already seen, skip
                    } else {
                        seen_capture_paths.insert(path.clone());
                        true // First occurrence, keep
                    }
                })
                .collect();
            captures.insert(name.clone(), deduplicated_captured_paths);
        }
    }

    (paths, captures)
}

/// VM for executing pattern programs against dCBOR values.
pub struct Vm;

impl Vm {
    /// Execute a program against a dCBOR value.
    pub fn run(
        prog: &Program,
        root: &CBOR,
    ) -> (Vec<Path>, HashMap<String, Vec<Path>>) {
        run(prog, root)
    }
}
