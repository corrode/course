//! # Number to string
//!
//! Start with the simplest direction: take a number, hand back its
//! textual form. Rust's standard library has a one-shot method for
//! this on every primitive integer type, and the `format!` macro from
//! chapter 0 works too. Either is fine.
//!
//! See: <https://doc.rust-lang.org/std/primitive.u32.html#method.to_string>

/// Convert a number to a string.
fn number_to_string(number: u32) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_string() {
        assert_eq!(number_to_string(1234), "1234");
        assert_eq!(number_to_string(0), "0");
    }
}
