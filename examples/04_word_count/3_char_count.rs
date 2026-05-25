/// Returns the number of non-whitespace characters in `text`.
/// Spaces, tabs and newlines are skipped; punctuation counts.
fn char_count(text: &str) -> usize {
    todo!()
}

#[test]
fn test_char_count_simple() {
    // "hi there" → h, i, t, h, e, r, e (the space is skipped).
    assert_eq!(char_count("hi there"), 7);
}

#[test]
fn test_char_count_empty() {
    assert_eq!(char_count(""), 0);
    assert_eq!(char_count("   \n\t"), 0);
}

#[test]
fn test_char_count_punctuation_counts() {
    // Only whitespace is excluded; punctuation is a real character.
    // "hi, world!" → h, i, ',', w, o, r, l, d, '!' = 9
    assert_eq!(char_count("hi, world!"), 9);
}
