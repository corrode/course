//! # Iterating over characters
//!
//! Strings aren't directly indexable in Rust (because UTF-8 characters
//! have varying widths), but you can iterate over their `char`s. A
//! plain `for c in text.chars()` loop will work, and so will the
//! iterator combinators like `any` or `find` — which usually express
//! "is there at least one ..." checks more directly.

/// Returns true if `text` contains at least one ASCII uppercase letter.
///
/// `for c in text.chars()` lets you inspect each character; the iterator
/// methods (`any`, `find`, ...) usually express this kind of "is there
/// at least one ..." check more directly.
/// See: <https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_uppercase>
fn has_uppercase(text: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_uppercase() {
        assert!(has_uppercase("Hello"));
        assert!(has_uppercase("rustY"));
        assert!(!has_uppercase("hello"));
        assert!(!has_uppercase(""));
    }
}
