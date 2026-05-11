//! # Defining a struct and a constructor
//!
//! A `struct` groups related fields under one name. Rust has no
//! built-in constructors; the convention is an associated function
//! called `new` that returns `Self`. "Associated" means it lives in the
//! `impl` block but doesn't take `self` — you call it as `User::new(..)`.
//!
//! Here we model a `User` and write the constructor that establishes
//! the sensible starting state: a brand-new account is unverified and
//! has zero logins recorded.

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let user = User::new("alice@example.com".to_string(), "Alice".to_string());
        assert_eq!(user.login_count, 0);
        assert_eq!(user.is_verified, false);
    }
}
