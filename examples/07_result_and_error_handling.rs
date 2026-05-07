//! # Result<T, E> - Error Handling
//!
//! "There are only two hard things in Computer Science: cache invalidation
//! and naming things" - Phil Karlton. Well, we'd like to add a third:
//! error handling! But Rust makes it much easier than most languages.
//!
//! Many languages use exceptions that can pop up anywhere and crash your
//! program unexpectedly. Rust uses Result<T, E> to make errors visible
//! in the type system. You can't ignore an error - you must handle it!
//!
//! This approach was inspired by functional languages like Haskell and ML.
//! Ready to become an error-handling ninja? ⚔️

/// Parses a string into a port number (1-65535).
/// Returns Ok(port) if valid, Err(message) if invalid.
fn parse_port(input: &str) -> Result<u16, &'static str> {
    // First parse to u16, then check if it's in valid range
    todo!()
}

/// Divides two numbers safely.
/// Returns Ok(result) on success, Err(message) on division by zero.
fn safe_divide(a: f64, b: f64) -> Result<f64, &'static str> {
    todo!()
}

/// Reads a configuration file (simulated).
/// Returns Ok(content) normally, Err("File not found") for empty input.
fn read_config_file(filename: &str) -> Result<String, &'static str> {
    // Return error if filename is empty, otherwise return "config content"
    todo!()
}

/// Validates an email address (basic check).
/// Returns Ok(email) if contains '@', Err(message) otherwise.
fn validate_email(email: &str) -> Result<&str, &'static str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_parsing() {
        assert_eq!(parse_port("8080"), Ok(8080));
        assert_eq!(parse_port("80"), Ok(80));
        assert!(parse_port("0").is_err());
        assert!(parse_port("99999").is_err());
        assert!(parse_port("invalid").is_err());
    }

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

    // Bonus exercises - run with: cargo test --example 07_result_and_error_handling --features bonus
    #[cfg(feature = "bonus")]
    mod bonus {
        use super::*;

        /// BONUS: Chain multiple fallible operations
        /// Database systems often chain operations that can fail!
        fn process_user_data(raw_data: &str) -> Result<u32, &'static str> {
            todo!()
        }

        /// BONUS: Implement a custom error type for better error handling
        /// Production systems need structured error reporting!
        #[derive(Debug, PartialEq)]
        enum ConfigError {
            MissingFile,
            InvalidFormat,
            PermissionDenied,
        }

        fn advanced_config_reader(filename: &str) -> Result<String, ConfigError> {
            todo!()
        }

        /// BONUS: Recover from errors with fallback values
        /// Resilient systems need graceful degradation!
        fn get_setting_with_fallback(key: &str) -> u32 {
            todo!()
        }

        /// BONUS: Transform errors from one type to another
        /// API layers often need to convert internal errors!
        fn transform_error(input: &str) -> Result<i32, String> {
            todo!()
        }

        #[test]
        fn test_processing_chain() {
            // Should parse "123" and double it
            assert_eq!(process_user_data("123"), Ok(246));
            assert!(process_user_data("invalid").is_err());
        }

        #[test]
        fn test_custom_errors() {
            assert_eq!(advanced_config_reader("config.toml"), Ok("config data".to_string()));
            assert_eq!(advanced_config_reader(""), Err(ConfigError::MissingFile));
            assert_eq!(advanced_config_reader("invalid.json"), Err(ConfigError::InvalidFormat));
        }

        #[test]
        fn test_fallback_recovery() {
            // Should return default values when settings don't exist
            assert_eq!(get_setting_with_fallback("port"), 8080);
            assert_eq!(get_setting_with_fallback("timeout"), 30);
        }

        #[test]
        fn test_error_transformation() {
            assert_eq!(transform_error("42"), Ok(42));
            assert_eq!(transform_error("invalid"), Err("Parse error: invalid input".to_string()));
        }
    }
}
