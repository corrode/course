//! # Environment File Parser
//!
//! Configuration files are everywhere in software. The `.env` format became
//! popular with the twelve-factor app methodology, which recommends storing
//! configuration in environment variables rather than in code.
//!
//! Keeping secrets out of source control (no more passwords accidentally
//! committed to git) is a big practical win. Heroku popularised this pattern,
//! and it's now used by a lot of applications.
//!
//! The format looks simple, but it's surprisingly hard to parse correctly.
//! Trust me on that one.

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum ParseError {
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
fn test_parse_line() {
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

/// Parses a complete .env file content.
/// Ignores empty lines and lines starting with `#` (after trimming, so
/// `   # comment` counts as a comment too). Stops at the first malformed
/// line and returns `Err`. Strict parsing is easier to debug than
/// silently dropping lines.
/// Returns HashMap of all valid key-value pairs.
fn parse_env_file(content: &str) -> Result<HashMap<String, String>, ParseError> {
    todo!()
}

#[test]
fn test_parse_file() {
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

/// Gets an environment variable with type conversion.
/// Parses the string value into the requested type.
///
/// Hint: the natural shape is `env.get(key)?.parse().ok()`. Don't try to
/// `?` the parse: `T::Err` is unconstrained here, so `?` would need a
/// `From<T::Err>` bound that we haven't added. `.ok()` collapses
/// `Result<T, T::Err>` into `Option<T>`, which is what the signature
/// returns anyway.
fn get_env_var<T>(env: &HashMap<String, String>, key: &str) -> Option<T>
where
    T: std::str::FromStr,
{
    todo!()
}

#[test]
fn test_type_conversion() {
    let mut env = HashMap::new();
    env.insert("PORT".to_string(), "8080".to_string());
    env.insert("DEBUG".to_string(), "true".to_string());

    let port: Option<u16> = get_env_var(&env, "PORT");
    assert_eq!(port, Some(8080));

    let debug: Option<bool> = get_env_var(&env, "DEBUG");
    assert_eq!(debug, Some(true));
}

/// Validates required environment variables are present.
/// Returns Ok(()) if all required keys exist, Err with missing key otherwise.
fn validate_required_vars(env: &HashMap<String, String>, required: &[&str]) -> Result<(), String> {
    todo!()
}

#[test]
fn test_validation() {
    let mut env = HashMap::new();
    env.insert("HOST".to_string(), "localhost".to_string());
    env.insert("PORT".to_string(), "8080".to_string());

    assert!(validate_required_vars(&env, &["HOST", "PORT"]).is_ok());
    assert!(validate_required_vars(&env, &["HOST", "MISSING"]).is_err());
}
