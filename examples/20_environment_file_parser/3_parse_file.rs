use std::collections::HashMap;

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
    todo!()
}

/// Parses a complete .env file content.
/// Ignores empty lines and lines starting with `#` (after trimming, so
/// `   # comment` counts as a comment too). Stops at the first malformed
/// line and returns `Err`. Strict parsing is easier to debug than
/// silently dropping lines.
/// Returns `HashMap` of all valid key-value pairs.
fn parse_env_file(content: &str) -> Result<HashMap<String, String>, ParseError> {
    todo!()
}

#[test]
fn test_parse_env_file() {
    let content = r#"
# Database configuration
HOST=localhost
PORT=5432
DATABASE=myapp

# Empty line above should be ignored
DEBUG=true
"#;
    let env = parse_env_file(content).unwrap();
    assert_eq!(env.get("HOST"), Some(&"localhost".to_string()));
    assert_eq!(env.get("PORT"), Some(&"5432".to_string()));
    assert_eq!(env.get("DEBUG"), Some(&"true".to_string()));
    assert_eq!(env.len(), 4);
}
