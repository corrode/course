use std::collections::HashMap;

/// Counts how many times each word appears in the text.
/// Words are whitespace-separated; counts use lowercase keys.
fn count_words(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Finds the most common word in the text.
/// Returns the word and its count, or None if text is empty.
///
/// To return `(String, usize)`, you need to own the key, but `iter()` on a `HashMap` only lends you references.
/// Use [`into_iter`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.into_iter) to consume the map and yield `(K, V)` pairs by value.
/// Combine it with `max_by_key` to get an owned `(String, usize)`.
fn most_common_word(text: &str) -> Option<(String, usize)> {
    // Use count_words() then find the max by count
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
