//! # Transforming the inside
//!
//! Same idea as before, but now the fallback isn't the value itself. You
//! need to call `.len()` on the inner string first. A `match` makes both
//! branches explicit; iterator-style methods on `Option` are tidier once
//! you spot them.
//!
//! See: <https://doc.rust-lang.org/std/keyword.match.html>

/// Returns the length if `Some`, 0 if `None`.
fn optional_string_length(maybe_string: Option<&str>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optional_string_length() {
        assert_eq!(optional_string_length(Some("hello")), 5);
        assert_eq!(optional_string_length(None), 0);
    }
}
