//! # User Account Management
//!
//! Structs and methods are how we model the real world in code! This concept
//! dates back to Simula in the 1960s, which introduced object-oriented programming.
//! Ole-Johan Dahl and Kristen Nygaard wanted to simulate real-world systems,
//! so they created ways to bundle data with behavior.
//!
//! Rust's structs are like simplified classes - they group related data together
//! and let you define methods that operate on that data. No inheritance complexity,
//! just clean, simple composition. It's object-oriented programming without
//! the sharp edges!
//!
//! Time to build your user management empire! 👥

#[derive(Debug, PartialEq)]
struct User {
    email: String,
    name: String,
    is_verified: bool,
    login_count: u32,
}

impl User {
    /// Creates a new user account.
    /// New users start unverified with 0 logins.
    fn new(email: String, name: String) -> Self {
        todo!()
    }

    /// Returns the user's display name for the UI.
    /// Format: "{name} ({email})"
    fn display_name(&self) -> String {
        todo!()
    }

    /// Records a successful login attempt.
    /// Increments login count and marks as verified after first login.
    fn record_login(&mut self) {
        todo!()
    }

    /// Checks if user can access premium features.
    /// Requires verification and at least 5 logins.
    fn can_access_premium(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user() {
        let user = User::new("alice@example.com".to_string(), "Alice".to_string());
        assert_eq!(user.login_count, 0);
        assert_eq!(user.is_verified, false);
    }

    #[test]
    fn test_display_name() {
        let user = User::new("alice@example.com".to_string(), "Alice".to_string());
        assert_eq!(user.display_name(), "Alice (alice@example.com)");
    }

    #[test]
    fn test_login_tracking() {
        let mut user = User::new("alice@example.com".to_string(), "Alice".to_string());

        user.record_login();
        assert_eq!(user.login_count, 1);
        assert_eq!(user.is_verified, true);

        user.record_login();
        assert_eq!(user.login_count, 2);
    }

    #[test]
    fn test_premium_access() {
        let mut user = User::new("alice@example.com".to_string(), "Alice".to_string());

        // Not enough logins yet
        assert_eq!(user.can_access_premium(), false);

        // Record enough logins
        for _ in 0..5 {
            user.record_login();
        }

        // Now has premium access
        assert_eq!(user.can_access_premium(), true);
    }
}
