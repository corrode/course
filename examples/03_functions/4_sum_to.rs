//! Exercise: write a function `sum_to` that returns the sum
//! `1 + 2 + ... + n`. By convention, `sum_to(0)` is `0`.
//!
//! Solve it recursively. The pattern is the same as `countdown`,
//! but this time each call returns a value that the caller adds to
//! its own work. Don't use a loop, and don't use the closed-form
//! `n * (n + 1) / 2`; the point is the recursion.

#[test]
fn test_sum_to() {
    assert_eq!(sum_to(0), 0);
    assert_eq!(sum_to(1), 1);
    assert_eq!(sum_to(3), 6); // 1 + 2 + 3
    assert_eq!(sum_to(10), 55);
    assert_eq!(sum_to(100), 5_050);
}
