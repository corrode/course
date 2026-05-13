//! Exercise: write a function `countdown` that takes an unsigned
//! integer `n` and prints the numbers from `n` down to `1`, each on
//! its own line, then prints `"Liftoff!"`. If `n == 0`, it should
//! print only `"Liftoff!"`.
//!
//! Solve it recursively: the function should call itself with a
//! smaller argument. Don't use a loop. The signature is for you to
//! figure out; the tests below will tell you when you've got it.

// We can't easily assert on stdout here, so the tests just check
// that `countdown` compiles, runs, and returns `()` for a few
// inputs without panicking.
#[test]
fn test_countdown_returns_unit() {
    let result: () = countdown(0);
    assert_eq!(result, ());
    countdown(1);
    countdown(5);
}
