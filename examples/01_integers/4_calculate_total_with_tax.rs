//! # Mixing integers and floats
//!
//! Rust never converts between numeric types implicitly. If you want
//! to multiply a `u32` by an `f64`, one of them has to change shape
//! first, and you have to say so. The `as` keyword does the conversion.
//!
//! Heads-up on rounding: `.round()` rounds to the nearest integer
//! (with ties going away from zero), while a bare `as u32` *truncates*
//! the fractional part. The tests pin down which behaviour we want
//! here, so try one and see what happens if you guessed wrong.
//!
//! See: <https://doc.rust-lang.org/std/primitive.f64.html#method.round>

/// Calculates the total price including tax.
/// Tax rate is given as a percentage (e.g., 8.5 for 8.5% tax).
fn calculate_total_with_tax(price: u32, tax_rate: f64) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_with_tax() {
        assert_eq!(calculate_total_with_tax(100, 8.5), 108);
        assert_eq!(calculate_total_with_tax(50, 10.0), 55);
        // Pin down the rounding behaviour: 100 + 8.4% = 108.4, which should
        // round to 108 (not truncate to anything else). If you used `as u32`
        // alone, this case may surprise you; try `.round()` first.
        assert_eq!(calculate_total_with_tax(100, 8.4), 108);
    }
}
