use std::collections::HashMap;

/// Counts how many times each word appears in the text.
/// Words are separated by spaces and should be case-insensitive.
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

#[test]
fn test_count_words() {
    let text = "hello world hello rust world";
    let counts = count_words(text);
    assert_eq!(counts.get("hello"), Some(&2));
    assert_eq!(counts.get("world"), Some(&2));
    assert_eq!(counts.get("rust"), Some(&1));
}

#[test]
fn test_count_words_case_insensitive() {
    let text = "Hello HELLO hello";
    let counts = count_words(text);
    assert_eq!(counts.get("hello"), Some(&3));
}
