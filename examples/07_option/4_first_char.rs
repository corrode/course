//! # Producing an `Option<char>`
//!
//! Now you have to *produce* an `Option`, not consume one. Asking a
//! string for its first character is the canonical example: if the
//! string is empty, there is no first character, and returning some
//! "default" `char` would be a lie. `Option<char>` is the honest type.
//!
//! You could pattern-match by hand, but the standard library has
//! already done the work for you: `text.chars()` returns an iterator,
//! and every iterator's `.next()` already hands you `Option<Item>`.
//! Compose the two.
//!
//! See: <https://doc.rust-lang.org/std/primitive.str.html#method.chars>

/// Returns the first character of `text`, or `None` if the string is empty.
fn first_char(text: &str) -> Option<char> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_char() {
        assert_eq!(first_char("hello"), Some('h'));
        assert_eq!(first_char("rust"), Some('r'));
        assert_eq!(first_char(""), None);
    }
}
