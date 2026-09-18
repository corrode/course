/// Parses a percentage from a string.
/// Accepts integers in `0..=100`, optionally with a trailing `%` (so `"42"` and `"42%"` both work).
///
/// Returns `Ok(value)` on success, `Err(message)` otherwise.
///
/// This takes more thought than the previous exercises because parsing successfully does not mean the number is in range.
/// More than one thing can go wrong, and they need different error messages.
/// Strip the optional `%` first, then `parse::<u8>()` the rest, then bounds-check.
/// Each step is its own potential `Err`.
///
/// The error type here is `&'static str`, so use string literals for the messages.
/// If you find yourself wanting `format!("{input} is out of range")` in an `Err`, you'd need to change the return type to `Result<u8, String>`.
/// Stick with literals for this exercise.
fn parse_percentage(input: &str) -> Result<u8, &'static str> {
    todo!()
}

#[test]
fn test_parse_percentage() {
    assert_eq!(parse_percentage("0"), Ok(0));
    assert_eq!(parse_percentage("42"), Ok(42));
    assert_eq!(parse_percentage("100"), Ok(100));
    assert_eq!(parse_percentage("75%"), Ok(75));
    assert!(parse_percentage("101").is_err());
    assert!(parse_percentage("-1").is_err());
    assert!(parse_percentage("half").is_err());
    assert!(parse_percentage("").is_err());
}
