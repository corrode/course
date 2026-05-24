/// Applies a damage bonus to a base damage value.
///
/// The bonus is a percentage *added on top* of the base, so
/// `50.0` means "+50%": a base of `100` becomes `150`. Think of
/// it as a critical-hit bonus, an equipment buff, or any other
/// damage modifier expressed as a percentage. Fractional HP is
/// dropped (truncated toward zero), which is what most games do;
/// half-HP doesn't exist.
fn damage_with_bonus(base: u32, bonus_percent: f64) -> u32 {
    todo!()
}

#[test]
fn test_damage_with_bonus() {
    // No bonus: damage is unchanged.
    assert_eq!(damage_with_bonus(100, 0.0), 100);
    // Classic +50% bonus (e.g. a critical hit).
    assert_eq!(damage_with_bonus(100, 50.0), 150);
    // +100% = double damage.
    assert_eq!(damage_with_bonus(80, 100.0), 160);
    // Fractional HP truncates toward zero:
    // 7 + 7 * 0.155 = 8.085 → 8, not 9.
    assert_eq!(damage_with_bonus(7, 15.5), 8);
}
