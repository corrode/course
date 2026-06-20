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

/// Filters words by minimum length.
/// Returns only words that appear at least `min_count` times.
fn frequent_words(text: &str, min_count: usize) -> Vec<String> {
    count_words(text)
        .into_iter()
        .filter(|(_, count)| *count >= min_count)
        .map(|(word, _)| word)
        .collect()
}

#[test]
fn test_frequent_words() {
    let text = "one two two three three three";
    let frequent = frequent_words(text, 2);
    assert!(frequent.contains(&"two".to_string()));
    assert!(frequent.contains(&"three".to_string()));
    assert!(!frequent.contains(&"one".to_string()));
}
