//! # A first peek at `Option`
//!
//! What should `first_char("")` return? There is no first character,
//! so returning some "default" `char` would be a lie. Rust's answer is
//! `Option<T>`: a value that is either `Some(x)` or `None`. Chapter 7
//! covers it properly; for now, just know that `text.chars()` is an
//! iterator, and iterators already know how to hand you their first
//! element wrapped in an `Option`.

/// Returns the first character of `text`, or `None` if the string is empty.
///
/// This is your first peek at `Option<T>`: Rust's way of saying "there
/// might not be a value here." Chapter 7 covers it properly; for now,
/// `text.chars()` is already an iterator, and iterators know how to
/// hand you their first element.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.chars>
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
