//! # Predicates over multiple fields
//!
//! Methods are a natural place to encode business rules that span
//! several fields. Rather than scattering `user.is_verified && ..`
//! checks across the codebase, name the rule once on the type so the
//! intent is obvious at every call site.
//!
//! `can_access_premium` combines two conditions into a single `bool`.
//! In Rust, the body of a function is an expression, so you can just
//! write the boolean expression with no `return` and no semicolon.

#[derive(Debug, PartialEq)]
struct User {
    email: String,
    name: String,
    is_verified: bool,
    login_count: u32,
}

impl User {
    fn new(email: String, name: String) -> Self {
        Self {
            email,
            name,
            is_verified: false,
            login_count: 0,
        }
    }

    fn record_login(&mut self) {
        self.login_count += 1;
        self.is_verified = true;
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
    fn test_can_access_premium() {
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
