//! Exercise: this file does not compile. Run `cargo test --example
//! 04_functions` and read the error. The fix is a single character.
//!
//! Once you've fixed it, the test below should pass.
//!
//! Bonus: before you make the change, try to predict what the
//! compiler will complain about, and why. The intro chapter has the
//! relevant rule.

fn double(n: i32) -> i32 {
    n * 2;
}

#[test]
fn test_double() {
    assert_eq!(double(0), 0);
    assert_eq!(double(3), 6);
    assert_eq!(double(-7), -14);
}
