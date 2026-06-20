/// Returns `n!` (n factorial). By convention, `factorial(0) == 1`.
///
/// Build it up with a `mut` accumulator and a `for` loop over the
/// inclusive range `1..=n`. For `n == 0` the loop body never runs,
/// so the initial value carries through unchanged.
fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(10), 3_628_800);
}
