//! # Strings, &str, and chars
//!
//! Now that you've handled numbers, let's meet Rust's text types. In many
//! languages "a string is a string", but Rust splits the job across a few
//! cooperating types — and that split is exactly what makes Rust strings
//! fast *and* safe.
//!
//! The cast you'll meet today:
//!
//! - `char`     — a single Unicode scalar value (always 4 bytes).
//! - `&str`     — a borrowed view into some UTF-8 text. Cheap to pass around.
//! - `String`   — an owned, growable UTF-8 buffer. You own the memory.
//! - `Vec<char>`— sometimes useful when you really need indexed character access.
//!
//! There's a recurring pattern here that you'll see again and again in
//! later exercises:
//!
//! | Borrowed (a "view") | Owned (you own the data) |
//! |---------------------|--------------------------|
//! | `&str`              | `String`                 |
//! | `&[T]`              | `Vec<T>`                 |
//! | `&Path`             | `PathBuf`                |
//!
//! Rule of thumb: take the borrowed form as input, return the owned form
//! when the caller needs to keep the result.
//!
//! These exercises are deliberately tiny. Solve them with the standard
//! library only — no external crates needed.

/// Returns the first character of `text`, or `None` if the string is empty.
///
/// Hint: `str::chars` returns an iterator. Iterators have a `.next()` method.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.chars>
fn first_char(text: &str) -> Option<char> {
    todo!()
}

/// Counts how many *characters* (not bytes!) are in `text`.
///
/// Why not `text.len()`? `len()` returns the number of *bytes*, and a
/// single emoji or accented character can take several bytes in UTF-8.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.chars>
fn count_chars(text: &str) -> usize {
    todo!()
}

/// Returns true if `text` contains at least one ASCII uppercase letter.
///
/// Hint: combine `.chars()` with `.any(...)` and `char::is_ascii_uppercase`.
/// See: <https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_uppercase>
fn has_uppercase(text: &str) -> bool {
    todo!()
}

/// Builds a `String` from a slice of `char`s.
///
/// This is the most common way to "assemble" a string from individual
/// characters in Rust: iterate, then `.collect::<String>()`.
/// See: <https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect>
fn chars_to_string(chars: &[char]) -> String {
    todo!()
}

/// Converts a `&str` into a `Vec<char>`.
///
/// Useful when you need indexed access to characters (e.g. `chars[3]`),
/// which `&str` does not allow because of UTF-8.
fn to_char_vec(text: &str) -> Vec<char> {
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
        assert_eq!(first_char(""), None);
        // chars() is Unicode-aware, not byte-based:
        assert_eq!(first_char("🦀 rust"), Some('🦀'));
    }

    #[test]
    fn test_count_chars() {
        assert_eq!(count_chars("hello"), 5);
        assert_eq!(count_chars(""), 0);
        // "🦀" is one character but takes 4 bytes in UTF-8.
        // This test would fail if you used .len() instead of .chars().count().
        assert_eq!(count_chars("🦀"), 1);
    }

    #[test]
    fn test_has_uppercase() {
        assert!(has_uppercase("Hello"));
        assert!(has_uppercase("rustY"));
        assert!(!has_uppercase("hello"));
        assert!(!has_uppercase(""));
    }

    #[test]
    fn test_chars_to_string() {
        let chars = ['r', 'u', 's', 't'];
        assert_eq!(chars_to_string(&chars), "rust");
        assert_eq!(chars_to_string(&[]), "");
    }

    #[test]
    fn test_to_char_vec() {
        assert_eq!(to_char_vec("abc"), vec!['a', 'b', 'c']);
        assert_eq!(to_char_vec(""), Vec::<char>::new());
    }

    #[test]
    fn test_shout() {
        assert_eq!(shout("hello"), "HELLO");
        assert_eq!(shout("Rust"), "RUST");
        assert_eq!(shout(""), "");
    }
}
