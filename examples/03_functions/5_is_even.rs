/// Returns `true` if `n` is even, `false` otherwise.
///
/// One expression. The `%` operator gives the remainder; even
/// numbers leave a remainder of `0` when divided by `2`.
fn is_even(n: u32) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(0));
        assert!(is_even(2));
        assert!(is_even(100));
        assert!(!is_even(1));
        assert!(!is_even(7));
    }
}
