/// Parses a percentage from a string.
/// Accepts integers in `0..=100`, optionally with a trailing `%` (so `"42"` and `"42%"` both work).
///
/// Returns `Ok(value)` on success.
/// If the text cannot be parsed as u8, returns `Err("not a valid percentage")`.
/// A parsed value above 100 returns `Err("percentage must be between 0 and 100")`.
/// Only one trailing `%` is allowed.
fn parse_percentage(input: &str) -> Result<u8, &'static str> {
    todo!()
}

#[test]
fn test_parse_percentage() {
    assert_eq!(parse_percentage("0"), Ok(0));
    assert_eq!(parse_percentage("42"), Ok(42));
    assert_eq!(parse_percentage("100"), Ok(100));
    assert_eq!(parse_percentage("75%"), Ok(75));
    assert_eq!(
        parse_percentage("101"),
        Err("percentage must be between 0 and 100")
    );
    assert_eq!(
        parse_percentage("255%"),
        Err("percentage must be between 0 and 100")
    );
    // These cannot be parsed as u8, unlike 101 and 255 above.
    assert_eq!(parse_percentage("256"), Err("not a valid percentage"));
    assert_eq!(parse_percentage("-1"), Err("not a valid percentage"));
    assert_eq!(parse_percentage("half"), Err("not a valid percentage"));
    assert_eq!(parse_percentage(""), Err("not a valid percentage"));
    assert_eq!(parse_percentage("75%%"), Err("not a valid percentage"));
}
