//! # User Account Management
//!
//! Structs model real-world entities. Methods implement business logic.
//! This pattern is fundamental for any application with user accounts.

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
        unimplemented!()
    }
    
    /// Returns the user's display name for the UI.
    /// Format: "{name} ({email})"
    fn display_name(&self) -> String {
        unimplemented!()
    }
    
    /// Records a successful login attempt.
    /// Increments login count and marks as verified after first login.
    fn record_login(&mut self) {
        unimplemented!()
    }
    
    /// Checks if user can access premium features.
    /// Requires verification and at least 5 logins.
    fn can_access_premium(&self) -> bool {
        unimplemented!()
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