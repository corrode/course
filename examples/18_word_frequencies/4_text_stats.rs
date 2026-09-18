use std::collections::HashMap;

/// Counts how many times each word appears in the text.
/// Words are whitespace-separated; counts use lowercase keys.
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

/// Calculates basic text statistics.
/// Returns (`total_words`, `unique_words`, `average_word_length`).
/// Lengths count Unicode scalar values in lowercase words, including every occurrence.
/// Empty or whitespace-only input returns (0, 0, 0.0).
///
/// In application code, I'd use a `struct TextStats { total: usize, unique: usize, avg_len: f64 }` so callers don't have to remember what each tuple position means.
/// Here we'll keep the tuple and focus on the iterator chain in the body.
fn text_stats(text: &str) -> (usize, usize, f64) {
    todo!()
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

#[test]
fn test_text_stats_repeated_words() {
    let (total, unique, avg_len) = text_stats("a A bbbb");
    assert_eq!(total, 3);
    assert_eq!(unique, 2);
    assert!((avg_len - 2.0).abs() < 1e-10);
}

#[test]
fn test_text_stats_empty() {
    assert_eq!(text_stats(""), (0, 0, 0.0));
    assert_eq!(text_stats(" \t\n"), (0, 0, 0.0));
}

#[test]
fn test_text_stats_unicode() {
    let (total, unique, avg_len) = text_stats("CAFÉ\tCafé\u{2003}🦀");
    assert_eq!(total, 3);
    assert_eq!(unique, 2);
    assert!((avg_len - 3.0).abs() < 1e-10);

    // Lowercase İ is i followed by a combining dot: two scalar values.
    let (total, unique, avg_len) = text_stats("İ i\u{307}");
    assert_eq!(total, 2);
    assert_eq!(unique, 1);
    assert!((avg_len - 2.0).abs() < 1e-10);
}
