/// Counts how many numbers in the slice are even.
///
/// A `for` loop over the slice plus a `mut` counter is the natural
/// shape. Use `continue` to skip the odd numbers if you like; it's
/// not required, just easier to read.
fn count_evens(numbers: &[i32]) -> u32 {
    todo!()
}

#[test]
fn test_count_evens() {
    assert_eq!(count_evens(&[]), 0);
    assert_eq!(count_evens(&[1, 3, 5]), 0);
    assert_eq!(count_evens(&[2, 4, 6, 8]), 4);
    assert_eq!(count_evens(&[1, 2, 3, 4, 5, 6]), 3);
    assert_eq!(count_evens(&[0, -2, -3, 7]), 2);
}
