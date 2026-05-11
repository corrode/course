//! # Borrowing without taking
//!
//! Most of the time a function only needs to *look at* a value, not own
//! it. That's a shared borrow, written `&T` in the signature. The
//! caller keeps ownership; the callee gets temporary read-only access.
//!
//! This function takes `&str` rather than `&String`. `&str` is the
//! universal "borrowed string slice" type: a string literal is already
//! a `&'static str`, and `&String` automatically coerces to `&str`, so
//! `&str` parameters accept both without forcing the caller to convert.
//! Reach for `&str` by default when you're just reading.
//!
//! The body is a one-liner — call `.len()` on the slice. The point of
//! the exercise is the signature: notice that after the call, the
//! caller's `s` is still usable in the test below.

/// Borrows a string reference without taking ownership.
/// The original string remains valid after this function returns.
fn borrow_string(s: &str) -> usize {
    // Return the length of the string
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_string() {
        let s = "The Matrix has you";
        let len = borrow_string(s);
        // s is still valid here because we only borrowed it
        assert_eq!(len, 18);
        assert_eq!(s, "The Matrix has you"); // Still here!
    }
}
