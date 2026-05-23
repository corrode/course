/// Calculates the total price (in whole cents) including tax.
///
/// Money is represented as integer cents to avoid float-precision bugs:
/// `$19.99` is `1999`. The tax rate stays an `f64` percentage, e.g.
/// `8.5` for 8.5% tax.
///
/// Any sub-cent amount in the tax is dropped (truncated toward zero),
/// so 108.4 cents becomes 108.
fn calculate_total_with_tax(price_cents: u32, tax_rate_percent: f64) -> u32 {
    todo!()
}

#[test]
fn test_calculate_total_with_tax() {
    // $100.00 with 8% tax = $108.00.
    assert_eq!(calculate_total_with_tax(10_000, 8.0), 10_800);
    // $50.00 with 10% tax = $55.00.
    assert_eq!(calculate_total_with_tax(5_000, 10.0), 5_500);
    // $100.00 with 8.5% tax = $108.50.
    assert_eq!(calculate_total_with_tax(10_000, 8.5), 10_850);
    // Sub-cent amounts must truncate, not round: 8.49% of $100 is
    // 849.something cents -> 849, not 850.
    assert_eq!(calculate_total_with_tax(10_000, 8.49), 10_849);
}
