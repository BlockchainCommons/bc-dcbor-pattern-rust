# Examples

This directory contains demonstration examples for the `dcbor-pattern` crate.

## Running Examples

Run any example with:
```bash
cargo run --example <example_name>
```

## Available Examples

### Core Pattern Types

- **`null_pattern_demo.rs`** - Demonstrates NULL pattern matching
- **`date_pattern_demo.rs`** - Shows date pattern parsing and matching with various formats
- **`map_pattern_demo.rs`** - Basic MAP pattern examples with length constraints
- **`map_constraints_demo.rs`** - Advanced MAP pattern with key-value constraints

### Advanced Features

- **`capture_pattern_demo.rs`** - Pattern capture groups and named captures
- **`format_demo.rs`** - Different formatting options for displaying matched paths

### Integration

- **`dcbor_parse_integration_demo.rs`** - Integration with `dcbor-parse` for parsing CBOR diagnostic notation

## Notes

- All examples use the `dcbor-parse` crate for easy CBOR value creation from diagnostic notation
- Examples demonstrate both programmatic pattern creation and text parsing
- Run `cargo test` to verify all examples still work correctly
