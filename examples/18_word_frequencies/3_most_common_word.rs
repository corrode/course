use std::collections::HashMap;

/// Counts how many times each word appears in the text. Words are
/// whitespace-separated; counts use lowercase keys.
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

/// Finds the most common word in the text. Returns the word and its count, or
/// None if text is empty.
fn most_common_word(text: &str) -> Option<(String, usize)> {
    todo!()
}

#[test]
fn test_most_common_word() {
    let text = "apple banana apple cherry apple";
    let (word, count) = most_common_word(text).unwrap();
    assert_eq!(word, "apple");
    assert_eq!(count, 3);
    assert_eq!(
        most_common_word("CAFÉ\tcafé tea"),
        Some(("café".to_string(), 2))
    );
    assert_eq!(most_common_word(""), None);
    assert_eq!(most_common_word(" \t\n"), None);
}
