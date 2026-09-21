use std::collections::HashMap;

/// Counts how many times each word appears. Returns a `HashMap` mapping words
/// to their counts.
///
/// Use the `entry` API to work with new and repeated words through the same
/// lookup. Its mutable reference lets you update the count inside the map.
fn count_words(words: &[&str]) -> HashMap<String, usize> {
    todo!("Count each word using the HashMap entry API")
}

#[test]
fn test_count_words() {
    let words = ["hello", "world", "hello", "rust"];
    let counts = count_words(&words);
    assert_eq!(counts.get("hello"), Some(&2));
    assert_eq!(counts.get("world"), Some(&1));
    assert_eq!(counts.get("rust"), Some(&1));
}
