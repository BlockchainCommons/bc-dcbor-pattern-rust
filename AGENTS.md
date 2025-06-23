# `dcbor-pattern` Crate Documentation

This file contains general information for contributors to the `dcbor-pattern` crate, which provides a pattern matcher and text syntax pattern parser for Deterministic CBOR (dCBOR) as implemented in the `dcbor` crate in this workspace. Further documentation including the pattern expression syntax can be found in the `docs/` directory. Make sure to read those before starting on any tasks.

## General Guidance

This crate is just a skeleton at the moment, based on the `bc-envelope-pattern` crate. You will be receiving tasks to implement the pattern matcher and text syntax parser for dCBOR. Always make sure that `cargo test` and `cargo clippy` pass before you're done with your changes.

## Important Differences between `dcbor-pattern` and `bc-envelope-pattern`

- This crate is focused on deterministic CBOR (dCBOR) patterns, while `bc-envelope-pattern` is focused on Gordian Envelope patterns.
- `bc-envelope-pattern` will eventually depend on `dcbor-pattern` for its LEAF pattern matching.
- This crate, `dcbor-pattern`, will not depend on `bc-envelope-pattern` as it is focused on the lower-level dCBOR patterns. It should never refer to Gordian Envelope concepts like subjects, assertions, or predicates.
- Some concepts mentioned in `bc-envelope-pattern` are properly concepts of dCBOR, such as dates, known values, and the like. These concepts will be implemented in this crate, `dcbor-pattern`.
- The concept of `Path` in this crate is analogous to the `Path` in `bc-envelope-pattern`, but each path element is a `CBOR` object, not an `Envelope`.
- `CBOR` objects, like `Envelope` objects, are trees. But the branching points of `CBOR` are its compound structures like arrays and maps, not assertions and wrapped envelopes.
- Both crates have analogous modules, such as `quantifier`.
- Both crates have analogous folder hierarchy, such as `pattern` and `parser`.
- A main difference is that `dcbor-pattern` refers to `value` patterns intead of `leaf` patterns. `value` patterns are atomic `CBOR` values, while `leaf` patterns in `bc-envelope-pattern` are *any* CBOR value, including compound structures like arrays and maps.

## Crates in this Workspace

You will only be making changes to the `dcbor-pattern` crate, but it is important to understand the other crates in this workspace as they provide the context and dependencies for your work:

- `dcbor-pattern`: The crate you are currently working on, which provides the pattern matching and text syntax parsing functionality for dCBOR.
- `dcbor`: The core crate for deterministic CBOR, which provides the basic data structures and functionality for working with dCBOR values.
- `dcbor-parse`: A parser for dCBOR diagnostic notation, which is used to specify patterns in a human-readable format. You will use this crate to parse CBOR diagnostic notation into `CBOR` values.
- `bc-envelope`: The core crate for Gordian Envelope, which provides the basic data structures and functionality for working with Gordian Envelope.
- `bc-envelope-pattern`: A crate that provides pattern matching and text syntax parsing functionality for Gordian Envelope, which will eventually depend on `dcbor-pattern` for its LEAF pattern matching.

## Current Tasks

Every task for now will require you to compare the analogous implementation in `bc-envelope-pattern` and adapt it to the `dcbor-pattern` crate.

At the moment we are working on building out the `pattern::value` module, including the programmatic API for composing patterns and the VM for matching patterns against dCBOR values.

For a given `value` pattern, you will need to:

- Look at the `bc-envelope-pattern` crate's `pattern::leaf` module for inspiration.
- Implement the `pattern::value` module in `dcbor-pattern`, ensuring that it can handle the specific requirements of dCBOR patterns.
- Implement the `parser` module to parse text syntax patterns into `value` patterns.
- Implement unit tests for the `value` patterns, ensuring that they cover all edge cases and conform to the dCBOR specifications.
- Implement integration tests in the `tests` directory to ensure that the `value` patterns work correctly with the dCBOR values.
