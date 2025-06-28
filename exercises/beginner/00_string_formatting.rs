//! # String Formatting
//! 
//! Learn to create and format strings - essential for logging, user interfaces,
//! and API responses. The `format!()` macro works like template literals.

/// Formats a user's display name for the UI.
/// Returns "Welcome, {name}!" for user-facing messages.
fn format_welcome_message(name: &str) -> String {
    unimplemented!()
}

/// Returns the application version string.
/// String literals have type &str and live for the entire program.
fn get_app_version() -> &'static str {
    // Return: "1.0.0"
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_welcome_message() {
        assert_eq!(format_welcome_message("Alice"), "Welcome, Alice!");
        assert_eq!(format_welcome_message("Bob"), "Welcome, Bob!");
    }
    
    #[test] 
    fn test_app_version() {
        assert_eq!(get_app_version(), "1.0.0");
    }
}