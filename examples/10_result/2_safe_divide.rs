/// Divides `dividend` by `divisor`.
///
/// Returns `Ok(quotient)` when the division is well-defined, or
/// `Err("cannot divide by zero")` when `divisor` is `0.0`.
///
/// Start here. The simplest way to produce a `Result`: an `if` checks
/// the failure case, the `else` branch returns `Ok(...)`.
///
/// The signature is the interesting part: `&'static str` for the error
/// is the simplest possible error type and is fine while you're learning.
fn safe_divide(dividend: f64, divisor: f64) -> Result<f64, &'static str> {
    todo!()
}

#[test]
fn test_safe_divide() {
    assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
    assert_eq!(safe_divide(-9.0, 3.0), Ok(-3.0));
    assert!(safe_divide(10.0, 0.0).is_err());
}
