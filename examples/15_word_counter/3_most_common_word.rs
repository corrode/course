use std::collections::HashMap;

/// Counts how many times each word appears in the text.
/// Words are separated by spaces and should be case-insensitive.
fn count_words(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Finds the most common word in the text.
/// Returns the word and its count, or None if text is empty.
///
/// Tip: this is the function where the borrow checker pushes back. To
/// return `(String, usize)` you need to own the key, but `iter()` on
/// a `HashMap` only hands out borrows. The trick is
/// [`into_iter`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.into_iter):
/// it consumes the map and yields `(K, V)` pairs by value, so combining
/// it with `max_by_key` gives you back an owned `(String, usize)`.
fn most_common_word(text: &str) -> Option<(String, usize)> {
    // Use count_words() then find the max by count
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_common_word() {
        let text = "apple banana apple cherry apple";
        let (word, count) = most_common_word(text).unwrap();
        assert_eq!(word, "apple");
        assert_eq!(count, 3);
    }
}
