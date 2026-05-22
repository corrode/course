/// Counts how many characters are in `text`.
///
/// Watch out: `text.len()` returns the number of bytes, not characters.
/// For "hello" those happen to be the same, but for "café" they aren't.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.chars>
fn count_chars(text: &str) -> usize {
    todo!()
}

#[test]
fn test_count_chars() {
    assert_eq!(count_chars("hello"), 5);
    assert_eq!(count_chars("rust"), 4);
    assert_eq!(count_chars(""), 0);
}
