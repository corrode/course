/// Returns the number of decimal digits in `n`. `digit_count(0)` is `1`.
///
/// Use a `while` loop. The "divide by 10 until you hit zero" pattern is the
/// classic solution for this kind of "I don't know how many iterations up
/// front" problem.
fn digit_count(n: u32) -> u32 {
    todo!()
}

#[test]
fn test_digit_count() {
    assert_eq!(digit_count(0), 1);
    assert_eq!(digit_count(7), 1);
    // Boundary check: 10 has two digits, not one.
    assert_eq!(digit_count(10), 2);
    assert_eq!(digit_count(42), 2);
    assert_eq!(digit_count(99), 2);
    assert_eq!(digit_count(100), 3);
    assert_eq!(digit_count(999), 3);
    assert_eq!(digit_count(1_000_000), 7);
    assert_eq!(digit_count(u32::MAX), 10);
}
