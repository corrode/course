//! # Result<T, E> - Error Handling
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

/// Divides two numbers safely.
/// Returns Ok(result) on success, Err(message) on division by zero.
///
/// Start here. The simplest way to produce a `Result`: an `if` checks the
/// failure case, the `else` branch returns `Ok(...)`.
fn safe_divide(a: f64, b: f64) -> Result<f64, &'static str> {
    todo!()
}

/// Reads a configuration file (simulated).
/// Returns Ok(content) normally, Err("File not found") for empty input.
///
/// Same shape as `safe_divide`, but returning an owned `String`. Notice
/// you can mix `Ok(String::from("..."))` and `Err("...")` in the same
/// function — the success and error types are independent.
fn read_config_file(filename: &str) -> Result<String, &'static str> {
    todo!()
}

/// Validates an email address (basic check).
/// Returns Ok(email) if contains '@', Err(message) otherwise.
///
/// Now the `Ok` value is a borrow of the input. The lifetime on the
/// input is what lets us hand part of it back to the caller.
fn validate_email(email: &str) -> Result<&str, &'static str> {
    todo!()
}

/// Parses a string into a port number (1-65535).
/// Returns Ok(port) if valid, Err(message) if invalid.
///
/// The trickiest of the four: two things can go wrong. First the input
/// might not be a number at all; then, even if it parses cleanly, the
/// value might be out of range. Both failures need to come out as `Err`.
fn parse_port(input: &str) -> Result<u16, &'static str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_division() {
        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        assert!(safe_divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_config_reading() {
        assert_eq!(
            read_config_file("app.toml"),
            Ok("config content".to_string())
        );
        assert!(read_config_file("").is_err());
    }

    #[test]
    fn test_email_validation() {
        assert_eq!(validate_email("user@example.com"), Ok("user@example.com"));
        assert!(validate_email("invalid-email").is_err());
    }

    #[test]
    fn test_port_parsing() {
        assert_eq!(parse_port("8080"), Ok(8080));
        assert_eq!(parse_port("80"), Ok(80));
        assert!(parse_port("0").is_err());
        assert!(parse_port("99999").is_err());
        assert!(parse_port("invalid").is_err());
    }
}
