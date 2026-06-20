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

/// Calculates basic text statistics.
/// Returns (`total_words`, `unique_words`, `average_word_length`).
///
/// In real code you'd reach for a `struct TextStats { total: usize,
/// unique: usize, avg_len: f64 }` here; a 3-tuple is hard to read at
/// the call site. We're sticking with a tuple to keep the focus on the
/// iterator chain in the body.
fn text_stats(text: &str) -> (usize, usize, f64) {
    let counts = count_words(text);
    let total: usize = counts.values().sum();
    let unique = counts.len();
    let total_length: usize = counts
        .iter()
        .map(|(word, count)| word.chars().count() * count)
        .sum();
    let average_word_length = total_length as f64 / total as f64;
    (total, unique, average_word_length)
}

#[test]
fn test_text_stats() {
    let text = "hello world rust";
    let (total, unique, avg_len) = text_stats(text);
    assert_eq!(total, 3);
    assert_eq!(unique, 3);
    assert!((avg_len - 4.66).abs() < 0.01); // Average length ≈ 4.66
    // Side note: floats don't compare exactly (the value here is
    // really 14/3 = 4.666...), so we check that we're close enough
    // by taking the absolute difference and comparing to a tolerance.
    // Direct `==` on `f64` is almost always the wrong thing.
}
