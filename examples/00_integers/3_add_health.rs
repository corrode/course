/// Adds `gain` health to `current`, capping at `u8::MAX` (255) instead of
/// overflowing.
fn add_health(current: u8, gain: u8) -> u8 {
    todo!()
}

#[test]
fn test_add_health() {
    assert_eq!(add_health(100, 50), 150);
    // A plain `current + gain` would panic in debug and wrap to 44 in release.
    // Saturating arithmetic clamps at the maximum instead.
    assert_eq!(add_health(200, 100), 255);
    assert_eq!(add_health(255, 1), 255);
    assert_eq!(add_health(0, 0), 0);
}
