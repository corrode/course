/// Returns `value` if it is at or below `max`, otherwise `max`.
///
/// The catch is the keyword `mut`: to reassign the parameter inside the
/// function it has to be declared `mut value`. Because `i32` is `Copy`,
/// the function mutates its own copy, so the caller's variable is left
/// untouched (see `caller_value_is_unchanged`).
fn cap_at(mut value: i32, max: i32) -> i32 {
    if value > max {
        value = max;
    }
    value
}

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
