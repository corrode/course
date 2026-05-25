/// Returns the total number of words in `text`. Words are pieces
/// separated by whitespace, so `"hello world"` has two words.
fn word_count(text: &str) -> usize {
    todo!()
}

#[test]
fn test_word_count_simple() {
    assert_eq!(word_count("hello world"), 2);
}

#[test]
fn test_word_count_empty() {
    assert_eq!(word_count(""), 0);
    assert_eq!(word_count("   "), 0);
}

#[test]
fn test_word_count_collapses_whitespace() {
    // Multiple spaces, tabs, and newlines all count as a single
    // boundary, so this is four words, not seven.
    assert_eq!(word_count("one  two\tthree\nfour"), 4);
}
