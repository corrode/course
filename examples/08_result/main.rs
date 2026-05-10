//! # Result
//!
//! Phil Karlton famously said: "There are only two hard things in computer
//! science: cache invalidation and naming things." Error handling probably
//! deserves a spot on that list too.
//!
//! Many languages use exceptions, which can pop up anywhere and crash your
//! program unexpectedly. Rust uses `Result<T, E>` to make errors visible in
//! the type system: if a function can fail, its signature says so, and you
//! have to deal with it.
//!
//! This approach was inspired by functional languages like Haskell and ML.

/// Divides `dividend` by `divisor`.
///
/// Returns `Ok(quotient)` when the division is well-defined, or
/// `Err("cannot divide by zero")` when `divisor` is `0.0`.
///
/// Start here. The simplest way to produce a `Result`: an `if` checks
/// the failure case, the `else` branch returns `Ok(...)`.
///
/// The signature is the interesting part: `&'static str` for the error
/// is the simplest possible error type and is fine while you're learning.
fn safe_divide(dividend: f64, divisor: f64) -> Result<f64, &'static str> {
    todo!()
}

#[test]
fn test_safe_division() {
    assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
    assert_eq!(safe_divide(-9.0, 3.0), Ok(-3.0));
    assert!(safe_divide(10.0, 0.0).is_err());
}

/// Reads a configuration file (simulated).
/// Returns Ok(content) normally, Err("File not found") for empty input.
///
/// Same shape as `safe_divide`, but returning an owned `String`. Notice
/// you can mix `Ok(String::from("..."))` and `Err("...")` in the same
/// function: the success and error types are independent.
fn read_config_file(filename: &str) -> Result<String, &'static str> {
    todo!()
}

#[test]
fn test_config_reading() {
    assert_eq!(
        read_config_file("app.toml"),
        Ok("config content".to_string())
    );
    assert!(read_config_file("").is_err());
}

/// Validates an email address (basic check).
/// Returns Ok(email) if contains '@', Err(message) otherwise.
///
/// Now the `Ok` value is a borrow of the input. The `&str` in the return
/// type implicitly borrows from `email`, so the compiler infers a lifetime
/// linking input and output via lifetime elision. Chapter 9 makes this
/// explicit; for now, just notice the function compiles even though no
/// lifetimes appear in the signature.
fn validate_email(email: &str) -> Result<&str, &'static str> {
    todo!()
}

#[test]
fn test_email_validation() {
    assert_eq!(validate_email("user@example.com"), Ok("user@example.com"));
    assert!(validate_email("invalid-email").is_err());
}

/// Parses a percentage from a string. Accepts integers in `0..=100`,
/// optionally with a trailing `%` (so `"42"` and `"42%"` both work).
///
/// Returns `Ok(value)` on success, `Err(message)` otherwise.
///
/// This is the hardest function in the chapter; the previous three
/// were warmups. More than one thing can go wrong, and they need
/// different error messages. Strip the optional `%` first, then
/// `parse::<u8>()` the rest, then bounds-check. Each step is its own
/// potential `Err`.
///
/// Note: the error type here is `&'static str`, which means the message
/// has to be a string literal. If you find yourself wanting
/// `format!("{input} is out of range")` in an `Err`, you'd need to
/// change the return type to `Result<u8, String>`. Stick with literals
/// for this exercise.
fn parse_percentage(input: &str) -> Result<u8, &'static str> {
    todo!()
}

#[test]
fn test_percentage_parsing() {
    assert_eq!(parse_percentage("0"), Ok(0));
    assert_eq!(parse_percentage("42"), Ok(42));
    assert_eq!(parse_percentage("100"), Ok(100));
    assert_eq!(parse_percentage("75%"), Ok(75));
    assert!(parse_percentage("101").is_err());
    assert!(parse_percentage("-1").is_err());
    assert!(parse_percentage("half").is_err());
    assert!(parse_percentage("").is_err());
}
