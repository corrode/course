#[derive(Debug, PartialEq)]
struct User {
    email: String,
    name: String,
    is_verified: bool,
    login_count: u32,
}

impl User {
    const fn new(email: String, name: String) -> Self {
        Self {
            email,
            name,
            is_verified: false,
            login_count: 0,
        }
    }

    const fn record_login(&mut self) {
        self.login_count += 1;
        self.is_verified = true;
    }

    /// Checks if user can access premium features.
    /// Requires verification and at least 5 logins.
    fn can_access_premium(&self) -> bool {
        todo!()
    }
}

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
