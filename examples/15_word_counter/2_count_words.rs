//! # Counting words
//!
//! The foundation for everything else in this chapter: take a string of
//! text and produce a `HashMap<String, usize>` that maps each word to
//! how many times it appears. Words are separated by whitespace and the
//! count should be case-insensitive — `"Hello"` and `"hello"` are the
//! same word.
//!
//! The classic recipe is: split on whitespace, lowercase each piece,
//! then walk the resulting iterator and bump a counter in the map. The
//! `entry` API on `HashMap` is the idiomatic way to do that last step:
//! `*map.entry(key).or_insert(0) += 1`.

use std::collections::HashMap;

/// Counts how many times each word appears in the text.
/// Words are separated by spaces and should be case-insensitive.
fn count_words(text: &str) -> HashMap<String, usize> {
    // 1. Split text by whitespace
    // 2. Convert each word to lowercase
    // 3. Count occurrences in a HashMap
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
