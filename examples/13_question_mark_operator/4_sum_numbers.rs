/// Sums a whitespace-separated list of integers held in `text`.
///
/// Each token is parsed with `?`: the first one that isn't a number
/// short-circuits and returns its `ParseIntError`. The function only
/// parses (no file I/O), so a single error type is enough and there's
/// no need for `Box<dyn Error>`.
fn sum_numbers(text: &str) -> Result<i32, std::num::ParseIntError> {
    todo!()
}

#[test]
fn test_sum_numbers() {
    assert_eq!(sum_numbers("5\n10\n15").unwrap(), 30);
    assert_eq!(sum_numbers("  1  2  3 ").unwrap(), 6);
    assert!(sum_numbers("5\nabc\n15").is_err()); // not a number
}
