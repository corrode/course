//! # Word Counter - Combining Concepts
//!
//! Congratulations! You've learned so many Rust concepts, and now it's time
//! to put them all together! Word counting might seem simple, but it's actually
//! the foundation of some amazing technologies.
//!
//! Google's PageRank algorithm (which made Google successful) started with
//! counting and analyzing word relationships. MapReduce, the technology that
//! powers big data processing, often uses word counting as its "Hello, World!"
//! example. And now it's your privilege to build the best word counter in the
//! world.
//!
//! You're building something that connects to the biggest ideas in computer science.
//! How cool is that? Let's count our way to greatness!

use std::collections::HashMap;

/// Counts how many times each word appears in the text.
/// Words are separated by spaces and should be case-insensitive.
fn count_words(text: &str) -> HashMap<String, usize> {
    // 1. Split text by whitespace
    // 2. Convert each word to lowercase
    // 3. Count occurrences in a HashMap
    todo!()
}

/// Finds the most common word in the text.
/// Returns the word and its count, or None if text is empty.
fn most_common_word(text: &str) -> Option<(String, usize)> {
    // Use count_words() then find the max by count
    todo!()
}

/// Filters words by minimum length.
/// Returns only words that appear at least min_count times.
fn frequent_words(text: &str, min_count: usize) -> Vec<String> {
    // Use count_words() then filter and collect
    todo!()
}

/// Calculates basic text statistics.
/// Returns (total_words, unique_words, average_word_length).
fn text_stats(text: &str) -> (usize, usize, f64) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_counting() {
        let text = "hello world hello rust world";
        let counts = count_words(text);
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&2));
        assert_eq!(counts.get("rust"), Some(&1));
    }

    #[test]
    fn test_case_insensitive() {
        let text = "Hello HELLO hello";
        let counts = count_words(text);
        assert_eq!(counts.get("hello"), Some(&3));
    }

    #[test]
    fn test_most_common() {
        let text = "apple banana apple cherry apple";
        let (word, count) = most_common_word(text).unwrap();
        assert_eq!(word, "apple");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_frequent_words() {
        let text = "one two two three three three";
        let frequent = frequent_words(text, 2);
        assert!(frequent.contains(&"two".to_string()));
        assert!(frequent.contains(&"three".to_string()));
        assert!(!frequent.contains(&"one".to_string()));
    }

    #[test]
    fn test_text_statistics() {
        let text = "hello world rust";
        let (total, unique, avg_len) = text_stats(text);
        assert_eq!(total, 3);
        assert_eq!(unique, 3);
        assert!((avg_len - 4.33).abs() < 0.1); // Average length ≈ 4.33
    }
}
