/// Returns the number of characters in `text`. Counts every `char`
/// the string contains, whitespace included.
fn char_count(text: &str) -> usize {
    // 1. Start a counter at 0.
    // 2. for _ in text.chars() { bump the counter }
    // 3. Return the counter.
    todo!()
}

#[test]
fn test_char_count_simple() {
    // "hi there" → h, i, ' ', t, h, e, r, e = 8
    assert_eq!(char_count("hi there"), 8);
}

#[test]
fn test_char_count_empty() {
    assert_eq!(char_count(""), 0);
}

#[test]
fn test_char_count_whitespace_counts() {
    // Whitespace characters are real characters too:
    // 3 spaces + '\n' + '\t' = 5.
    assert_eq!(char_count("   \n\t"), 5);
}

#[test]
fn test_char_count_unicode() {
    // `café` is 4 characters even though it's 5 bytes in UTF-8.
    // `text.len()` would say 5; `text.chars().count()` says 4.
    // `"hi café"` → h, i, ' ', c, a, f, é = 7.
    assert_eq!(char_count("café"), 4);
    assert_eq!(char_count("hi café"), 7);
}
