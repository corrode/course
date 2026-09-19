/// Sums two boxed integers, taking ownership of both boxes.
///
/// The boxes are dropped when the function returns, freeing their allocations.
fn boxed_sum(a: Box<i32>, b: Box<i32>) -> i32 {
    *a + *b
}

#[test]
fn test_boxed_sum() {
    assert_eq!(boxed_sum(Box::new(2), Box::new(3)), 5);
    assert_eq!(boxed_sum(Box::new(-10), Box::new(40)), 30);
    assert_eq!(boxed_sum(Box::new(0), Box::new(0)), 0);
}
