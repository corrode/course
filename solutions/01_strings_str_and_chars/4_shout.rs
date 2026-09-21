/// Takes a borrowed `&str` and returns an owned, uppercased `String`.
///
/// Notice the signature: borrow on the way in, own on the way out. That's the
/// same pattern as the text replacement example in the chapter intro. See:
/// <https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase>
fn shout(text: &str) -> String {
    text.to_uppercase()
}

#[test]
fn test_shout() {
    assert_eq!(shout("hello"), "HELLO");
    assert_eq!(shout("Rust"), "RUST");
    assert_eq!(shout(""), "");
}
