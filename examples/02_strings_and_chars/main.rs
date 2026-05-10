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

/// Counts how many characters are in `text`.
///
/// Watch out: `text.len()` returns the number of bytes, not characters.
/// For "hello" those happen to be the same, but for "café" they aren't.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.chars>
fn count_chars(text: &str) -> usize {
    todo!()
}

#[test]
fn test_count_chars() {
    assert_eq!(count_chars("hello"), 5);
    assert_eq!(count_chars("rust"), 4);
    assert_eq!(count_chars(""), 0);
}

/// Takes a borrowed `&str` and returns an owned, uppercased `String`.
///
/// Notice the signature: borrow on the way in, own on the way out. That's
/// the pattern the table in the chapter intro is hinting at, and you'll
/// see it everywhere in real Rust code.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase>
fn shout(text: &str) -> String {
    todo!()
}

#[test]
fn test_shout() {
    assert_eq!(shout("hello"), "HELLO");
    assert_eq!(shout("Rust"), "RUST");
    assert_eq!(shout(""), "");
}

/// Returns true if `text` contains at least one ASCII uppercase letter.
///
/// `for c in text.chars()` lets you inspect each character; the iterator
/// methods (`any`, `find`, ...) usually express this kind of "is there
/// at least one ..." check more directly.
/// See: <https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_uppercase>
fn has_uppercase(text: &str) -> bool {
    todo!()
}

#[test]
fn test_has_uppercase() {
    assert!(has_uppercase("Hello"));
    assert!(has_uppercase("rustY"));
    assert!(!has_uppercase("hello"));
    assert!(!has_uppercase(""));
}

/// Returns the first character of `text`, or `None` if the string is empty.
///
/// This is your first peek at `Option<T>` — Rust's way of saying "there
/// might not be a value here." Chapter 7 covers it properly; for now,
/// `text.chars()` is already an iterator, and iterators know how to
/// hand you their first element.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.chars>
fn first_char(text: &str) -> Option<char> {
    todo!()
}

#[test]
fn test_first_char() {
    assert_eq!(first_char("hello"), Some('h'));
    assert_eq!(first_char("rust"), Some('r'));
    assert_eq!(first_char(""), None);
}
