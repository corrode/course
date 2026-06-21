//! Exercise: write a function `cap_at` that takes a `value` and a
//! `max`, both `i32`, and returns `value` if it's at or below
//! `max`, or `max` otherwise.
//!
//! The logic itself is one `if` away. The signature, however, has a
//! catch. Write it the most natural way you can think of, run the
//! tests, and read the compiler error carefully before changing
//! anything. The `.md` has the one hint you'll need.

#[test]
fn test_cap_at() {
    assert_eq!(cap_at(5, 10), 5);
    assert_eq!(cap_at(10, 10), 10);
    assert_eq!(cap_at(11, 10), 10);
    assert_eq!(cap_at(-3, 10), -3);
    assert_eq!(cap_at(1_000, 0), 0);
}

// The caller's variable is not affected by the function modifying
// its parameter. `i32` is `Copy`, so the function got its own copy.
#[test]
fn caller_value_is_unchanged() {
    let original = 42;
    let _capped = cap_at(original, 10);
    assert_eq!(original, 42);
}
