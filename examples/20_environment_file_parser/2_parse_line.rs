#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidFormat,
    EmptyKey,
    EmptyValue,
}

/// Parses a single line of an .env file.
/// Format: `KEY=value`. Surrounding whitespace on either side of `=` is
/// trimmed (so `KEY = value` is accepted and yields `("KEY", "value")`).
/// Returns `Ok((key, value))` or `Err(ParseError)`.
fn parse_env_line(line: &str) -> Result<(String, String), ParseError> {
    // 1. Check if line contains '='
    // 2. Split on '=' (use .split_once())
    // 3. Trim whitespace from both halves
    // 4. Validate that key and value are not empty after trimming
    todo!()
}

#[test]
fn test_parse_env_line() {
    assert_eq!(
        parse_env_line("PORT=8080"),
        Ok(("PORT".to_string(), "8080".to_string()))
    );
    assert_eq!(
        parse_env_line("HOST=localhost"),
        Ok(("HOST".to_string(), "localhost".to_string()))
    );
    assert!(parse_env_line("INVALID").is_err());
    assert!(parse_env_line("=value").is_err());
    assert!(parse_env_line("KEY=").is_err());
    // Surrounding whitespace is trimmed, not rejected.
    assert_eq!(
        parse_env_line("KEY = value"),
        Ok(("KEY".to_string(), "value".to_string()))
    );
}
