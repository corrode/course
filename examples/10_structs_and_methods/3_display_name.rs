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

    /// Returns the user's display name for the UI.
    /// Format: "{name} ({email})"
    fn display_name(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_name() {
        let user = User::new("alice@example.com".to_string(), "Alice".to_string());
        assert_eq!(user.display_name(), "Alice (alice@example.com)");
    }
}
