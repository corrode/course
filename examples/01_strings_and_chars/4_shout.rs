/// Takes a borrowed `&str` and returns an owned, uppercased `String`.
///
/// Notice the signature: borrow on the way in, own on the way out. You'll
/// see that everywhere in real Rust code.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase>
fn shout(text: &str) -> String {
    todo!()
}

#[test]
fn test_shout() {
    assert_eq!(shout("hello"), "HELLO");
    assert_eq!(shout("Rust"), "RUST");
    assert_eq!(shout(""), "");
}
