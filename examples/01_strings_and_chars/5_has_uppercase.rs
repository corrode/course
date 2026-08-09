/// Returns true if `text` contains at least one ASCII uppercase letter.
///
/// `for c in text.chars()` works fine and lets you inspect each character.
///
/// More experienced Rust developers often use iterator methods like `any`,
/// `find`, ... to express this kind of "is there at least one ..." check more
/// directly.
///
/// See: <https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_uppercase>
fn has_uppercase(text: &str) -> bool {
    todo!()
}

#[test]
fn test_has_uppercase() {
    assert!(has_uppercase("Hello"));
    assert!(has_uppercase("rustY"));
    assert!(!has_uppercase("hello"));
    assert!(!has_uppercase(""));
}
