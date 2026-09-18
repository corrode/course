/// Sums whitespace-separated integers with an iterator pipeline and fallible `sum`.
fn sum_numbers(text: &str) -> Result<i32, std::num::ParseIntError> {
    text.split_whitespace()
        .map(|token| token.parse::<i32>())
        .sum()
}

#[test]
fn test_sum_numbers() {
    assert_eq!(sum_numbers("5\n10\n15").unwrap(), 30);
    assert_eq!(sum_numbers("  1  2  3 ").unwrap(), 6);
    assert!(sum_numbers("5\nabc\n15").is_err()); // not a number
}

#[test]
fn test_sum_numbers_empty_and_signed() {
    assert_eq!(sum_numbers(""), Ok(0));
    assert_eq!(sum_numbers(" \t\n"), Ok(0));
    assert_eq!(sum_numbers("-5 +2 3"), Ok(0));
}

#[test]
fn test_sum_numbers_first_error() {
    let first_error = "abc".parse::<i32>().unwrap_err();
    assert_eq!(sum_numbers("5 abc 99999999999999999999"), Err(first_error));
}
