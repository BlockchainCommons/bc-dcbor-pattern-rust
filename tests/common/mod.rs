/// A macro to assert that two values are equal, printing them if they are not,
/// including newlines and indentation they may contain. This macro is useful
/// for debugging tests where you want to see the actual and expected values
/// when they do not match, especially for complex structures like CBOR data
/// where the output may be multi-line or contain significant whitespace.
///
/// General usage rubric:
/// - Run your test with this macro, and a dummy expected value so the test
///   fails.
/// - Observe the actual output printed to the console.
/// - If it is not what you expected, adjust your test or the expected value.
/// - Replace the dummy expected value with the correct one once you have
///   verified the actual output.
/// - If the output contains multiple lines, use the `indoc!` macro:
///
/// ```rust
/// // expected-text-output-rubric:
/// #[rustfmt::skip]
/// let expected = indoc! {r#"
///     Expected output
/// "#}.trim();
/// assert_actual_expected!(generate_actual_output(), expected);
/// ```
#[macro_export]
macro_rules! assert_actual_expected {
    ($actual:expr, $expected:expr $(,)?) => {
        match (&$actual, &$expected) {
            (actual_val, expected_val) => {
                if !(*actual_val == *expected_val) {
                    println!("Actual:\n{actual_val}\nExpected:\n{expected_val}");
                    assert_eq!(*actual_val, *expected_val);
                }
            }
        }
    };
    ($actual:expr, $expected:expr, $($arg:tt)+) => {
        match (&$actual, &$expected) {
            (actual_val, expected_val) => {
                if !(*actual_val == *expected_val) {
                    println!("Actual:\n{actual_val}\nExpected:\n{expected_val}");
                    assert_eq!(*actual_val, *expected_val, $crate::option::Option::Some($crate::format_args!($($arg)+)));
                }
            }
        }
    };
}
