use std::collections::HashMap;

/// Counts how many times each word appears in the text.
/// Words are separated by spaces and should be case-insensitive.
fn count_words(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Filters words by minimum length.
/// Returns only words that appear at least min_count times.
fn frequent_words(text: &str, min_count: usize) -> Vec<String> {
    // Use count_words() then filter and collect
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequent_words() {
        let text = "one two two three three three";
        let frequent = frequent_words(text, 2);
        assert!(frequent.contains(&"two".to_string()));
        assert!(frequent.contains(&"three".to_string()));
        assert!(!frequent.contains(&"one".to_string()));
    }
}
