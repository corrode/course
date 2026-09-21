/// Divides `dividend` by `divisor`.
///
/// Returns `Ok(quotient)` when the division is well-defined, or
/// `Err("cannot divide by zero")` when `divisor` is `0.0`.
///
/// You can produce a `Result` with an `if` that checks the failure case and an
/// `else` branch that returns `Ok(...)`.
///
/// The error type is `&'static str`, so you can return a fixed error message
/// without defining a new type.
fn safe_divide(dividend: f64, divisor: f64) -> Result<f64, &'static str> {
    todo!("Divide the numbers, returning an error for a zero divisor")
}

#[test]
fn test_safe_divide() {
    assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
    assert_eq!(safe_divide(-9.0, 3.0), Ok(-3.0));
    assert_eq!(safe_divide(10.0, 0.0), Err("cannot divide by zero"));
    assert_eq!(safe_divide(10.0, -0.0), Err("cannot divide by zero"));
}
