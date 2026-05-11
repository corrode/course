//! # Methods that mutate via `&mut self`
//!
//! When a method needs to change the struct's data, it takes
//! `&mut self`. The caller must hold the value in a `mut` binding for
//! this to compile — mutability is opt-in at every layer.
//!
//! `record_login` bumps a counter and flips a flag. The flag write is
//! idempotent: setting `is_verified = true` on an already-verified
//! user is harmless, which keeps the code simpler than a branch.

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

    /// Records a successful login attempt.
    ///
    /// Increments `login_count` and sets `is_verified` to `true`. The
    /// verification flag is idempotent: setting it on every login is
    /// fine because once you're verified you stay verified. (A real
    /// system would only set it on first login; we keep it simple here.)
    fn record_login(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_login() {
        let mut user = User::new("alice@example.com".to_string(), "Alice".to_string());

        user.record_login();
        assert_eq!(user.login_count, 1);
        assert_eq!(user.is_verified, true);

        user.record_login();
        assert_eq!(user.login_count, 2);
        // Verification stays on across subsequent logins (idempotent).
        assert!(user.is_verified);
    }
}
