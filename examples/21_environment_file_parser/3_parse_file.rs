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

#[test]
fn malformed_entry_between_valid_entries_is_not_skipped() {
    for (bad, error) in [
        ("BROKEN", ParseError::InvalidFormat),
        (" =value", ParseError::EmptyKey),
        ("KEY=   ", ParseError::EmptyValue),
    ] {
        let content = format!("HOST=localhost\n{bad}\nPORT=8080");
        assert_eq!(parse_env_file(&content), Err(error));
    }
}

#[test]
fn first_error_wins() {
    assert_eq!(
        parse_env_file("HOST=localhost\nKEY=\nBROKEN\nPORT=8080"),
        Err(ParseError::EmptyValue)
    );
}

#[test]
fn indented_comments_and_blank_lines_are_not_errors() {
    let env = parse_env_file("  # comment\n  \nHOST=localhost\n").unwrap();
    assert_eq!(env.len(), 1);
    assert_eq!(env.get("HOST").map(String::as_str), Some("localhost"));
}
