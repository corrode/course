/// Returns the smaller of two numbers. Ties return the first.
///
/// The body fits in a single `if`/`else` expression. Don't use
/// `std::cmp::min` here; the point is to practice the syntax.
fn min(a: i32, b: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min() {
        assert_eq!(min(2, 5), 2);
        assert_eq!(min(5, 2), 2);
        assert_eq!(min(-3, -7), -7);
        assert_eq!(min(4, 4), 4);
    }
}
