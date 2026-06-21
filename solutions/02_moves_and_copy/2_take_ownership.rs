/// Takes ownership of a String and modifies it.
/// When you pass a String to this function, ownership transfers.
fn take_ownership(s: String) -> String {
    let mut s = s;
    s.push_str(" - owned by Rust!");
    s
}

#[test]
fn test_take_ownership() {
    let s = String::from("Rust");
    let result = take_ownership(s);
    // Note: s is no longer valid here! It was moved.
    assert_eq!(result, "Rust - owned by Rust!");
}
