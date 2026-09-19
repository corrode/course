//! Exercise: write a function `cap_at` that takes a `value` and a `max`, both
//! `i32`, and returns `value` if it's at or below `max`, or `max` otherwise.
//!
//! Any correct implementation is welcome. After it passes, try the
//! parameter-reassignment experiment in the paired instructions.

#[test]
fn test_cap_at() {
    assert_eq!(cap_at(5, 10), 5);
    assert_eq!(cap_at(10, 10), 10);
    assert_eq!(cap_at(11, 10), 10);
    assert_eq!(cap_at(-3, 10), -3);
    assert_eq!(cap_at(1_000, 0), 0);
}

// Even if the function reassigns its parameter, the caller keeps its original
// value: `i32` is `Copy`.
#[test]
fn caller_value_is_unchanged() {
    let original = 42;
    let _capped = cap_at(original, 10);
    assert_eq!(original, 42);
}
