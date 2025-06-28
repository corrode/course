//! # Safe Error Handling
//!
//! Option<T> handles missing values, Result<T, E> handles operations that can fail.
//! This prevents crashes and forces explicit error handling.

/// Finds a user by email in the database.
/// Returns Some(user) if found, None if not in database.
fn find_user_by_email(users: &[&str], email: &str) -> Option<&str> {
    // Use .iter().find() to search for the email
    unimplemented!()
}

/// Parses a string into a number safely.
/// Returns Ok(number) on success, Err(message) on parse failure.
fn parse_port_number(input: &str) -> Result<u16, &'static str> {
    // Use input.parse() and handle the error with .map_err()
    unimplemented!()
}

/// Gets configuration value with fallback.
/// Returns the value if Some, otherwise returns default.
fn get_config_or_default(config_value: Option<u32>) -> u32 {
    // Use .unwrap_or() with default value 8080
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_user() {
        let users = ["alice@example.com", "bob@test.com", "charlie@domain.org"];
        assert_eq!(find_user_by_email(&users, "bob@test.com"), Some("bob@test.com"));
        
        let no_match = ["alice@example.com", "charlie@domain.org"];
        assert_eq!(find_user_by_email(&no_match, "missing@email.com"), None);
    }
    
    #[test]
    fn test_parse_port() {
        assert_eq!(parse_port_number("8080"), Ok(8080));
        assert!(parse_port_number("invalid").is_err());
    }
    
    #[test]
    fn test_config_fallback() {
        assert_eq!(get_config_or_default(Some(3000)), 3000);
        assert_eq!(get_config_or_default(None), 8080);
    }
}