//! # `?` for the simple case
//!
//! `?` is Rust's shortcut for "if this is `Err`, return it from the
//! current function; otherwise, unwrap the value and continue." It
//! compresses a lot of `match` boilerplate into one character.
//!
//! Here both calls to `parse()` return the *same* error type
//! ([`ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html)),
//! so `?` works directly without any conversion. Compare to writing this
//! out with a `match` on each `parse()` result — that's the boilerplate
//! `?` is replacing.
//!
//! See: <https://doc.rust-lang.org/std/primitive.str.html#method.parse>

/// Adds two parsed numbers. Compare this to doing it with match statements.
fn add_parsed_numbers(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_parsed_numbers() {
        assert_eq!(add_parsed_numbers("10", "20"), Ok(30));
        assert!(add_parsed_numbers("abc", "10").is_err());
    }
}
