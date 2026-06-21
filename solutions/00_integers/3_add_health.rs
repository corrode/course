/// Adds `gain` health to `current`, capping at `u8::MAX` (255) instead of
/// overflowing.
///
/// `saturating_add` performs the addition but clamps the result at the type's
/// maximum, so a big stack of buffs tops out at 255 rather than wrapping around
/// to a small number (which is what a plain `+` would do in a release build).
fn add_health(current: u8, gain: u8) -> u8 {
    current.saturating_add(gain)
}

#[test]
fn test_add_health() {
    assert_eq!(add_health(100, 50), 150);
    assert_eq!(add_health(200, 100), 255);
    assert_eq!(add_health(255, 1), 255);
    assert_eq!(add_health(0, 0), 0);
}
