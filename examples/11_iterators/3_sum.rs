//! # Summing with an iterator
//!
//! Iterators were popularised by functional languages like Lisp (created by
//! John McCarthy in 1958), and today they're a core building block in most
//! modern languages. Rust's iterators are lazy: they don't do any work
//! until you ask for a result.
//!
//! The simplest pattern is to take a sequence and collapse it down to a
//! single value. You could write a `for` loop with a running total, but
//! the standard library can do this for you in one call.

/// Calculates total revenue from sales data.
///
/// The simplest iterator pattern: take a sequence, produce one number.
/// You could write a `for` loop with a running total, but the standard
/// library can collapse a numeric iterator down for you in one call.
/// See: <https://doc.rust-lang.org/std/iter/trait.Iterator.html>
fn calculate_total_revenue() -> i32 {
    let sales = vec![1200, 850, 2300, 950, 1800, 3200, 1100, 2800];
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_revenue() {
        let total = calculate_total_revenue();
        assert_eq!(total, 14200); // Sum of all sales
    }
}
