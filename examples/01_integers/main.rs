//! # Integers
//!
//! Numbers are everywhere in programming, from calculating a grocery bill to
//! powering the algorithms that sent humans to the moon (thanks, Margaret
//! Hamilton!). Integer arithmetic is fundamental to almost everything we do.
//!
//! Fun fact: the first computer bug was literally a bug. Grace Hopper found
//! a moth stuck in a Harvard Mark II computer in 1947. Today we'll focus on
//! avoiding logical bugs in our number handling code.

/// Convert a number to a string
fn number_to_string(number: u32) -> String {
    todo!()
}

#[test]
fn test_number_to_string() {
    assert_eq!(number_to_string(1234), "1234");
    assert_eq!(number_to_string(0), "0");
}

/// Calculates the total price including tax.
/// Tax rate is given as a percentage (e.g., 8.5 for 8.5% tax).
fn calculate_total_with_tax(price: u32, tax_rate: f64) -> u32 {
    todo!()
}

#[test]
fn test_calculate_total_with_tax() {
    assert_eq!(calculate_total_with_tax(100, 8.5), 108);
    assert_eq!(calculate_total_with_tax(50, 10.0), 55);
    // Pin down the rounding behaviour: 100 + 8.4% = 108.4, which should
    // round to 108 (not truncate to anything else). If you used `as u32`
    // alone, this case may surprise you; try `.round()` first.
    assert_eq!(calculate_total_with_tax(100, 8.4), 108);
}

/// Parses a string into a positive integer.
/// Returns the number if valid, 0 if invalid.
///
/// Note: returning `0` on failure is a bad idea in real code. It
/// silently merges "the input was the number zero" with "the input was
/// garbage". Rust has a much better tool for this: `Result`. We use the
/// lossy version here only because we haven't met `Result` yet; chapter 7
/// (`Option`) and chapter 8 (`Result`) introduce the proper way.
fn parse_positive_integer(input: &str) -> u32 {
    todo!()
}

#[test]
fn test_parse_positive_integer() {
    assert_eq!(parse_positive_integer("123"), 123);
    assert_eq!(parse_positive_integer("0"), 0);
    assert_eq!(parse_positive_integer("invalid"), 0);
    assert_eq!(parse_positive_integer("-5"), 0);
}
