//! # Strings, &str, and chars
//!
//! Now that you've handled numbers, let's meet Rust's text types. In many
//! languages "a string is a string", but Rust splits the job across a few
//! cooperating types, and that split is exactly what makes Rust strings
//! fast *and* safe.
//!
//! The cast you'll meet today:
//!
//! - `char`     : a single Unicode scalar value (always 4 bytes).
//! - `&str`     : a borrowed view into some UTF-8 text. Cheap to pass around.
//! - `String`   : an owned, growable UTF-8 buffer. You own the memory.
//!
//! There's a recurring pattern here that you'll see again and again in
//! later exercises:
//!
//! | Borrowed (a "view") | Owned (you own the data) |
//! |---------------------|--------------------------|
//! | `&str`              | `String`                 |
//!
//! Rule of thumb: take the borrowed form as input, return the owned form
//! when the caller needs to keep the result.
//!
//! These exercises are deliberately tiny. Solve them with the standard
//! library only. No external crates needed.

/// Returns the first character of `text`, or `None` if the string is empty.
///
/// Hint: `text.chars().next()` gives you the first `char` (or `None`).
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.chars>
fn first_char(text: &str) -> Option<char> {
    todo!()
}

/// Counts how many *characters* are in `text`.
///
/// Watch out: `text.len()` returns the number of *bytes*, which is not what
/// we want here. Use `text.chars().count()` instead.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.chars>
fn count_chars(text: &str) -> usize {
    todo!()
}

/// Returns true if `text` contains at least one ASCII uppercase letter.
///
/// Two equally fine approaches:
/// 1. A `for c in text.chars()` loop that returns `true` early.
/// 2. `text.chars().any(|c| c.is_ascii_uppercase())`.
///
/// See: <https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_uppercase>
fn has_uppercase(text: &str) -> bool {
    todo!()
}

/// Takes a borrowed `&str` and returns an owned, uppercased `String`.
///
/// Notice the signature: borrow on the way in, own on the way out. This
/// is the most common shape of a string-processing function in Rust.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase>
fn shout(text: &str) -> String {
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

    #[test]
    fn test_count_chars() {
        assert_eq!(count_chars("hello"), 5);
        assert_eq!(count_chars("rust"), 4);
        assert_eq!(count_chars(""), 0);
    }

    #[test]
    fn test_has_uppercase() {
        assert!(has_uppercase("Hello"));
        assert!(has_uppercase("rustY"));
        assert!(!has_uppercase("hello"));
        assert!(!has_uppercase(""));
    }

    #[test]
    fn test_shout() {
        assert_eq!(shout("hello"), "HELLO");
        assert_eq!(shout("Rust"), "RUST");
        assert_eq!(shout(""), "");
    }
}
