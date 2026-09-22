/// Returns true if `text` contains at least one Unicode uppercase character.
///
/// `for c in text.chars()` lets you inspect each character; the iterator
/// methods (`any`, `find`, ...) usually express this kind of "is there at least
/// one ..." check more directly. See:
/// <https://doc.rust-lang.org/std/primitive.char.html#method.is_uppercase>
fn has_uppercase(text: &str) -> bool {
    text.chars().any(char::is_uppercase)
}

#[test]
fn test_has_uppercase() {
    assert!(has_uppercase("Hello"));
    assert!(has_uppercase("rustY"));
    assert!(!has_uppercase("hello"));
    assert!(has_uppercase("École"));
    assert!(!has_uppercase("école"));
    assert!(has_uppercase("éZ"));
    assert!(!has_uppercase(""));
}

#[test]
fn test_has_uppercase_outside_ascii() {
    assert!(has_uppercase("éΩ"));
    assert!(!has_uppercase("éω"));
}
