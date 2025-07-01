//! # Hello Rust!
//!
//! Welcome to the Rust programming language!
//!
//! This is our first Rust exercise. As is common for new programming languages,
//! we will start with a simple "Hello, World!" program. This tradition goes
//! back all the way to the 1970s with Brian Kernighan's C programming tutorial
//! and we will follow that tradition.
//!
//! Have fun! And now, without further ado, let's get started with Rust!

/// Formats a user's display name for the UI.
/// Returns "Welcome, {name}!" for user-facing messages.
fn format_welcome_message(name: &str) -> String {
    todo!()
}

/// Returns the application version string, e.g. "1.0.0".
/// String literals have type &str and live for the entire program.
fn get_app_version() -> &'static str {
    todo!()
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
