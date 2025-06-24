use dcbor_parse::parse_dcbor_item;
use dcbor_pattern::{Matcher, Pattern};

fn main() {
    println!(
        "Testing dCBOR date pattern parsing with dcbor-parse integration...\n"
    );

    // Test 1: Simple date pattern
    println!("1. Simple date pattern:");
    let pattern = Pattern::parse("DATE(2023-12-25)").unwrap();
    println!("   Pattern: {}", pattern);

    // Create a matching CBOR date using dcbor-parse
    let date_cbor = parse_dcbor_item("2023-12-25").unwrap();
    let paths = pattern.paths(&date_cbor);
    println!("   Matches date CBOR: {}", !paths.is_empty());

    // Test 2: Date with time
    println!("\n2. Date with time pattern:");
    let pattern2 = Pattern::parse("DATE(2023-12-25T15:30:45Z)").unwrap();
    println!("   Pattern: {}", pattern2);

    let datetime_cbor = parse_dcbor_item("2023-12-25T15:30:45Z").unwrap();
    let paths2 = pattern2.paths(&datetime_cbor);
    println!("   Matches datetime CBOR: {}", !paths2.is_empty());

    // Test 3: Date range
    println!("\n3. Date range pattern:");
    let pattern3 = Pattern::parse("DATE(2023-12-24...2023-12-26)").unwrap();
    println!("   Pattern: {}", pattern3);

    let paths3 = pattern3.paths(&date_cbor);
    println!(
        "   Christmas day (2023-12-25) in range: {}",
        !paths3.is_empty()
    );

    // Test 4: Open-ended range (earliest)
    println!("\n4. Open-ended range (earliest) pattern:");
    let pattern4 = Pattern::parse("DATE(2023-12-24...)").unwrap();
    println!("   Pattern: {}", pattern4);

    let paths4 = pattern4.paths(&date_cbor);
    println!("   Christmas day after start date: {}", !paths4.is_empty());

    // Test 5: Open-ended range (latest)
    println!("\n5. Open-ended range (latest) pattern:");
    let pattern5 = Pattern::parse("DATE(...2023-12-26)").unwrap();
    println!("   Pattern: {}", pattern5);

    let paths5 = pattern5.paths(&date_cbor);
    println!("   Christmas day before end date: {}", !paths5.is_empty());

    // Test 6: Regex pattern
    println!("\n6. Regex pattern:");
    let pattern6 = Pattern::parse("DATE(/2023-.*/)").unwrap();
    println!("   Pattern: {}", pattern6);

    let paths6 = pattern6.paths(&date_cbor);
    println!("   Matches year 2023: {}", !paths6.is_empty());

    // Test 7: Any date pattern
    println!("\n7. Any date pattern:");
    let pattern7 = Pattern::parse("DATE").unwrap();
    println!("   Pattern: {}", pattern7);

    let paths7 = pattern7.paths(&date_cbor);
    println!("   Matches any date: {}", !paths7.is_empty());

    println!("\n✅ All date pattern parsing tests completed successfully!");
}
