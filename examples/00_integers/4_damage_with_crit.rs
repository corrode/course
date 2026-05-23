/// Applies a critical-hit bonus to a base damage value.
///
/// The bonus is a percentage *added on top* of the base, so
/// `50.0` means "+50%": a base of `100` becomes `150`. Fractional
/// HP is dropped (truncated toward zero), which is what most
/// games do — half-HP doesn't exist.
fn damage_with_crit(base: u32, crit_bonus_percent: f64) -> u32 {
    todo!()
}

#[test]
fn test_damage_with_crit() {
    // No crit: damage is unchanged.
    assert_eq!(damage_with_crit(100, 0.0), 100);
    // Classic +50% crit.
    assert_eq!(damage_with_crit(100, 50.0), 150);
    // +100% = double damage.
    assert_eq!(damage_with_crit(80, 100.0), 160);
    // Fractional HP truncates toward zero:
    // 7 + 7 * 0.155 = 8.085 → 8, not 9.
    assert_eq!(damage_with_crit(7, 15.5), 8);
}
