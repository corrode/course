/// Returns the length (in characters) of the longest word in
/// `text`. Words are whitespace-separated. Returns 0 when the
/// text has no words.
fn longest_word(text: &str) -> usize {
    // 1. let mut max = 0;
    // 2. for word in text.split_whitespace() {
    //        let len = word.chars().count();
    //        if len > max { max = len; }
    //    }
    // 3. Return max.
    todo!()
}

#[test]
fn test_longest_word_simple() {
    // "a bb ccc dddd" → 4
    assert_eq!(longest_word("a bb ccc dddd"), 4);
}

#[test]
fn test_longest_word_empty() {
    assert_eq!(longest_word(""), 0);
    assert_eq!(longest_word("   "), 0);
}

#[test]
fn test_longest_word_unicode() {
    // "café" is 4 characters even though it's 5 bytes in UTF-8.
    // `word.len()` would say 5; `word.chars().count()` says 4.
    assert_eq!(longest_word("hi café"), 4);
}
