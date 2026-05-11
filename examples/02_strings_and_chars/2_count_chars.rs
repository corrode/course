//! # Counting characters
//!
//! Your first encounter with `&str`. In many languages, asking for the
//! "length" of a string gives you back the number of characters. In
//! Rust, `str::len` returns the number of *bytes* in the underlying
//! UTF-8 buffer, which only matches the character count for plain ASCII.
//!
//! For "hello" the byte count and char count both happen to be 5, but
//! "café" is 5 bytes and 4 chars. Reach for `chars()` when you want the
//! character count.

/// Counts how many characters are in `text`.
///
/// Watch out: `text.len()` returns the number of bytes, not characters.
/// For "hello" those happen to be the same, but for "café" they aren't.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.chars>
fn count_chars(text: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_chars() {
        assert_eq!(count_chars("hello"), 5);
        assert_eq!(count_chars("rust"), 4);
        assert_eq!(count_chars(""), 0);
    }
}
