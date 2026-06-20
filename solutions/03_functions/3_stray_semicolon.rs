/// Doubles `n`.
///
/// The exercise version is missing its return value: `n * 2;` with a
/// trailing semicolon is a statement, so the function returns `()`
/// instead of `i32`. Dropping the semicolon makes `n * 2` the final
/// expression, which is what gets returned.
fn double(n: i32) -> i32 {
    n * 2
}

#[test]
fn test_double() {
    assert_eq!(double(0), 0);
    assert_eq!(double(3), 6);
    assert_eq!(double(-7), -14);
}
