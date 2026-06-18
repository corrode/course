/// Parses a string into a positive integer.
/// Returns the number if valid, 0 if invalid.
fn parse_positive_integer(input: &str) -> u32 {
    input.parse().unwrap_or(0)
}

#[test]
fn test_parse_positive_integer() {
    assert_eq!(parse_positive_integer("123"), 123);
    assert_eq!(parse_positive_integer("0"), 0);
    assert_eq!(parse_positive_integer("invalid"), 0);
    assert_eq!(parse_positive_integer("-5"), 0);
}
