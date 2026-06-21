/// Returns the sum `1 + 2 + ... + n`, computed recursively.
/// By convention, `sum_to(0)` is `0`.
///
/// Each call returns its number plus the sum of everything below it;
/// `n == 0` is the base case that returns `0` and stops the recursion.
fn sum_to(n: u32) -> u32 {
    if n == 0 { 0 } else { n + sum_to(n - 1) }
}

#[test]
fn test_sum_to() {
    assert_eq!(sum_to(0), 0);
    assert_eq!(sum_to(1), 1);
    assert_eq!(sum_to(3), 6); // 1 + 2 + 3
    assert_eq!(sum_to(10), 55);
    assert_eq!(sum_to(100), 5_050);
}
