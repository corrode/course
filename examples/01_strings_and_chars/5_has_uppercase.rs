/// Returns true if `text` contains at least one ASCII uppercase letter.
///
/// `for c in text.chars()` works fine and lets you inspect each character.
///
/// You can also use iterator methods like `any` or `find` to express this kind of "is there at least one ..." check directly.
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
    assert!(!has_uppercase("École"));
    assert!(has_uppercase("éZ"));
    assert!(!has_uppercase(""));
}
