/// Returns true if `text` contains at least one ASCII uppercase letter.
///
/// `for c in text.chars()` lets you inspect each character; the iterator
/// methods (`any`, `find`, ...) usually express this kind of "is there
/// at least one ..." check more directly.
/// See: <https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_uppercase>
fn has_uppercase(text: &str) -> bool {
    text.chars().any(|c| c.is_ascii_uppercase())
}

#[test]
fn test_has_uppercase() {
    assert!(has_uppercase("Hello"));
    assert!(has_uppercase("rustY"));
    assert!(!has_uppercase("hello"));
    assert!(!has_uppercase(""));
}
