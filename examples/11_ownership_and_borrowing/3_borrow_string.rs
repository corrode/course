/// Borrows a string reference without taking ownership.
/// The original string remains valid after this function returns.
fn borrow_string(s: &str) -> usize {
    // Return the length of the string
    todo!()
}

#[test]
fn test_borrow_string() {
    let s = "The Matrix has you";
    let len = borrow_string(s);
    // s is still valid here because we only borrowed it
    assert_eq!(len, 18);
    assert_eq!(s, "The Matrix has you"); // Still here!
}
