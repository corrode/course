/// Doubles a number.
///
/// `i32` is a `Copy` type, so the caller's value is duplicated bit-for-bit when
/// it's passed in. The original stays usable after the call (see the test).
/// Passing a `String` by value would move it instead.
fn double(n: i32) -> i32 {
    n * 2
}

#[test]
fn test_double() {
    let x = 21;
    let y = double(x);
    assert_eq!(y, 42);
    // x is still usable here: i32 is Copy, so `double(x)` copied it instead of
    // moving it.
    assert_eq!(x, 21);
}
