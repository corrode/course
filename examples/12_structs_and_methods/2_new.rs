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
