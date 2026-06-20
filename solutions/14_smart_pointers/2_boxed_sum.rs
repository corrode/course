/// Sums two heap-allocated integers and returns the result.
///
/// Both inputs are `Box<i32>`: integers that live on the heap
/// instead of the stack. You read the inner value with the deref
/// operator (`*a`), then add normally.
///
/// The boxes are dropped at the end of the function, freeing the
/// two heap allocations automatically.
fn boxed_sum(a: Box<i32>, b: Box<i32>) -> i32 {
    *a + *b
}

#[test]
fn test_boxed_sum() {
    assert_eq!(boxed_sum(Box::new(2), Box::new(3)), 5);
    assert_eq!(boxed_sum(Box::new(-10), Box::new(40)), 30);
    assert_eq!(boxed_sum(Box::new(0), Box::new(0)), 0);
}

#[test]
fn test_box_holds_a_value() {
    let boxed = Box::new(42);
    // Auto-deref lets method calls go through the box.
    assert_eq!(*boxed, 42);
}
