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

#[test]
fn test_calculate_total_revenue() {
    let total = calculate_total_revenue();
    assert_eq!(total, 14200); // Sum of all sales
}
