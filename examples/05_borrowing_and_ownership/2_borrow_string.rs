/// Borrows a string reference without taking ownership. The original string
/// remains valid after this function returns.
fn borrow_string(s: &str) -> usize {
    todo!("Return the length of the borrowed string")
}

#[test]
fn test_borrow_string() {
    let mut s = String::from("The Matrix has you");
    let len = borrow_string(&s);
    // s is still valid here because we only borrowed it
    assert_eq!(len, 18);
    assert_eq!(s, "The Matrix has you");
    s.push_str(", Neo");
    assert_eq!(s, "The Matrix has you, Neo");
    assert_eq!(borrow_string(&s), 23);
}

#[test]
fn test_borrow_string_literal() {
    assert_eq!(borrow_string("Ferris"), 6);
}
