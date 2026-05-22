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
