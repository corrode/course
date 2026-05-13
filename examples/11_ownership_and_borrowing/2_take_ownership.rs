/// Takes ownership of a String and modifies it.
/// When you pass a String to this function, ownership transfers.
fn take_ownership(s: String) -> String {
    // Add " - owned by Rust!" to the end and return
    todo!()
}

#[test]
fn test_take_ownership() {
    let s = String::from("Rust");
    let result = take_ownership(s);
    // Note: s is no longer valid here! It was moved.
    assert_eq!(result, "Rust - owned by Rust!");
}
