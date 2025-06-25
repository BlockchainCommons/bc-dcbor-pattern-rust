/// Example demonstrating the use of the format module
use dcbor::prelude::*;
use dcbor_pattern::{
    FormatPathsOpts, PathElementFormat, format_paths, format_paths_opt,
};

fn main() {
    // Create some sample paths for demonstration
    let path1 = vec![
        CBOR::from(1),
        CBOR::from("hello"),
        CBOR::from(vec![1, 2, 3]),
    ];

    let path2 = vec![
        CBOR::from(42),
        CBOR::from("world"),
        CBOR::from({
            let mut map = dcbor::Map::new();
            map.insert("key", "value");
            map
        }),
    ];

    let paths = vec![path1, path2];

    println!("=== Default formatting ===");
    println!("{}", format_paths(&paths));

    println!("\n=== Flat diagnostic formatting ===");
    let opts = FormatPathsOpts::new()
        .element_format(PathElementFormat::DiagnosticSummary(None));
    println!("{}", format_paths_opt(&paths, opts));

    println!("\n=== Last element only ===");
    let opts = FormatPathsOpts::new().last_element_only(true);
    println!("{}", format_paths_opt(&paths, opts));

    println!("\n=== Truncated elements ===");
    let opts = FormatPathsOpts::new()
        .element_format(PathElementFormat::DiagnosticSummary(Some(10)));
    println!("{}", format_paths_opt(&paths, opts));

    println!("\n=== No indentation ===");
    let opts = FormatPathsOpts::new().indent(false);
    println!("{}", format_paths_opt(&paths, opts));
}
