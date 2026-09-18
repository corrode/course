use std::collections::HashMap;

/// Counts how many times each word appears in the text.
/// Words are separated by spaces and should be case-insensitive.
fn count_words(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Calculates basic text statistics.
/// Returns (`total_words`, `unique_words`, `average_word_length`).
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
